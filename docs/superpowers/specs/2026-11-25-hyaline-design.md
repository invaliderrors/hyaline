# Hyaline — Design

Date: 2026-11-25 · Status: **approved** (2026-11-25)

## 1. Intent

A personal, fully owned alternative to TranslucentTB: make the Windows taskbar clear, blurred,
acrylic or tinted, and switch appearance automatically based on what is on screen. It must be
**very light** and **just work**, built on the **latest stable version** of every tool and library,
to the quality bar in `CLAUDE.md`.

*Assumption (not stated by the owner):* the motivation is owning a lean, reliable tool rather than
adding features TranslucentTB lacks. Feature parity with TranslucentTB's core is the v1 target.

## 2. Goals, non-goals, anti-goals

**Goals (v1.0)**
- Appearance modes: normal (Windows default), opaque, clear, blur, acrylic — each with color + opacity.
- Dynamic states, each with its own appearance: Desktop (default), Visible window, Maximized window,
  Start menu open, Search open, Task View open.
- Every taskbar on every monitor, each evaluated independently.
- Tray icon menu for all settings; settings persisted to a config file.
- Start with Windows (per-user, no admin). Single instance.
- Restores the normal taskbar on exit, crash and uninstall.

**Non-goals (v1)**
- Windows 10 (end of support Oct 2025; halves the test matrix).
- A settings window (tray menu only), localization (English only), Microsoft Store.
- Taskbar layout changes (centering, rounding, hiding icons) — different product category.

**Anti-goals — failure even if it "works"**
- Crashing, freezing or visibly slowing Explorer.
- Any CPU use while nothing on screen changes (no polling).
- Flicker when switching states.
- Needing admin rights; any network access or telemetry.

## 3. Constraints

- Target: Windows 11, current release and Insider builds (owner runs build 26300).
- The Windows 11 taskbar is XAML hosted inside `explorer.exe`. Its own opaque XAML background
  covers any composition effect applied to the taskbar window from outside. **Full transparency
  requires running code inside Explorer.** This is the riskiest part of the project and the part
  most likely to break with Windows updates.
- Undocumented APIs are unavoidable (`SetWindowCompositionAttribute`, taskbar XAML element names).
  Each must be isolated, verified on the target build, and fail safe.

## 4. Approaches considered

| | Approach | Trade-off |
|---|---|---|
| **A ✅** | **Rust**: `hyaline.exe` + small `hyaline_tap.dll` loaded into Explorer through the official XAML Diagnostics API | Full effects on Win11. Memory-safe code inside Explorer. One toolchain (cargo). Fewer reference examples than C++. |
| B | Rust, exe only, no code in Explorer | Lightest and safest, but on Win11 the taskbar's own background hides the effect — fails "works perfectly". |
| C | C++/WinRT, same split as A (what TranslucentTB does) | Most prior art, but no memory safety inside Explorer and a heavier build setup. |

**Chosen: A** (owner delegated the technical choice). **Confirmed by Phase 1**
(`docs/research/win11-taskbar.md`): B shows no visible effect on build 26300, and a Rust DLL loads into
Explorer through XAML Diagnostics and clears the background. The C++ fallback is no longer needed.

## 5. Architecture

```
hyaline.exe  (Rust, GUI subsystem, per-user)
 ├─ tray        tray icon + popup menu (Shell_NotifyIcon, TrackPopupMenu)
 ├─ config      TOML in %APPDATA%\Hyaline\config.toml (serde)
 ├─ taskbars    finds Shell_TrayWnd + Shell_SecondaryTrayWnd; re-finds on "TaskbarCreated"
 ├─ watcher     event-driven state detection (SetWinEventHook, IAppVisibility, WM_* broadcasts)
 ├─ rules       pure function: (per-monitor observed state, config) -> appearance   [unit tested]
 ├─ effect      applies appearance via SetWindowCompositionAttribute (isolated unsafe wrapper)
 └─ injector    loads hyaline_tap.dll into Explorer via InitializeXamlDiagnosticsEx; reloads after Explorer restart

hyaline_tap.dll  (Rust cdylib, inside explorer.exe)
 └─ finds the taskbar's XAML background elements, makes them transparent, restores them on unload.
    Every entry point wrapped in catch_unwind; any error = restore + unload. Never panics into Explorer.

hyaline-core  (Rust lib, shared)
 └─ config types, appearance/state types, rules, IPC message definitions — all pure, all tested.
```

**State priority** (highest wins, per monitor): Start → Search → Task View → Maximized window →
Visible window → Desktop. A disabled state falls through to the next.

**Event sources (no polling):**
- Windows on a monitor: `SetWinEventHook` (foreground, show/hide, minimize, location change filtered to
  top-level windows, cloak/uncloak for virtual desktops), coalesced per message-loop turn.
- Start menu: `IAppVisibility` launcher-visibility events.
- Search / Task View: foreground window identity (exact detection verified in Phase 1).
- Explorer restart: registered `TaskbarCreated` message. Display/DPI/theme/power:
  `WM_DISPLAYCHANGE`, `WM_DPICHANGED`, `WM_SETTINGCHANGE`, `WM_POWERBROADCAST`.
- Explorer resetting the effect: re-apply on the events that trigger it (measured in Phase 1).
  Hooking Explorer's own calls is a last resort only. (Phase 1: no resets observed.)

**Binding rules from Phase 1** (details and evidence: `docs/research/win11-taskbar.md`):
- Target only `Rectangle` `BackgroundFill` / `BackgroundStroke` whose grandparent is
  `Taskbar.TaskbarBackground` named `BackgroundControl` — never the hover-flyout copy.
