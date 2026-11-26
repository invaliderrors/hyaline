//! Hyaline: a tiny, fast Windows 11 taskbar customizer.
//!
//! Applies the look picked in the tray menu to every taskbar: an accent on the taskbar windows
//! plus the Explorer helper hiding their XAML background. Both are kept in place across theme,
//! display and Explorer restarts; the helper restores Windows' own look when this process exits.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[rustfmt::skip]
#[allow(clippy::all, clippy::pedantic, clippy::undocumented_unsafe_blocks, nonstandard_style, dead_code)]
mod bindings;
mod autostart;
mod config_file;
mod effect;
mod explorer;
mod inject;
mod tap_client;
mod tray;

use bindings::{
    CreateMutexW, CreateWindowExW, DefWindowProcW, DispatchMessageW, ERROR_ALREADY_EXISTS,
    ERROR_NOT_FOUND, GetLastError, GetMessageW, GetModuleHandleW, HINSTANCE, HWND, KillTimer,
    LPARAM, LRESULT, MB_ICONERROR, MB_OK, MSG, MessageBoxW, PostQuitMessage, RegisterClassW,
    RegisterWindowMessageW, SetTimer, TranslateMessage, WM_DISPLAYCHANGE, WM_DWMCOMPOSITIONCHANGED,
    WM_LBUTTONUP, WM_RBUTTONUP, WM_SETTINGCHANGE, WM_THEMECHANGED, WM_TIMER, WNDCLASSW, WPARAM,
};
use hyaline_core::appearance::{Appearance, Mode};
use hyaline_core::config::Config;
use hyaline_core::tap::{Command, INSTANCE_MUTEX, InitData, retry_delay_ms};
use std::cell::Cell;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};
use windows_core::{HRESULT, HSTRING, PCWSTR, w};

static TAP_DLL: OnceLock<PathBuf> = OnceLock::new();
static TASKBAR_CREATED: AtomicU32 = AtomicU32::new(0);
/// Retries of loading the helper since the last success or Explorer start.
static ATTEMPT: AtomicU32 = AtomicU32::new(0);
const RETRY_TIMER: usize = 1;
/// One-shot timers that re-apply the accent after Explorer has re-applied its own: a quick
/// one, and a late one for when Explorer is busy and its own re-apply lands later.
const REAPPLY_TIMERS: [(usize, u32); 2] = [(2, 250), (3, 1500)];

thread_local! {
    /// The look currently picked. Only the UI thread touches it.
    static SETTINGS: Cell<Appearance> = Cell::new(Appearance::default());
}

/// Outcome of [`ensure_tap`].
enum Ensure {
    Done,
    /// Explorer's taskbar or XAML runtime is not up yet; try again shortly.
    NotReady,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            report_fatal(&error.to_string());
            ExitCode::FAILURE
        }
    }
}

/// Shows a fatal error; release builds have no console.
fn report_fatal(text: &str) {
    eprintln!("hyaline: {text}");
    let text = HSTRING::from(text);
    // SAFETY: NUL-terminated strings alive for the modal call.
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(text.as_ptr()),
            w!("Hyaline"),
            (MB_OK | MB_ICONERROR).cast_unsigned(),
        )
    };
}

fn run() -> Result<(), Box<dyn Error>> {
    let mutex_name = HSTRING::from(INSTANCE_MUTEX);
    // SAFETY: default security, NUL-terminated name. The handle lives until the process exits,
    // which is what keeps a second instance out.
    let _instance = unsafe { CreateMutexW(None, false, PCWSTR(mutex_name.as_ptr())) };
    // SAFETY: no arguments.
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS.cast_unsigned() {
        return Ok(());
    }
    let dll = std::env::current_exe()?.with_file_name("hyaline_tap.dll");
    if !dll.is_file() {
        return Err(format!("{} not found", dll.display()).into());
    }
    let _ = TAP_DLL.set(dll);
    // SAFETY: constant, NUL-terminated message name.
    let taskbar_created = unsafe { RegisterWindowMessageW(w!("TaskbarCreated")) };
    TASKBAR_CREATED.store(taskbar_created, Ordering::Relaxed);
    let listener = create_listener_window()?;
    let (config, notice) = config_file::load();
    SETTINGS.set(config.appearance);
    tray::add(listener);
    if let Some(notice) = notice {
        tray::notify(listener, "Hyaline settings", &notice, true);
    }
    apply_settings(listener)?;
    let mut msg = MSG::default();
    // SAFETY: `msg` is a valid out pointer; GetMessageW returns 0 on WM_QUIT and -1 on error.
    while unsafe { GetMessageW(&raw mut msg, None, 0, 0) }.0 > 0 {
        // SAFETY: `msg` was filled by GetMessageW.
        unsafe {
            let _ = TranslateMessage(&raw const msg);
            DispatchMessageW(&raw const msg);
        }
    }
    Ok(())
}

