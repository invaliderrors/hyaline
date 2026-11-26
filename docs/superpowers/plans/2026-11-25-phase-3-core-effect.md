# Phase 3 — Core Effect Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** With Hyaline running, every taskbar is fully clear (wallpaper visible) on Windows 11 with transparency effects on or off; the effect survives theme/composition/display changes and Explorer restarts; when Hyaline exits or crashes, the taskbar returns exactly to Windows' default.

**Architecture:** `hyaline.exe` applies the accent with `SetWindowCompositionAttribute` (undocumented, loaded from `user32.dll`) to every `Shell_TrayWnd` / `Shell_SecondaryTrayWnd`, before asking the helper to hide the XAML background. Explorer re-applies its own accent on `WM_DWMCOMPOSITIONCHANGED` / `WM_THEMECHANGED` / immersive color broadcasts, so the exe re-applies 250 ms after those (one-shot timer). On restore — including after a host crash — the helper posts `WM_DWMCOMPOSITIONCHANGED` to its taskbar windows, which makes Explorer put back its exact default accent.

**Tech Stack:** Rust 1.100.0, `windows-core`/`windows-bindgen` 0.100.

**Spec:** `docs/superpowers/specs/2026-11-25-hyaline-design.md` (§5, §8 Phase 3); evidence `docs/research/win11-taskbar.md` §2, §4 and the Phase 3 addendum written in Task 4.

## Global Constraints

- Commits: Conventional Commits, granular, **no AI attribution**. Branch `feat/phase-3-core-effect`, PR, squash merge.
- Accent for "clear": state `2` (transparent gradient), flags `2` (use the given color), color `0x00000000`. Attribute id `19` (`WCA_ACCENT_POLICY`).
- No polling: re-apply is driven by window messages plus one bounded one-shot timer.
- Apply the accent **before** hiding the XAML background (otherwise a black flash).
- Everything in the helper stays inside `guard` (Phase 2 rule).

## Review Focus

1. Theme switch / transparency toggle while running → expect the taskbar to stay clear (Explorer re-applies its accent first; ours must land after) — checklist T10.
2. Hyaline killed while a non-default accent is on the taskbar → expect Windows' exact default (pixel-identical to the pre-run baseline with transparency on) — checklist T11.
3. `SetWindowCompositionAttribute` missing from `user32.dll` (future Windows) → expect a clear error at startup, no crash.
4. Explorer restart → expect the accent re-applied to the new taskbar windows, not to the dead handles.
5. A second monitor's `Shell_SecondaryTrayWnd` → expect it in the target list (not testable locally; reviewed by code).

---

### Task 1: Accent encoding in `hyaline-core` (TDD)

**Files:** Create `crates/hyaline-core/src/appearance.rs`; modify `crates/hyaline-core/src/lib.rs` (`pub mod appearance;`).

**Interfaces — Produces:**
- `pub enum Appearance { Clear }` (`Debug, Clone, Copy, PartialEq, Eq`)
- `#[repr(C)] pub struct AccentPolicy { pub state: u32, pub flags: u32, pub gradient_color: u32, pub animation_id: u32 }` (`Debug, Clone, Copy, PartialEq, Eq, Default`) — layout of the Win32 `ACCENT_POLICY`.
- `impl Appearance { pub const fn accent(self) -> AccentPolicy }`
- `pub const WCA_ACCENT_POLICY: u32 = 19;`

- [ ] Step 1: failing test `clear_is_transparent_gradient_with_given_color` asserting `Appearance::Clear.accent() == AccentPolicy { state: 2, flags: 2, gradient_color: 0, animation_id: 0 }` and `size_of::<AccentPolicy>() == 16`.
- [ ] Step 2: `cargo test -p hyaline-core` → fails to compile.
- [ ] Step 3: implement.
- [ ] Step 4: `cargo test -p hyaline-core` → all pass; clippy clean.
- [ ] Step 5: commit `feat(core): add appearance and accent policy encoding`.

