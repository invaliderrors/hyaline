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
