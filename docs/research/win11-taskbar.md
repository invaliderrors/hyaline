# Research: Windows 11 taskbar internals (Phase 1)

Date: 2026-11-25 · Machine: Windows 11 build 26300 (Insider), one 1920×1080 monitor at 100 %,
dark theme, **"Transparency effects" off**. Toolchain: Rust 1.100.0, `windows` 0.62.2.
Method: throwaway Rust probe + Rust DLL; results verified by screenshot pixel sampling.
The spike code was deleted after this write-up.

## Summary

| Question | Answer |
|---|---|
| Can an outside process change the look (no injection)? | **No.** `SetWindowCompositionAttribute` succeeds but the taskbar's own XAML background covers it completely. |
| Can a Rust DLL be loaded into Explorer via XAML Diagnostics? | **Yes.** `InitializeXamlDiagnosticsEx` returns `S_OK` in ~2.5 ms; Explorer stayed stable through 5 injections. |
| Which elements to change? | `Rectangle` named `BackgroundFill` and `BackgroundStroke` whose grandparent is `Taskbar.TaskbarBackground` named `BackgroundControl`. |
| Does clearing them + accent give real effects? | **Yes.** Clear and tint work with transparency effects on *or off*; blur/acrylic need transparency effects on. |
| Does Explorer undo the effect? | Not observed: survived Start, Search, Task View, a maximized window and a settings broadcast. |
| Start / Search / Task View detection? | All three detectable (details below). |
| Restore without restarting Explorer? | Only if the original brush is saved **in the same session** and set back. `ClearProperty` is wrong. |

## 1. Taskbar structure

- Top-level `Shell_TrayWnd` (secondary monitors: `Shell_SecondaryTrayWnd`, not tested — single monitor).
- Child `Windows.UI.Composition.DesktopWindowContentBridge` titled `DesktopWindowXamlSource`: the
  taskbar is **system XAML** (`Windows.UI.Xaml.dll`) hosted in `explorer.exe`, view in `Taskbar.View.dll`.
  WinUI 2 (`Microsoft.UI.Xaml.dll`) is also loaded for controls such as `ItemsRepeater`.
- XAML path to the background:
  `DesktopWindowXamlSource › RootScrollViewer › ScrollContentPresenter › Border › Taskbar.TaskbarFrame#TaskbarFrame › Grid#RootGrid › Taskbar.TaskbarBackground#BackgroundControl › Grid › Rectangle#BackgroundFill | Rectangle#BackgroundStroke`
- A second `BackgroundFill` exists under `Taskbar.TaskbarBackground#HoverFlyoutBackgroundControl`
  (thumbnail hover flyout). **Must not be touched** — filter on the `BackgroundControl` ancestor.
- Explorer hosts ~20 XAML islands; the tree dump at advise time was ~870 elements.
- Original values: `BackgroundFill.Fill` = `AcrylicBrush`, `BackgroundStroke.Fill` = `SolidColorBrush`.

## 2. Composition effect from outside (`SetWindowCompositionAttribute`, attribute 19)

- Call returns success, but with the XAML background present the pixels are unchanged
  (`#1C1C1C` before and after, transparency effects off). With transparency effects on, the effect is
  only faintly visible through the semi-transparent XAML brush.
- `GetWindowCompositionAttribute` for the accent **fails** → Hyaline cannot read the current accent back;
  it must track what it applied.
- Accent `GradientColor` is ignored unless `AccentFlags = 2`; without it Windows uses the system accent color.

## 3. XAML Diagnostics injection

- `InitializeXamlDiagnosticsEx` exported by `Windows.UI.Xaml.dll`, called from the host process with
  endpoint `VisualDiagConnection1`, Explorer PID, `wszDllXamlDiagnostics = ""`, the DLL path, our CLSID and
  an initialization string (readable in the DLL via `IXamlDiagnostics::GetInitializationData`).
- The DLL exports `DllGetClassObject`; its object implements `IObjectWithSite`; `SetSite` receives a site
  that QIs to `IXamlDiagnostics` and `IVisualTreeService2`.
- `AdviseVisualTreeChange` must be called **from another thread** than `SetSite` (it deadlocks otherwise);
  the callback then receives the full existing tree and later changes, on the UI thread.
- The `windows` crate has **no** bindings for the `xamlom.h` interfaces; they were declared by hand with
  `#[interface]` from the SDK header (`10.0.26100.0/um/xamlom.h`) and worked first time.
- Each injection of a different DLL path loads another copy (5 copies were loaded); the DLL cannot be
  unloaded. Explorer restart unloads everything. → **Inject once per Explorer instance; talk over IPC.**

## 4. Changing the background

- `IVisualTreeService2::GetPropertyIndex(h, "Fill")` → `E_INVALIDARG`. Working route:
  `GetPropertyValuesChain` → entry with `PropertyName == "Fill"` → its `Index` (1106 on this build;
  **never hard-code**).
