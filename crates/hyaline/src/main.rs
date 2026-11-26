//! Hyaline: a tiny, fast Windows 11 taskbar customizer.
//!
//! Phase 2: loads the Explorer helper and keeps the taskbar background cleared, including
//! after Explorer restarts. The helper restores the taskbar when this process exits.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[rustfmt::skip]
#[allow(clippy::all, clippy::pedantic, clippy::undocumented_unsafe_blocks, nonstandard_style, dead_code)]
mod bindings;
mod explorer;
mod inject;
mod tap_client;

use bindings::{
    CreateMutexW, CreateWindowExW, DefWindowProcW, DispatchMessageW, ERROR_ALREADY_EXISTS,
    ERROR_NOT_FOUND, GetLastError, GetMessageW, GetModuleHandleW, HINSTANCE, HWND, KillTimer,
    LPARAM, LRESULT, MSG, RegisterClassW, RegisterWindowMessageW, SetTimer, TranslateMessage,
    WM_TIMER, WNDCLASSW, WPARAM,
};
use hyaline_core::tap::{Command, INSTANCE_MUTEX, InitData, retry_delay_ms};
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
            eprintln!("hyaline: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mutex_name = HSTRING::from(INSTANCE_MUTEX);
    // SAFETY: default security, NUL-terminated name. The handle lives until the process exits,
    // which is what keeps a second instance out.
    let _instance = unsafe { CreateMutexW(None, false, PCWSTR(mutex_name.as_ptr())) };
    #[allow(clippy::cast_sign_loss)] // ERROR_ALREADY_EXISTS is a positive error code
    // SAFETY: no arguments.
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS as u32 {
        eprintln!("hyaline: already running");
        return Ok(());
    }
    let dll = std::env::current_exe()?.with_file_name("hyaline_tap.dll");
    if !dll.is_file() {
        return Err(format!("{} not found", dll.display()).into());
    }
    let dll = TAP_DLL.get_or_init(|| dll);
    // SAFETY: constant, NUL-terminated message name.
    let taskbar_created = unsafe { RegisterWindowMessageW(w!("TaskbarCreated")) };
    TASKBAR_CREATED.store(taskbar_created, Ordering::Relaxed);
    let listener = create_listener_window()?;
    if let Ensure::NotReady = ensure_tap(dll)? {
        schedule_retry(listener);
    }
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

/// Makes sure the helper is loaded in the current Explorer, attached to this process and
/// clearing the background. Loads it only if it is not there yet.
fn ensure_tap(dll: &Path) -> Result<Ensure, Box<dyn Error>> {
    let host_pid = std::process::id();
    let windows = tap_client::find_all();
    if windows.is_empty() {
        let Some(explorer) = explorer::taskbar_process_id() else {
            return Ok(Ensure::NotReady);
        };
        if explorer::has_helper(explorer) {
            // Loaded but its window is not up yet; it starts with this host's state.
            return Ok(Ensure::Done);
        }
        let init = InitData {
            host_pid,
            cleared: true,
        };
        // HRESULT_FROM_WIN32(ERROR_NOT_FOUND): Explorer's XAML runtime is not up yet.
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)] // bit pattern of an HRESULT
        let xaml_not_ready = HRESULT((0x8007_0000 | ERROR_NOT_FOUND as u32) as i32);
        return match inject::inject(dll, explorer, init) {
            Ok(()) => Ok(Ensure::Done),
            Err(error) if error.code() == xaml_not_ready => Ok(Ensure::NotReady),
            Err(error) => Err(error.into()),
        };
    }
    for hwnd in windows {
        tap_client::send(hwnd, Command::Attach { host_pid })?;
        tap_client::send(hwnd, Command::Clear)?;
    }
    Ok(Ensure::Done)
}

/// Tries to load the helper and, while Explorer is not ready, schedules the next try.
fn try_load(listener: HWND) {
    let Some(dll) = TAP_DLL.get() else {
        return;
    };
    match ensure_tap(dll) {
        Ok(Ensure::Done) => ATTEMPT.store(0, Ordering::Relaxed),
        Ok(Ensure::NotReady) => schedule_retry(listener),
        Err(error) => eprintln!("hyaline: loading the helper failed: {error}"),
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

/// A hidden top-level window: `TaskbarCreated` is broadcast only to top-level windows. It
/// also owns the retry timer.
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
        ATTEMPT.store(0, Ordering::Relaxed);
        try_load(hwnd);
        return LRESULT(0);
    }
    #[allow(clippy::cast_sign_loss)] // WM_TIMER is a positive message id
    if msg == WM_TIMER as u32 && wparam.0 == RETRY_TIMER {
        // SAFETY: our own window and timer id.
        let _ = unsafe { KillTimer(Some(hwnd), RETRY_TIMER) };
        try_load(hwnd);
        return LRESULT(0);
    }
    // SAFETY: the unmodified message goes to the default procedure.
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
