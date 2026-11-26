# Manual tests

Behaviour that needs a live Windows desktop is tested here; everything else has unit tests.
Run the relevant tables before every release and after changing the code they cover.

## Setup

1. `cargo build --workspace --release`
2. Copy `target\release\hyaline.exe` and `target\release\hyaline_tap.dll` into a separate folder
   and run from there. Explorer keeps the DLL it loaded locked until Explorer restarts, which
   would otherwise block rebuilding `hyaline-tap`.
3. Close File Explorer windows (an Explorer restart closes them) and note Explorer's PID:
   `(Get-Process explorer).Id`.
4. Taskbar pixel check: sample the taskbar at a spot where the wallpaper is **not** black.
   Default (transparency effects off) is a flat `#1C1C1C`. "Cleared" means anything else:
   until Phase 3 sets the effect, the area under the hidden background shows black or the
   wallpaper, depending on the window's composition state.
5. Helper loaded check: `(Get-Process explorer).Modules | Where-Object ModuleName -eq hyaline_tap.dll`.

## Phase 2 — Explorer helper

Last run: 2026-11-25, Windows 11 build 26300, one 1920×1080 monitor, transparency effects off.

| # | Action | Expected | Result |
|---|---|---|---|
| T1 | Run `hyaline.exe` | Taskbar background cleared; Explorer PID unchanged | pass |
| T2 | Hover a taskbar button to show thumbnails | Flyout keeps its background | pass before the target type check was added; automated re-run could not open the popup (the change only narrows matching and is unit-tested) |
| T3 | `taskkill /im hyaline.exe /f` | Default taskbar (`#1C1C1C`) back within 1 s; Explorer PID unchanged | pass (failed first: fixed by hiding via `Opacity`) |
| T4 | Run `hyaline.exe` again | Background cleared again; exactly one `hyaline_tap.dll` in Explorer | pass |
| T5 | With Hyaline running, restart Explorer | New Explorer PID; helper loaded again and background cleared within 20 s (Explorer's XAML needs ~11 s) | pass (12 s; failed first: fixed by retrying until Explorer is ready) |
| T6 | Repeat T3 → T4 five times | Explorer never restarts; still one `hyaline_tap.dll` | pass |
| T7 | Fresh Explorer: start `hyaline.exe` and kill it ~30 ms later | Default taskbar; never left cleared | pass |
| T8 | Start `hyaline.exe` while it is already running | Second process exits with code 0; nothing changes | pass |

## Phase 3 — Core effect

Last run: 2026-11-25, same machine. Pixels sampled at x = 50, 300, 550, 1400, 1600 on the
taskbar's row. Phase 2 T3, T5 and T8 were re-run with this build and pass.

| # | Action | Expected | Result |
|---|---|---|---|
| T9 | Run Hyaline (transparency effects off) | Taskbar fully clear: the wallpaper continues behind the icons | pass (screenshot) |
| T10 | While running, toggle transparency effects on, then off | Still clear after each toggle | pass |
| T11 | Transparency on: note the default pixels; run Hyaline; force a red tint on `Shell_TrayWnd`; kill Hyaline | Pixels identical to the noted default | pass (`#212121 #202129 …` before and after) |
| T12 | While running, restart Explorer | Clear again once the helper reloads (≤ 20 s) | pass (13 s) |
| T13 | While running, switch Windows to light mode, then back to dark | Still clear in both | pass |

## Phase 4 — Tray and settings

Last run: 2026-11-25, same machine, transparency effects off. Modes were set by writing
`%APPDATA%\Hyaline\config.toml` and restarting Hyaline (no simulated input).

| # | Action | Expected | Result |
|---|---|---|---|
| T14 | With Hyaline running, restart Explorer | Tray icon back; look re-applied | pass (12 s) |
| T15 | Tray menu → Exit | Windows' default look; icon gone; process exits | pass (owner, 2026-11-25) |
| T16 | Invalid `config.toml` (`mode = "neon"`), start Hyaline | Default look; one notification from "Hyaline" titled "Hyaline settings" explaining the error; file unchanged | pass |
| T17 | Tray menu → Start with Windows, twice | `HKCU\…\Run\Hyaline` = quoted exe path, then removed | pass (owner, 2026-11-25) |
| T18 | Tray menu → Appearance: Normal → Clear → Normal → Clear | Default ↔ clear each time; still one `hyaline_tap.dll` | pass (owner, 2026-11-25) |
| T19 | Config: normal / opaque `#FF0000` / clear `#0000FF` 50 % / clear 0 % / blur / acrylic | Default / solid red / blue over wallpaper / wallpaper / solid tint (effects off) | pass |
| T20 | Change a setting in the menu, restart Hyaline | Same look after restart | pass (owner, 2026-11-25) |
| T21 | Keyboard: Win+B, arrow to the H icon, Enter (or Shift+F10) | Menu opens at the icon | pass (owner, 2026-11-25) |
| T22 | Explorer restarted, then Hyaline started right away (cold load) | Look applied once the helper is up; a kill 40 ms after start leaves the default | pass |

## Phase 5 — Dynamic states

Last run: 2026-11-25, same machine. No simulated input: a test window created by the test
script changes only its own state; situations were configured through `config.toml`.

| # | Action | Expected | Result |
|---|---|---|---|
| T23 | Start with a version 1 `config.toml` | Its look becomes the Desktop look; file unchanged until a setting changes | pass |
| T24 | Maximized = red, Visible = blue, both on; then Maximized off; then both off | Highest-priority situation that is happening wins | pass |
| T25 | Test window: normal → maximized → minimized → maximized → closed (Maximized = red) | Desktop look ↔ red, switching on each change | pass (within 1.5 s each) |
| T26 | Hyaline idle for 30 s | No measurable CPU; private memory < 5 MB | pass (0 ms CPU, 2.1 MB) |
| T27 | Window classification on a real desktop | Minimized, cloaked, tool, owned and shell windows ignored | pass (checked against an independent script) |
| T31 | Hyaline's window DPI awareness | Per-monitor (2) | pass |
| T32 | Debug build logging each recompute; test window changes 5 times | A few coalesced recomputes, none while idle | pass (7 for 5 changes; 1 in 10 s of user activity) |
| T28 | Show desktop (Win+D), then bring windows back | Desktop look, then the window situation's look | pass (owner, 2026-11-25) |
| T29 | Turn on Start menu / Search / Task View looks in the menu; open each; also Alt+Tab with the Task View look on | Their looks while open; previous look after closing; Alt+Tab does not show the Task View look | pass (owner, 2026-11-25) |
| T33 | Default settings, desktop with no maximized window: open the emoji panel (Win+.) or clipboard (Win+V) | Look does not change | pass (owner, 2026-11-25) |
| T30 | Menu: a situation's submenu — On, Appearance, Color…, Opacity | Changes apply and persist | pass (owner, 2026-11-25) |

## Phase 6 — Hardening + performance

Last run: 2026-11-26, same machine. Timings came from a temporary build that logged each
refresh (removed afterwards); memory and CPU from `Get-Process`.

| # | Action | Expected | Result |
|---|---|---|---|
| T34 | Release build sizes | `hyaline.exe` + `hyaline_tap.dll` < 1 MB | pass (474 KB + 169 KB) |
| T35 | Restart Hyaline with the helper already loaded, 3 times | First look applied < 150 ms after start | pass (19–21 ms) |
| T36 | Fresh Explorer, then start Hyaline (helper loaded by this start) | First look applied < 150 ms after start | pass (35 ms; 527 ms before the helper's ready message) |
| T37 | State switch: time of one refresh after an event | < one frame (16.7 ms) | pass (1.3–4 ms typical, 11.5 ms worst of 40) |
| T38 | Hyaline private memory | < 5 MB | pass (2.1–2.4 MB) |
| T39 | Explorer private memory before and after loading the helper (fresh Explorer, 10 s samples) | < 2 MB extra | inconclusive: +0 MB on one run, +2.9 MB on another; Explorer drifts ±1 MB on its own within 30 s. The helper's own data is ~870 tree entries; the rest is Windows' XAML diagnostics |
| T40 | Desktop in use (mouse moving, apps switching), 30 s | Small | 140–310 ms CPU per 30 s (0.5–1 % of one core). 98 % of event callbacks are cursor moves, which Windows reports as location changes; the shell hook has no maximize notification and a per-process hook still receives them (both tried) |
| T41 | Static screen, 30 s | No measurable CPU | not re-run: the desktop was in use. No code runs without an event; Phase 5 measured 0 ms (T26) |
| T42 | Old helper build loaded in Explorer, start this build | No second helper, no commands to the old one, one notice asking to restart Explorer | pass |
| T43 | Sleep, then wake | Look back within 2 s | owner check |
| T44 | Lock (Win+L), then unlock | Look kept | owner check |
| T45 | Change display scale (Settings → System → Display → Scale), then back | Look kept at each scale | owner check |
| T46 | Change display resolution, then back | Look kept | owner check |
| T47 | Turn high contrast on (Settings → Accessibility → Contrast themes), then off | Windows' own taskbar while on; Hyaline's look back after off | owner check |
| T48 | Plug in, then unplug a second monitor (if available) | The new taskbar gets its look; nothing breaks on unplug | owner check (only if a monitor is available) |
| T49 | Optional: with Hyaline running, restart Explorer 3 times within 2 minutes | After the third: Windows' look, one "Hyaline paused" notice; changing a setting brings the look back | owner check (optional) |