/// Brings the taskbar to the current settings, loading the helper when needed.
fn apply_settings(listener: HWND) -> Result<(), Box<dyn Error>> {
    let Some(dll) = TAP_DLL.get() else {
        return Ok(());
    };
    match ensure_tap(dll, SETTINGS.get())? {
        Ensure::Done => {
            ATTEMPT.store(0, Ordering::Relaxed);
            // The helper's window may appear a moment after loading; re-apply then.
            schedule_reapply(listener);
        }
        Ensure::NotReady => schedule_retry(listener),
    }
    Ok(())
}

/// Makes sure the helper matches `appearance`: attached to this process, and hiding the
/// background unless the mode is Normal. Loads it only when a non-Normal look needs it.
fn ensure_tap(dll: &Path, appearance: Appearance) -> Result<Ensure, Box<dyn Error>> {
    let host_pid = std::process::id();
    let cleared = appearance.mode != Mode::Normal;
    let windows = tap_client::find_all();
    if windows.is_empty() {
        if !cleared {
            return Ok(Ensure::Done);
        }
        let Some(explorer) = explorer::taskbar_process_id() else {
            return Ok(Ensure::NotReady);
        };
        if explorer::has_helper(explorer) {
            // Loaded but its window is not up yet; it starts with this host's state.
            return Ok(Ensure::Done);
        }
        let init = InitData { host_pid, cleared };
        // HRESULT_FROM_WIN32(ERROR_NOT_FOUND): Explorer's XAML runtime is not up yet.
        let xaml_not_ready =
            HRESULT((0x8007_0000_u32 | ERROR_NOT_FOUND.cast_unsigned()).cast_signed());
        return match inject::inject(dll, explorer, init) {
            Ok(()) => Ok(Ensure::Done),
            Err(error) if error.code() == xaml_not_ready => Ok(Ensure::NotReady),
            Err(error) => Err(error.into()),
        };
    }
    if cleared {
        // Accent first: hiding the XAML background before it would flash.
        effect::apply(&appearance)?;
    }
    let command = if cleared {
        Command::Clear
    } else {
        Command::Restore
    };
    for hwnd in windows {
        tap_client::send(hwnd, Command::Attach { host_pid })?;
        tap_client::send(hwnd, command)?;
    }
    Ok(Ensure::Done)
}

/// Re-applies the accent. Only while the helper is attached: if this process dies, the helper
/// is what puts Windows' own accent back.
fn reapply() {
    let appearance = SETTINGS.get();
    if appearance.mode == Mode::Normal || tap_client::find_all().is_empty() {
        return;
    }
    if let Err(error) = effect::apply(&appearance) {
        eprintln!("hyaline: applying the taskbar effect failed: {error}");
    }
}

fn schedule_reapply(listener: HWND) {
    for (id, delay) in REAPPLY_TIMERS {
        // SAFETY: our own window; re-arming an existing timer just restarts it.
        unsafe { SetTimer(Some(listener), id, delay, None) };
    }
}

fn schedule_retry(listener: HWND) {
    let attempt = ATTEMPT.fetch_add(1, Ordering::Relaxed);
    match retry_delay_ms(attempt) {
        // SAFETY: `listener` is our own window; the timer is killed when it fires.
        Some(delay) => unsafe {
            SetTimer(Some(listener), RETRY_TIMER, delay, None);
        },
        None => eprintln!("hyaline: Explorer did not become ready; waiting for its next restart"),
    }
}

fn try_load(listener: HWND) {
    if let Err(error) = apply_settings(listener) {
        eprintln!("hyaline: loading the helper failed: {error}");
    }
}

