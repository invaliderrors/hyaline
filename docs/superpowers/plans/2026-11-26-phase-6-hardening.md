# Phase 6 — Hardening + Performance Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Hyaline keeps the right look, or steps aside safely, through sleep/resume, display and DPI changes, monitor hot-plug, theme and high-contrast switches, Explorer crash loops and helper upgrades. Every budget in spec §6 is measured and met.

**Architecture:** New pure rules in `hyaline-core` (`tap::RestartGuard` crash-loop backoff, protocol version in `tap`), unit-tested. The exe reacts to more system messages (power resume, high contrast) by re-applying or stepping aside, re-applies when a new taskbar appears, survives one failing taskbar, and checks the helper's protocol version. The helper answers a version query. Release profile shrinks the binaries.

**Tech Stack:** Rust 1.100.0, `windows-core`/`windows-bindgen` 0.100.

**Spec:** `docs/superpowers/specs/2026-11-25-hyaline-design.md` (§6 budgets, §8 Phase 6).

## Global Constraints

- Commits: Conventional Commits, granular, **no AI attribution**. Branch `feat/phase-6-hardening`, PR, squash merge.
- Event-driven only: nothing new may wake the process while idle.
- Never destabilize Explorer: when in doubt, step aside (restore Windows' look) rather than retry forever.
- No simulated keyboard/mouse input in tests; sleep/resume, hot-plug, DPI and high-contrast checks are owner checks.
- Budgets (spec §6): idle 0 CPU; exe private memory < 5 MB; extra memory in Explorer < 2 MB; exe + dll < 1 MB; startup to effect < 150 ms; state switch < 1 frame after the event is delivered.

## Owner-delegated rulings (made while planning)

- **Per-monitor background hiding is deferred past 1.0.** It changes the helper protocol and cannot be verified without a second monitor; the mixed Normal/tinted case stays a documented limitation.
- **High contrast on → Hyaline steps aside** (Windows' look on every taskbar, helper not loaded), and comes back when high contrast turns off.
- **Crash-loop backoff:** 3 Explorer restarts within 2 minutes while the helper was loaded → pause (no helper, Windows' look) and say so once; changing any setting in the menu resumes.
- **Protocol version:** the helper answers a version query; a helper that answers anything else (an older build still pinned in Explorer) is left alone, with one notice asking to restart Explorer or sign out.

## Review Focus

1. Explorer crashes in a loop with the helper loaded → expect at most 3 loads, then a pause with Windows' look, no further injections until a setting changes.
2. Old helper build still loaded in Explorer, new exe started → expect no commands sent to it, one notice, no crash.
3. High contrast turned on while cleared → expect Windows' look within one re-apply; turned off → look back (helper loaded if needed).
4. A taskbar window appears later (monitor plugged in, Explorer slow) → expect it to get the look even if Explorer applies its own accent after ours.
5. One taskbar window fails `SetWindowCompositionAttribute` (destroyed mid-refresh) → expect the other taskbars still updated and the helper still told to clear.

---

### Task 1: Release profile and size budget

`Cargo.toml`: `[profile.release]` with `lto = true`, `codegen-units = 1`, `strip = true`, `opt-level = 3` (keep `panic = "unwind"`: the helper's `guard` relies on `catch_unwind`). Build release, record exe + dll sizes; expect < 1 MB total. Commit `build: optimize release binaries`.

### Task 2: Crash-loop backoff (`hyaline-core::tap::RestartGuard`, TDD)

`pub struct RestartGuard` (`Default`), `pub fn record(&mut self, now_ms: u64) -> bool` (true = paused: this is the 3rd restart within `120_000` ms), `pub fn reset(&mut self)`, `pub fn paused(&self) -> bool`. Tests: 2 restarts → not paused; 3 within 2 min → paused; 3 spread over more than 2 min → not paused; paused stays paused on later records until `reset`; old timestamps age out. Exe: on `TaskbarCreated`, record only if a helper was loaded in the Explorer that died (an `INJECTED` flag set when a helper is found or loaded, cleared on `TaskbarCreated`); while paused `ensure_tap` is not called (`Ensure::Idle`) and one notice is shown; `update` (any menu change) calls `reset`.

### Task 3: Protocol version (`hyaline-core::tap`, TDD; helper; exe)

`pub const PROTOCOL_VERSION: usize = 1;` and `Command::Version` (`WM_APP + 4`, wparam 0). Tests: round trip; `from_message(WM_APP+4, 0)` is `Version`. Helper replies `PROTOCOL_VERSION` to `Version` (other commands keep 0 = success, nonzero = failure). Exe: `tap_client::version(hwnd) -> Option<usize>`; `ensure_tap` returns `Ensure::Stale` when any helper window answers another version → one notice ("Restart Explorer or sign out to finish updating Hyaline."), nothing sent.

### Task 4: Step aside in high contrast; re-apply on resume and new taskbars

- `SystemParametersInfoW(SPI_GETHIGHCONTRAST)` → `HCF_HIGHCONTRASTON`: every resolved look becomes Normal and the helper is not needed. `WM_SETTINGCHANGE` with `wParam == SPI_SETHIGHCONTRAST` → `try_load` (loads the helper when leaving high contrast).
- `WM_POWERBROADCAST` with `PBT_APMRESUMEAUTOMATIC` / `PBT_APMRESUMESUSPEND` → `schedule_reapply`.
- `refresh`: a taskbar window not in `APPLIED` (hot-plugged monitor, recreated taskbar) also schedules the re-apply timers.

### Task 5: One failing taskbar no longer aborts a refresh; startup notice survives

- `apply_accents`: a failing `apply_to` is logged and that window is left out of `APPLIED` (retried next refresh); the others are still applied; the error is returned after the loop. `refresh` still sends Clear/Restore to every helper window, collecting the first error.
- `tray::add` failing at startup (taskbar not up) keeps the settings notice and shows it after the icon is added on `TaskbarCreated`.

### Task 6: Measure budgets, manual checks, docs

Measure on this machine: sizes (Task 1), idle CPU + private memory over 30 s, Explorer private-bytes delta for loading the helper (fresh Explorer), startup to effect with the helper loaded (process start → first accent, temporary debug timing, removed after), refresh duration for one state switch (temporary debug timing, removed after). Phase 6 table in `docs/testing.md` (T34+) including owner checks: sleep/resume, display resolution / DPI scale change, high contrast on/off, monitor plug/unplug if available. Update spec limitations, `docs/PLAN.md`, README status, CHANGELOG.
