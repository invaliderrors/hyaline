# Phase 5 — Dynamic States Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Each taskbar automatically shows the look for the highest-priority situation on its monitor — Start, Search, Task View, Maximized window, Visible window, Desktop — each configurable in the tray menu, with zero CPU while nothing changes.

**Architecture:** Pure rules in `hyaline-core` (`states`: per-situation looks + priority resolution; `windows`: which windows count; `config` v2 with v1 migration), unit-tested. `hyaline.exe` gains `watcher.rs` (out-of-context `SetWinEventHook`s + an `IAppVisibilityEvents` sink, all coalesced into one posted "recompute" message) and `observe.rs` (per-monitor observation from window attributes), and applies the resolved look per taskbar. The helper protocol is unchanged.

**Tech Stack:** Rust 1.100.0, `windows-core`/`windows-bindgen` 0.100, `serde`/`toml`.

**Spec:** `docs/superpowers/specs/2026-11-25-hyaline-design.md` (§2 dynamic states, §5 state priority and event sources, §8 Phase 5); detection signals in `docs/research/win11-taskbar.md` §7.

## Global Constraints

- Commits: Conventional Commits, granular, **no AI attribution**. Branch `feat/phase-5-dynamic-states`, PR, squash merge.
- Owner-delegated decisions: priority Start → Search → Task View → Maximized window → Visible window → Desktop; Desktop always on; defaults Desktop = Clear 0 %, Maximized window = Normal, others off; v1 config migrates to v2 with its look as Desktop.
- Background hiding stays global: cleared while any taskbar's resolved look is not Normal. A taskbar resolving to Normal while another does not gets Explorer's default accent (documented limitation until Phase 6).
- Detection is event-driven only (no timers while idle). Events are coalesced: at most one pending recompute message.
- Testing uses no simulated keyboard/mouse input (owner preference); Start/Search/Task View are owner checks.

## Review Focus

1. A burst of location-change events (dragging a window) → expect one recompute per message-loop turn, no backlog, no idle CPU afterwards.
2. Maximized window on another virtual desktop (cloaked) → expect it ignored.
3. Minimized / tool / owned / zero-size / shell windows → expect them ignored.
4. Start opened → expect the Start look even though the foreground window class is the Search host.
5. Existing v1 config → expect the same desktop look after upgrading, file rewritten as v2 only on the next change.

---

### Task 1: States and priority (`hyaline-core::states`, TDD)

`pub enum Situation { Start, Search, TaskView, MaximizedWindow, VisibleWindow, Desktop }` (priority order), `pub struct Looks { pub desktop: Appearance, pub visible_window: Option<Appearance>, pub maximized_window: Option<Appearance>, pub start: Option<Appearance>, pub search: Option<Appearance>, pub task_view: Option<Appearance> }` + `Default` (per constraints), `pub struct Observed { start, search, task_view, maximized_window, visible_window: bool }`, `impl Looks { pub fn resolve(&self, observed: Observed) -> Appearance; pub fn get/set(Situation) }`. Tests: each priority pair, disabled state falls through, nothing observed → desktop.

### Task 2: Which windows count (`hyaline-core::windows`, TDD)

`pub struct WindowFacts { visible, cloaked, minimized, maximized, tool_window, owned, zero_size, shell: bool }` and `pub fn counts(facts) -> Option<WindowKind>` (`Maximized` / `Visible` / `None`). Tests for each exclusion.

### Task 3: Config v2 (`hyaline-core::config`, TDD)

`Config { looks: Looks }`; v2 layout `[desktop]`, `[visible_window]`, `[maximized_window]`, `[start]`, `[search]`, `[task_view]`, each `enabled` (not for desktop) + `mode`/`color`/`opacity`. v1 `[appearance]` parses into `desktop` with other defaults. Tests: v2 round trip, v1 migration, defaults text, rejections.

### Task 4: Exe — observe, watch, apply per taskbar

`observe.rs`: per-monitor `Observed` (`MonitorFromWindow`, `IsWindowVisible`, `DwmGetWindowAttribute(DWMWA_CLOAKED)`, `IsIconic`, `IsZoomed`, ex-style, owner, class/process for Search/Task View, launcher visibility). `watcher.rs`: hooks (`EVENT_SYSTEM_FOREGROUND`, `EVENT_SYSTEM_MINIMIZESTART/END`, `EVENT_SYSTEM_MOVESIZEEND`, `EVENT_OBJECT_SHOW/HIDE/DESTROY/LOCATIONCHANGE` (top-level windows only), `EVENT_OBJECT_CLOAKED/UNCLOAKED`) + `IAppVisibilityEvents`; posts `RECOMPUTE` once until handled. `effect.rs`: apply per taskbar window; remember last applied per window. `main.rs`: settings become `Looks`; recompute → resolve per taskbar → apply.

### Task 5: Tray menu per situation

Submenu per situation: On/Off (not Desktop), Appearance, Color…, Opacity. Ids `situation * 100 + item`.

### Task 6: Manual tests and docs

Phase 5 table: maximize/restore/minimize a test window created by the test script (no input), virtual-desktop cloaking via the window's own API if available, idle CPU sample while static, config migration; owner checks for Start, Search, Task View, menu.
