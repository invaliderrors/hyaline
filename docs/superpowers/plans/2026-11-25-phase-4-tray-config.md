# Phase 4 — Tray + Config Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A tray icon with a menu to pick the taskbar look (mode, color, opacity), start with Windows, and exit; settings persist in a versioned TOML file; invalid config falls back to defaults with a visible notice.

**Architecture:** Pure settings, accent encoding and TOML parsing live in `hyaline-core` (unit-tested). `hyaline.exe` owns the tray icon (`Shell_NotifyIconW`) on its existing hidden window, builds the popup menu each time it opens, persists settings to `%APPDATA%\Hyaline\config.toml`, and stores "Start with Windows" only in `HKCU\…\Run`. The helper protocol is unchanged: `Normal` = `Restore`, any other mode = `Clear` + accent.

**Tech Stack:** Rust 1.100.0, `windows-core`/`windows-bindgen` 0.100, `serde` 1.0.229, `toml` 1.2.3.

**Spec:** `docs/superpowers/specs/2026-11-25-hyaline-design.md` (§2 goals, §8 Phase 4).

## Global Constraints

- Commits: Conventional Commits, granular, **no AI attribution**. Branch `feat/phase-4-tray-config`, PR, squash merge.
- Owner decisions (2026-11-25, delegated): tray menu only; Hyaline never changes the system "Transparency effects" setting — Blur/Acrylic are labelled "(needs Transparency effects)" while it is off.
- Modes and accents (`AccentFlags = 2`, color `0xAABBGGRR`): Normal = Windows default (no accent; helper `Restore`); Opaque = state 1, alpha 255; Clear = state 2, alpha = opacity; Blur = state 3, alpha = opacity; Acrylic = state 4, alpha = opacity, minimum 1.
- Defaults: mode Clear, color `#000000`, opacity 0 %.
- Config: `version = 1`, `[appearance] mode = "clear" | …`, `color = "#RRGGBB"`, `opacity = 0..=100`. Unknown version, unknown keys or out-of-range values → defaults + notice; the file is only rewritten when the user changes a setting.
- The accent is applied only while a helper window exists (so a host crash can never leave a tint without the helper to reset it).
- No polling; the menu reads the transparency setting when it opens.

## Review Focus

1. Explorer restart → tray icon re-added (`TaskbarCreated`), settings re-applied.
2. Exit → taskbar back to Windows' default and the tray icon removed.
3. Corrupt/hand-edited config → defaults + one notice; no crash; file not overwritten until a change.
4. Start with Windows toggled twice → Run value created then removed; path quoted.
5. Switching Normal → Clear → Normal repeatedly → no leftover accent, helper not re-injected.

---

### Task 1: Settings and accent encoding (`hyaline-core::appearance`, TDD)

Replace `Appearance::Clear` with:
- `pub enum Mode { Normal, Opaque, Clear, Blur, Acrylic }` (serde lowercase), `pub struct Rgb { pub r: u8, pub g: u8, pub b: u8 }` with `parse("#RRGGBB")` / `Display`, `pub struct Appearance { pub mode: Mode, pub color: Rgb, pub opacity: u8 }` + `Default` (Clear, black, 0).
- `impl Appearance { pub fn accent(&self) -> Option<AccentPolicy> }` — `None` for Normal.

Tests: each mode's accent (state, flags 2, color bytes and alpha), opacity 100 % → alpha 255, acrylic alpha ≥ 1, Normal → None, `Rgb` parse valid/invalid/round trip.

### Task 2: Config file format (`hyaline-core::config`, TDD)

`pub struct Config { pub appearance: Appearance }`; `pub fn parse(text: &str) -> Result<Config, String>`; `pub fn to_toml(&self) -> String`. Tests: round trip, defaults text, wrong version, unknown mode, bad color, opacity 101, unknown key → error.

### Task 3: Exe applies settings (mode-aware)

`effect::apply(&Appearance)`; `main`: current settings in a `Mutex<Appearance>` (+ setter that persists), `apply_settings()`: Normal → send `Restore` to helper windows; else `ensure_tap` with `cleared: true` then accent once helper windows exist (re-apply timers after load). `ensure_tap` honours Normal (inject with `cleared: false`, no `Clear`).

### Task 4: Config file IO, tray icon, menu, autostart, exit, notices

New modules: `config_file.rs` (load/save under `%APPDATA%\Hyaline`), `tray.rs` (add/re-add/remove icon, balloon notice, popup menu, command ids), `autostart.rs` (Run key get/set). Fatal startup errors → `MessageBoxW`. `TaskbarCreated` → re-add icon.

### Task 5: Manual tests and docs

`docs/testing.md` "Phase 4" table (T14–T20 mirroring the Review Focus plus each mode visually), README usage section, CHANGELOG, PLAN.