- Value: `CreateInstance("Windows.UI.Xaml.Media.SolidColorBrush", "Transparent")` → `SetProperty(h, value, idx)`
  → `S_OK`. No XAML WinRT bindings are needed.
- Result (pixel samples, transparency effects off unless noted):

| XAML background | Accent (state, flags, color AABBGGRR) | Result |
|---|---|---|
| default | any | solid `#1C1C1C` |
| cleared | 0 (disabled) | solid black |
| cleared | 2, 0, `00000000` | solid system accent color |
| cleared | **2, 2, `00000000`** | **fully clear — wallpaper visible** (also with transparency on) |
| cleared | **2, 2, `80FF0000`** | **50 % blue tint over wallpaper** |
| cleared | 3 (blur) / 4 (acrylic) | transparency off: solid black · **on: real blur/acrylic** |

## 5. Persistence (effect applied once, never re-applied)

Unchanged after: opening/closing Start, Task View, Search; a maximized window; a `WM_SETTINGCHANGE`
`ImmersiveColorSet` broadcast; toggling transparency effects. Not tested: monitor/DPI change, taskbar
settings change, sleep/resume, multiple monitors — expected to recreate XAML elements, which the
visual-tree callback reports as new `Add` events (re-apply there, event-driven).

## 6. Restore

- `ClearProperty(h, Fill)` → `S_OK` but leaves **no** fill (Explorer sets the brush as a local value
  in code) → taskbar stays black/clear.
- Setting back value handles from an earlier session → `0x80070490` (not found): handles are per session.
- Toggling transparency effects does **not** make Explorer rebuild the brush.
- Only an Explorer restart restored the default in the spike.
- → ~~The DLL must `GetProperty` the original brush before overriding, keep it, and set it back.~~
  **Superseded in Phase 2:** a handle does not keep its object alive, so once the brush is replaced
  the saved handle is dead (`0x80070490`). Hyaline instead sets `Opacity` = 0 and clears it to restore.
  If the host app dies, the DLL must notice (wait on the host process handle) and restore by itself.
  Whether `SetProperty` may be called off the UI thread is **unverified** — Phase 2 must check, or marshal
  through `IXamlDiagnostics::GetDispatcher` (a `Windows.UI.Core.CoreDispatcher`).

## 7. State detection (foreground + `IAppVisibility`)

| State | Signal observed |
|---|---|
| Start open | `IAppVisibility::IsLauncherVisible() == true`. Start is now hosted by `SearchHost.exe` (foreground `Windows.UI.Core.CoreWindow` titled "Search"). |
| Search open | Foreground `SearchHost.exe` `Windows.UI.Core.CoreWindow` **and** launcher not visible. |
| Task View open | Foreground `XamlExplorerHostIslandWindow` titled "Task View" in `explorer.exe` (the title was English on a Spanish-locale system; prefer class + process). |

## 8. Tooling finding

The monolithic `windows` crate stopped at **0.62.2 (Oct 2025)**. Microsoft's current Rust line is
**`windows-bindgen` + `windows-core` 0.100.0 (Sep 2026)**, which generates only the APIs a project uses.
Unverified in the spike: whether `windows-bindgen` 0.100 can also generate the `Windows.UI.Core`
dispatcher types — confirm when adopting it.

## Open items carried into later phases

1. `SetProperty` from a non-UI thread, or dispatcher marshalling (Phase 2).
2. Secondary monitors (`Shell_SecondaryTrayWnd`, multiple `TaskbarFrame`s) — needs a 2-monitor test.
3. Explorer memory/CPU overhead of an active XAML Diagnostics session (Phase 6 budget).
4. Behaviour with transparency effects off for blur/acrylic (Windows draws them solid) — product decision.

## Addendum (Phase 3): the accent and how to undo it

Measured with transparency effects on, default XAML background, pixels at five points:

| Taskbar accent | Result |
|---|---|
| Windows default | `#212121 #202129 #212022 #202226 #20212A` (wallpaper faintly through acrylic) |
| Clear (state 2, flags 2, color 0) | identical to the default |
| `ACCENT_DISABLED` (state 0) | flat `#202020` — **not** the default |
| Red tint (state 2, flags 2, `0xC00000FF`) | `#491313 …` |
| Red tint, then `WM_DWMCOMPOSITIONCHANGED` or `WM_THEMECHANGED` sent to `Shell_TrayWnd` | back to the exact default |
| Red tint, then an `ImmersiveColorSet` broadcast | back to the exact default |
| Red tint, then a plain `WM_SETTINGCHANGE` to `Shell_TrayWnd` | tint stays |

Consequences: Explorer re-applies its own accent on composition/theme changes, so Hyaline
re-applies after them; and the helper restores Windows' exact default by posting
`WM_DWMCOMPOSITIONCHANGED` to its taskbar windows.
