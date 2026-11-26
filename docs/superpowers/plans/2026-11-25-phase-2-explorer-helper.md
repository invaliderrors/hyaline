# Phase 2 — Explorer Helper Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `hyaline.exe` loads `hyaline_tap.dll` into Explorer once; the DLL clears the taskbar's XAML background and restores the original brush on request, when the host exits, and when the host crashes — with no Explorer restart.

**Architecture:** The DLL hooks the XAML visual tree through XAML Diagnostics. When it first sees the target background elements (on Explorer's UI thread) it creates a message-only window `HyalineTapWindow` on that same thread; all commands arrive there, so every XAML change runs on the UI thread. The exe finds that window to talk to an already-loaded DLL, and injects only when it is missing. Protocol types and decision logic live in `hyaline-core` and are unit-tested.

**Tech Stack:** Rust 1.100.0 (edition 2024), `windows-core` 0.100 + `windows-bindgen` 0.100 (build.rs-generated bindings), XAML Diagnostics (`xamlom.h`, hand-declared).

**Spec:** `docs/superpowers/specs/2026-11-25-hyaline-design.md` (§5 incl. "Binding rules from Phase 1", §8 Phase 2) and evidence in `docs/research/win11-taskbar.md`.

## Global Constraints

- Commits: Conventional Commits, granular, **no AI attribution**. Branch `feat/phase-2-explorer-helper`, PR, squash merge.
- Bindings: `windows-core = "0.100.0"`, build-dependency `windows-bindgen = "0.100.0"`, flags `--out src/bindings.rs --flat --implement --filter …`. No `windows` crate. Generated `src/bindings.rs` is committed and declared as `#[rustfmt::skip] #[allow(clippy::all, clippy::pedantic, nonstandard_style, dead_code)] mod bindings;`.
- Add to `[workspace.lints.clippy]`: `undocumented_unsafe_blocks = "warn"`. Every `unsafe` block has a `// SAFETY:` comment.
- **Nothing may panic into Explorer.** Since Rust 1.81 a panic crossing `extern "system"` aborts the process — Explorer. Every exported function, COM method body, window procedure and wait callback in `hyaline-tap` runs inside `guard(default, || …)` (`std::panic::catch_unwind`). Release profile keeps `panic = "unwind"`.
- Binding rules (spec §5): target only `BackgroundFill`/`BackgroundStroke` whose grandparent is `BackgroundControl`; `Fill` index from `GetPropertyValuesChain`; save the original via `GetProperty` before `SetProperty`; restore by setting it back (never `ClearProperty`); inject once per Explorer.
- `AdviseVisualTreeChange` is called from a new thread, never inside `SetSite`.
- Exe: `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` so debug builds keep a console for `eprintln!` diagnostics.

## Review Focus

1. A panic in any tap entry point (bad pointer, failed lock) aborting Explorer → expect every entry point wrapped in `guard` (Task 2 review + checklist T6).
2. Starting `hyaline.exe` a second time while the DLL is loaded → expect no second injection: still exactly one `hyaline_tap.dll` module in Explorer (checklist T4).
3. `hyaline.exe` killed → expect the default taskbar back within 1 s with the same Explorer PID (checklist T3).
4. Thumbnail hover flyout → expect its background unchanged (checklist T2).
5. Explorer restarted while `hyaline.exe` runs → expect one fresh injection and a cleared taskbar (checklist T5).

---

### Task 1: Tap protocol in `hyaline-core` (TDD)

**Files:**
- Create: `crates/hyaline-core/src/tap.rs`
- Modify: `crates/hyaline-core/src/lib.rs` (add `pub mod tap;`)

**Interfaces:**
- Produces (all in `hyaline_core::tap`):
  - `pub const WINDOW_CLASS: &str = "HyalineTapWindow";`
  - `pub enum Command { Clear, Restore, Attach { host_pid: u32 } }` (`Debug, Clone, Copy, PartialEq, Eq`)
  - `impl Command { pub const fn to_message(self) -> (u32, usize); pub const fn from_message(msg: u32, wparam: usize) -> Option<Command> }` — `WM_APP = 0x8000`; Clear `0x8001`, Restore `0x8002`, Attach `0x8003` with `wparam = host_pid`.
  - `pub struct InitData { pub host_pid: u32, pub cleared: bool }` (`Debug, Clone, Copy, PartialEq, Eq`) with `pub fn encode(&self) -> String` → `"v1;host=<pid>;cleared=<0|1>"` and `pub fn decode(s: &str) -> Option<InitData>`.
  - `pub fn is_taskbar_background(name: &str, grandparent_name: Option<&str>) -> bool`
  - `pub fn fill_property_index<'a>(props: impl IntoIterator<Item = (&'a str, u32)>) -> Option<u32>` — index of the first property named `"Fill"`.

- [ ] **Step 1: Write the failing tests** (in `tap.rs`, `#[cfg(test)] mod tests`)

```rust
#[test] fn commands_round_trip_through_window_messages() {
    for c in [Command::Clear, Command::Restore, Command::Attach { host_pid: 4242 }] {
        let (msg, wparam) = c.to_message();
        assert_eq!(Command::from_message(msg, wparam), Some(c));
    }
    assert_eq!(Command::Clear.to_message(), (0x8001, 0));
    assert_eq!(Command::Attach { host_pid: 7 }.to_message(), (0x8003, 7));
}
#[test] fn unknown_message_is_not_a_command() {
    assert_eq!(Command::from_message(0x0010, 0), None); // WM_CLOSE
    assert_eq!(Command::from_message(0x8004, 0), None);
}
#[test] fn attach_with_pid_beyond_u32_is_rejected() {
    assert_eq!(Command::from_message(0x8003, usize::MAX), None);
}
#[test] fn init_data_round_trips() {
    let d = InitData { host_pid: 1234, cleared: true };
    assert_eq!(d.encode(), "v1;host=1234;cleared=1");
    assert_eq!(InitData::decode(&d.encode()), Some(d));
}
#[test] fn init_data_rejects_malformed_input() {
    for bad in ["", "v2;host=1;cleared=1", "v1;host=x;cleared=1", "v1;host=1;cleared=2", "v1;host=1"] {
        assert_eq!(InitData::decode(bad), None, "{bad}");
    }
}
#[test] fn only_main_taskbar_background_is_targeted() {
    assert!(is_taskbar_background("BackgroundFill", Some("BackgroundControl")));
    assert!(is_taskbar_background("BackgroundStroke", Some("BackgroundControl")));
    assert!(!is_taskbar_background("BackgroundFill", Some("HoverFlyoutBackgroundControl")));
    assert!(!is_taskbar_background("BackgroundFill", None));
    assert!(!is_taskbar_background("BackgroundElement", Some("BackgroundControl")));
}
#[test] fn fill_index_is_first_fill_entry() {
    let chain = [("Opacity", 453), ("Fill", 1106), ("Fill", 1106), ("Stroke", 1109)];
    assert_eq!(fill_property_index(chain), Some(1106));
    assert_eq!(fill_property_index([("Opacity", 453)]), None);
}
```

- [ ] **Step 2: Run to verify failure**

Run: `cargo test -p hyaline-core`
Expected: compile errors for missing `Command`, `InitData`, etc.

- [ ] **Step 3: Implement the items in the Interfaces block** with doc comments on each public item.

- [ ] **Step 4: Verify**

Run: `cargo test -p hyaline-core` → 7 passed. Then `cargo clippy --workspace --all-targets -- -D warnings` → clean.

- [ ] **Step 5: Commit** — `feat(core): add explorer tap protocol and target rules`

---

### Task 2: `hyaline_tap.dll`

**Files:**
- Modify: `Cargo.toml` (workspace lint `undocumented_unsafe_blocks`), `crates/hyaline-tap/Cargo.toml`
- Create: `crates/hyaline-tap/build.rs`, `crates/hyaline-tap/src/bindings.rs` (generated)
- Create: `crates/hyaline-tap/src/{guard.rs, xamlom.rs, site.rs, tree.rs, background.rs, control.rs}`
- Modify: `crates/hyaline-tap/src/lib.rs`

**Interfaces:**
- Consumes: everything in `hyaline_core::tap` (Task 1).
- Produces: DLL exports `DllGetClassObject`, `DllCanUnloadNow` (returns `S_FALSE`); `pub const CLSID_TAP: GUID = GUID::from_u128(0x6a1d3b7e_9c2f_4e5a_8b10_3d7f4c2a9e11)` in `lib.rs` (the exe uses the same value). A message-only window of class `WINDOW_CLASS` handling `Command`s; each command's `LRESULT` is `0` on success, `1` on failure.

Module responsibilities (one each):
- `guard.rs`: `pub fn guard<T>(fallback: T, f: impl FnOnce() -> T) -> T` — `catch_unwind(AssertUnwindSafe(f))`, fallback on panic.
- `xamlom.rs`: hand-declared `IVisualTreeServiceCallback(2)`, `IVisualTreeService(2)`, `IXamlDiagnostics` and structs `SourceInfo`, `ParentChildRelation`, `VisualElement`, `PropertyChainValue` exactly as in `um/xamlom.h` (SDK 10.0.26100.0); vtable order must match the header.
- `site.rs`: class factory + `IObjectWithSite`. `SetSite` decodes `InitData` from `GetInitializationData` (invalid → log nothing, return `E_INVALIDARG`), stores the `IVisualTreeService2`, spawns the advise thread.
- `tree.rs`: callback; keeps `handle → (name, parent)` for `Add`, removes on `Remove`; on an `Add` passing `is_taskbar_background`, hands the element to `background` and ensures the control window exists (created here, on this UI thread).
- `background.rs`: per element `{ handle, fill_index, original }`; `clear()` = `GetProperty` original (first time only) then `SetProperty(transparent brush)`; `restore()` = `SetProperty(original)`. A global `cleared: bool` decides whether newly added targets get cleared.
- `control.rs`: registers/creates the message-only window; `Clear` / `Restore` call `background`; `Attach { host_pid }` re-arms the host watch. Host watch = `OpenProcess(SYNCHRONIZE)` + `RegisterWaitForSingleObject(WT_EXECUTEONLYONCE)`, callback `PostMessageW(Restore)`.

- [ ] **Step 1: Bindings** — `build.rs` with bindgen filter: `CreateWindowExW DefWindowProcW RegisterClassW HWND_MESSAGE PostMessageW OpenProcess RegisterWaitForSingleObject UnregisterWait CloseHandle IObjectWithSite IClassFactory CLASS_E_NOAGGREGATION CLASS_E_CLASSNOTAVAILABLE E_INVALIDARG S_FALSE SYNCHRONIZE WT_EXECUTEONLYONCE INFINITE` (add others the compiler asks for; keep the list minimal). Add the lint to the workspace.

Run: `cargo build -p hyaline-tap`
Expected: `src/bindings.rs` generated, builds.
Commit: `build(tap): generate win32 bindings with windows-bindgen`

- [ ] **Step 2: `guard.rs` + `xamlom.rs`** — compile.
Commit: `feat(tap): add panic guard and xaml diagnostics interfaces`

- [ ] **Step 3: `site.rs` + exports in `lib.rs`** — every method body and export inside `guard`.
Commit: `feat(tap): add com class factory and site`

- [ ] **Step 4: `tree.rs` + `background.rs` + `control.rs`.**
Commit: `feat(tap): clear and restore taskbar background on command and host exit`

- [ ] **Step 5: Verify**

Run: `cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo build --workspace --release; cargo test --workspace`
Expected: all exit 0; `target/release/hyaline_tap.dll` exists.

---

### Task 3: Injector in `hyaline.exe`

**Files:**
- Modify: `crates/hyaline/Cargo.toml`, `crates/hyaline/src/main.rs`
- Create: `crates/hyaline/build.rs`, `crates/hyaline/src/bindings.rs` (generated), `crates/hyaline/src/{explorer.rs, inject.rs, tap_client.rs}`

**Interfaces:**
- Consumes: `hyaline_core::tap::{Command, InitData, WINDOW_CLASS}`; `CLSID_TAP` value from Task 2 (duplicate the constant in `inject.rs` with a comment pointing to `hyaline-tap/src/lib.rs`; the exe must not link the DLL crate).
- Produces:
  - `explorer::taskbar_process_id() -> Option<u32>` (`FindWindowW("Shell_TrayWnd")` + `GetWindowThreadProcessId`)
  - `inject::inject(dll: &Path, explorer_pid: u32, init: InitData) -> windows_core::Result<()>` — `LoadLibraryW("Windows.UI.Xaml.dll")`, `GetProcAddress("InitializeXamlDiagnosticsEx")`, endpoint `"VisualDiagConnection1"`, `wszDllXamlDiagnostics = ""`.
  - `tap_client::find() -> Option<HWND>` (`FindWindowExW(HWND_MESSAGE, None, WINDOW_CLASS, None)`), `tap_client::send(hwnd, Command) -> windows_core::Result<()>` (`SendMessageTimeoutW`, `SMTO_ABORTIFHUNG`, 1000 ms; `LRESULT != 0` → error).
  - `main`: `ensure_tap()` = if `find()` → `send(Attach{own pid})` + `send(Clear)`; else `inject(<exe dir>/hyaline_tap.dll, pid, InitData{host_pid: own pid, cleared: true})`. Run once at start, then create a hidden top-level window (broadcasts skip message-only windows) that calls `ensure_tap()` on `RegisterWindowMessageW("TaskbarCreated")`; standard message loop. Errors → `eprintln!` + exit code 1 at start; logged and ignored inside the loop.

- [ ] **Step 1: Bindings** (`build.rs` like Task 2; filter: `FindWindowW FindWindowExW GetWindowThreadProcessId LoadLibraryW GetProcAddress SendMessageTimeoutW SMTO_ABORTIFHUNG HWND_MESSAGE RegisterWindowMessageW CreateWindowExW RegisterClassW DefWindowProcW GetMessageW TranslateMessage DispatchMessageW GetModuleHandleW`).
Commit: `build: generate win32 bindings for hyaline exe`
- [ ] **Step 2: `explorer.rs`, `inject.rs`, `tap_client.rs`.**
Commit: `feat: add explorer injector and tap client`
- [ ] **Step 3: `main.rs` lifecycle** (cfg_attr subsystem, `ensure_tap`, TaskbarCreated window, loop).
Commit: `feat: keep taskbar background cleared across explorer restarts`
- [ ] **Step 4: Verify** — same four commands as Task 2 Step 5 → all exit 0.

---

### Task 4: Manual test checklist and run

**Files:**
- Create: `docs/testing.md`

- [ ] **Step 1: Write `docs/testing.md`** — a "Phase 2 — Explorer helper" table with these cases (setup line: build release, close File Explorer windows, note Explorer PID; Explorer locks the DLL, so restart Explorer before rebuilding `hyaline-tap`):

| # | Action | Expected |
|---|---|---|
| T1 | Run `target\release\hyaline.exe` | Taskbar background gone (solid black with transparency effects off); Explorer PID unchanged |
| T2 | Hover a taskbar button to show thumbnails | Flyout background unchanged |
| T3 | `taskkill /im hyaline.exe /f` | Default taskbar back within 1 s; Explorer PID unchanged |
| T4 | Run `hyaline.exe` again | Background cleared again; exactly one `hyaline_tap.dll` in Explorer's modules |
| T5 | With hyaline running, restart Explorer | New Explorer PID; background cleared again within 2 s |
| T6 | Repeat T3→T4 five times | Explorer never restarts; still one `hyaline_tap.dll` |

Verification of pixels: sample the taskbar at (50, height−20) — default `#1C1C1C`, cleared `#000000` (transparency effects off).

- [ ] **Step 2: Run every case** and record pass/fail with the date in the table.
Expected: all pass. A failure → superpowers:systematic-debugging, fix, re-run the whole table.

- [ ] **Step 3: Commit** — `docs: add manual test checklist for explorer helper`

- [ ] **Step 4: Update `docs/PLAN.md`** (Phase 2 → done) and `CHANGELOG.md` (`### Added` → `- Explorer helper: clears the taskbar background and always restores it.`).
Commit: `docs: mark phase 2 done`
