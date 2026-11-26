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
| T2 | Hover a taskbar button to show thumbnails | Flyout keeps its background | pass |
| T3 | `taskkill /im hyaline.exe /f` | Default taskbar (`#1C1C1C`) back within 1 s; Explorer PID unchanged | pass (failed first: fixed by hiding via `Opacity`) |
| T4 | Run `hyaline.exe` again | Background cleared again; exactly one `hyaline_tap.dll` in Explorer | pass |
| T5 | With Hyaline running, restart Explorer | New Explorer PID; helper loaded again; background cleared within 2 s | pass |
| T6 | Repeat T3 → T4 five times | Explorer never restarts; still one `hyaline_tap.dll` | pass |