- Find the `Fill` property index via `GetPropertyValuesChain` (never hard-code; `GetPropertyIndex` fails).
- Before overriding, `GetProperty` and keep the original brush; restore by setting it back
  (`ClearProperty` leaves no fill; handles do not survive a session).
- Inject **once** per Explorer instance (the DLL cannot unload); all later changes go over IPC.
  Re-inject only after `TaskbarCreated`.
- If the host app exits or crashes, the DLL restores the original brushes by itself (waits on the host
  process handle).
- Apply the accent with `AccentFlags = 2` so the configured color is used instead of the system accent.
  `GetWindowCompositionAttribute` cannot read the accent back — track applied state.
- Re-apply on visual-tree `Add` events for the target elements (Explorer recreating the taskbar).
- Detection: Start = `IAppVisibility` launcher visible; Search = foreground `SearchHost.exe`
  `Windows.UI.Core.CoreWindow` while launcher not visible; Task View = foreground
  `XamlExplorerHostIslandWindow` in `explorer.exe`.

## 6. Performance budgets (targets, measured in Phase 6)

| Metric | Target |
|---|---|
| Idle CPU (static screen) | 0 wakeups / no measurable CPU |
| `hyaline.exe` private memory | < 5 MB |
| Extra memory inside Explorer | < 2 MB |
| exe + dll size (release) | < 1 MB total |
| Startup to effect applied | < 150 ms |
| State switch latency | < 1 frame after the triggering event is delivered |

## 7. Tooling (latest stable at setup time, then pinned)

- Rust via **rustup** (replaces the standalone 1.93 install), latest stable, **edition 2024**,
  pinned in `rust-toolchain.toml`. Dependencies pinned in `Cargo.lock`.
- Latest Visual Studio Build Tools (MSVC linker) + latest Windows SDK.
- Windows bindings: **`windows-bindgen` + `windows-core` (0.100 line)**, generating only the APIs used.
  The monolithic `windows` crate is frozen at 0.62.2 (Oct 2025) and is not used. The XAML Diagnostics
  interfaces (`xamlom.h`) are not in Windows metadata and are declared by hand with `#[interface]`.
- Other crates: `serde` + `toml`. Anything else needs a reason in its PR.
- `rustfmt` + `clippy` (pedantic, warnings = errors). GitHub Actions on `windows-latest`.

## 8. Phases

Each phase ends **working, tested and releasable**. Nothing in a later phase is started early.

| # | Phase | Delivers | Done when |
|---|---|---|---|
| 0 | **Foundation** | rustup + Build Tools + SDK; cargo workspace (`hyaline`, `hyaline_tap`, `hyaline-core`); fmt/clippy config; CI; LICENSE, CHANGELOG | `cargo build/test/clippy` green locally and in CI on an empty skeleton |
| 1 | **Research spike** (throwaway) | On build 26300, verify: effect via `SetWindowCompositionAttribute` from outside; XAML Diagnostics injection from Rust; taskbar background element names; `IAppVisibility`; Search/Task View detection; how often Explorer resets the effect | Findings written to `docs/research/win11-taskbar.md`; spike code deleted; design updated if anything changed |
| 2 | **Explorer helper** | `hyaline.exe` injects `hyaline_tap.dll` once; DLL clears the taskbar background (saving the original brush) and restores it on request, on host exit and on host crash; re-inject after Explorer restart | Taskbar background toggles off/on with no Explorer restart; killing `hyaline.exe` restores the default taskbar |
| 3 | **Core effect** | exe applies one fixed appearance (clear) to every taskbar via the accent API; re-applies after Explorer restart; restores on exit | Clear taskbar visible on Win11 with transparency effects on and off; rules/appearance types unit tested |
| 4 | **Tray + config** | tray menu for mode/color/opacity; TOML config; single instance; start with Windows | Settings survive restart; invalid config falls back to defaults with a visible notice |
| 5 | **Dynamic states** | All six states, per monitor, priority rules, per-state settings | State machine fully unit tested; manual checklist across 2 monitors passes |
| 6 | **Hardening + performance** | sleep/resume, display/DPI changes, hot-plug, theme, "transparency effects" off, high contrast, Explorer crash-loop backoff; budgets measured | Every budget in §6 met with numbers recorded; `docs/testing.md` checklist complete |
| 7 | **Release 1.0** | Release workflow from git tag, checksums, signing if a certificate is available, winget manifest, README | v1.0.0 published and installable via winget |

## 9. Testing strategy

- `hyaline-core` (config, rules, state priority): unit tests, written first (TDD).
- Win32/Explorer behavior: `docs/testing.md` manual checklist (multi-monitor, Explorer restart,
  sleep, DPI, themes), run before every release.
- Every bug gets a regression test where it can be expressed without a live desktop.

## 10. Left to the builder

- Exact IPC mechanism between exe and DLL (window message, named pipe or COM) — choose the simplest that works after Phase 1.
- Internal module layout within each crate, beyond §5.
- Exact `SetWinEventHook` event set and coalescing strategy, as long as idle CPU stays at zero.
- Config file schema details (keep it human-readable and versioned).

## 11. Owner decisions

- **License:** MIT.
- **GitHub repository:** public (used for CI and releases).
- **Phase order (2026-11-25, after Phase 1):** Explorer helper moved before Core effect, because the
  effect alone has no visible result on Windows 11.
- **Pending:** with "Transparency effects" off, Windows draws blur/acrylic as solid; clear and tint still
  work. Proposed: Hyaline does not change the system setting; blur/acrylic entries show a hint in the
  tray menu when it is off.