### Task 2: Helper resets the accent on restore

**Files:** Modify `crates/hyaline-tap/build.rs` (filter: `EnumWindows`, `GetClassNameW`, `GetWindowThreadProcessId`, `GetCurrentProcessId` already present, `WM_DWMCOMPOSITIONCHANGED`), `crates/hyaline-tap/src/control.rs`.

**Interfaces — Produces:** `fn reset_accents()` in `control.rs`: for every top-level window of the current process whose class is `Shell_TrayWnd` or `Shell_SecondaryTrayWnd`, `PostMessageW(hwnd, WM_DWMCOMPOSITIONCHANGED, 0, 0)`. Called after `background::set_cleared(false)` wherever the helper restores (`Command::Restore`, failed host watch). Callback of `EnumWindows` runs inside `guard`.

- [ ] Step 1: implement; Step 2: fmt/clippy/build; Step 3: commit `fix(tap): make explorer reapply its own accent on restore`.

### Task 3: Exe applies and keeps the accent

**Files:** Modify `crates/hyaline/build.rs` (filter: `EnumWindows`, `GetClassNameW`, `WM_DWMCOMPOSITIONCHANGED`, `WM_THEMECHANGED`, `WM_SETTINGCHANGE`, `WM_DISPLAYCHANGE`), create `crates/hyaline/src/effect.rs`, modify `crates/hyaline/src/main.rs`.

**Interfaces — Produces (`effect.rs`):**
- `pub fn taskbars() -> Vec<HWND>` — top-level windows of class `Shell_TrayWnd` / `Shell_SecondaryTrayWnd`.
- `pub fn apply(appearance: Appearance) -> windows_core::Result<()>` — loads `SetWindowCompositionAttribute` once (`OnceLock`), applies `appearance.accent()` with attribute `WCA_ACCENT_POLICY` to every taskbar; error if the export is missing or every call fails.

**`main.rs`:** call `effect::apply(Appearance::Clear)?` before the first `ensure_tap`; after a successful load on `TaskbarCreated`/retry; and on `WM_DWMCOMPOSITIONCHANGED`, `WM_THEMECHANGED`, `WM_SETTINGCHANGE`, `WM_DISPLAYCHANGE` start a 250 ms one-shot `REAPPLY_TIMER` whose `WM_TIMER` kills itself and calls `effect::apply`. Errors inside the loop are logged.

- [ ] Step 1: bindings; Step 2: `effect.rs`; Step 3: `main.rs`; Step 4: fmt/clippy/release build/tests; Step 5: commits `build: add bindings for the accent effect`, `feat: apply a clear accent to every taskbar`, `feat: reapply the accent after theme and display changes`.

### Task 4: Manual tests, docs

**Files:** Modify `docs/testing.md`, `docs/research/win11-taskbar.md` (Phase 3 addendum: restore via `WM_DWMCOMPOSITIONCHANGED`/`WM_THEMECHANGED`; `ACCENT_DISABLED` ≠ default; Explorer re-applies on those messages), spec §5 binding rules, `docs/PLAN.md`, `CHANGELOG.md`, `README.md` status.

Add a "Phase 3 — Core effect" table and run it (re-run Phase 2 T1, T3, T5 too):

| # | Action | Expected |
|---|---|---|
| T9 | Run Hyaline (transparency effects off) | Taskbar fully clear: sampled pixels equal the wallpaper, not `#000000`/`#1C1C1C` |
| T10 | While running, toggle transparency effects on → off | Still clear after each toggle |
| T11 | Transparency on; while running set a red tint on `Shell_TrayWnd` (research script); kill Hyaline | Pixels equal the pre-run baseline (Windows default) |
| T12 | While running, restart Explorer | Clear again once the helper reloads (≤ 20 s) |

- [ ] Run, record, commit `docs: add core effect tests and research addendum`, `docs: mark phase 3 done`.