/// Stores, saves and applies a new look picked in the menu.
fn update(listener: HWND, appearance: Appearance) {
    SETTINGS.set(appearance);
    if let Err(error) = config_file::save(Config { appearance }) {
        tray::notify(
            listener,
            "Hyaline settings",
            &format!("Could not save settings: {error}"),
            true,
        );
    }
    if let Err(error) = apply_settings(listener) {
        tray::notify(
            listener,
            "Hyaline",
            &format!("Could not apply the look: {error}"),
            true,
        );
    }
}

fn on_menu(listener: HWND) {
    let current = SETTINGS.get();
    let Some(choice) = tray::show_menu(listener, &current, autostart::is_enabled()) else {
        return;
    };
    match choice {
        tray::Choice::Mode(mode) => update(listener, Appearance { mode, ..current }),
        tray::Choice::Opacity(opacity) => update(listener, Appearance { opacity, ..current }),
        tray::Choice::Color => {
            if let Some(color) = tray::pick_color(listener, current.color) {
                update(listener, Appearance { color, ..current });
            }
        }
        tray::Choice::Autostart => {
            if let Err(error) = autostart::set(!autostart::is_enabled()) {
                tray::notify(
                    listener,
                    "Hyaline",
                    &format!("Could not change Start with Windows: {error}"),
                    true,
                );
            }
        }
        tray::Choice::Exit => exit(listener),
    }
}

/// Puts Windows' own look back, removes the icon and quits.
fn exit(listener: HWND) {
    for hwnd in tap_client::find_all() {
        if let Err(error) = tap_client::send(hwnd, Command::Restore) {
            eprintln!("hyaline: restoring the taskbar failed: {error}");
        }
    }
    tray::remove(listener);
    // SAFETY: ends this thread's message loop.
    unsafe { PostQuitMessage(0) };
}

/// A hidden top-level window: `TaskbarCreated` is broadcast only to top-level windows. It also
/// owns the tray icon and the timers.
fn create_listener_window() -> Result<HWND, Box<dyn Error>> {
    // SAFETY: null name = this executable's module.
    let instance = HINSTANCE(unsafe { GetModuleHandleW(PCWSTR::null()) }.0);
    let class = w!("HyalineHostWindow");
    let window_class = WNDCLASSW {
        lpfnWndProc: Some(window_proc),
        hInstance: instance,
        lpszClassName: class,
        ..Default::default()
    };
    // SAFETY: `window_class` is fully initialized and its strings are static.
    if unsafe { RegisterClassW(&raw const window_class) }.0 == 0 {
        return Err(windows_core::Error::from_thread().into());
    }
    // SAFETY: registered class and static strings; the window is never shown.
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class,
            w!("Hyaline"),
            0,
            0,
            0,
            0,
            0,
            None,
            None,
            Some(instance),
            None,
        )
    };
    if hwnd.0.is_null() {
        return Err(windows_core::Error::from_thread().into());
    }
    Ok(hwnd)
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == TASKBAR_CREATED.load(Ordering::Relaxed) {
        // Explorer restarted: it dropped our icon and the helper with it.
        ATTEMPT.store(0, Ordering::Relaxed);
        tray::add(hwnd);
        try_load(hwnd);
        return LRESULT(0);
    }
    if msg == tray::MESSAGE {
        // The low word carries the mouse message.
        let event = lparam.0 & 0xFFFF;
        if event == WM_RBUTTONUP as isize || event == WM_LBUTTONUP as isize {
            on_menu(hwnd);
        }
        return LRESULT(0);
    }
    // Generated message constants are i32; real message ids are small positive values.
    match msg.cast_signed() {
        WM_TIMER if wparam.0 == RETRY_TIMER => {
            // SAFETY: our own window and timer id.
            let _ = unsafe { KillTimer(Some(hwnd), RETRY_TIMER) };
            try_load(hwnd);
            return LRESULT(0);
        }
        WM_TIMER if REAPPLY_TIMERS.iter().any(|&(id, _)| id == wparam.0) => {
            // SAFETY: our own window and timer id.
            let _ = unsafe { KillTimer(Some(hwnd), wparam.0) };
            reapply();
            return LRESULT(0);
        }
        // Explorer re-applies its own accent on these; ours must land after it.
        WM_DWMCOMPOSITIONCHANGED | WM_THEMECHANGED | WM_SETTINGCHANGE | WM_DISPLAYCHANGE => {
            schedule_reapply(hwnd);
        }
        _ => {}
    }
    // SAFETY: the unmodified message goes to the default procedure.
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
