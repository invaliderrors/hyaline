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
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetModuleHandleW, HINSTANCE,
    HWND, LPARAM, LRESULT, MSG, RegisterClassW, RegisterWindowMessageW, TranslateMessage,
    WNDCLASSW, WPARAM,
};
use hyaline_core::tap::{Command, InitData};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};
use windows_core::{PCWSTR, w};

static TAP_DLL: OnceLock<PathBuf> = OnceLock::new();
static TASKBAR_CREATED: AtomicU32 = AtomicU32::new(0);

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
    let dll = std::env::current_exe()?.with_file_name("hyaline_tap.dll");
    if !dll.is_file() {
        return Err(format!("{} not found", dll.display()).into());
    }
    ensure_tap(&dll)?;
    let _ = TAP_DLL.set(dll);
    // SAFETY: constant, NUL-terminated message name.
    let taskbar_created = unsafe { RegisterWindowMessageW(w!("TaskbarCreated")) };
    TASKBAR_CREATED.store(taskbar_created, Ordering::Relaxed);
    create_listener_window()?;
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
/// clearing the background. Loads it only if no helper window exists yet.
fn ensure_tap(dll: &Path) -> Result<(), Box<dyn Error>> {
    let host_pid = std::process::id();
    let windows = tap_client::find_all();
    if windows.is_empty() {
        let explorer = explorer::taskbar_process_id().ok_or("taskbar not found")?;
        inject::inject(
            dll,
            explorer,
            InitData {
                host_pid,
                cleared: true,
            },
        )?;
        return Ok(());
    }
    for hwnd in windows {
        tap_client::send(hwnd, Command::Attach { host_pid })?;
        tap_client::send(hwnd, Command::Clear)?;
    }
    Ok(())
}

/// A hidden top-level window: `TaskbarCreated` is broadcast only to top-level windows.
fn create_listener_window() -> Result<(), Box<dyn Error>> {
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
    Ok(())
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == TASKBAR_CREATED.load(Ordering::Relaxed) {
        if let Some(dll) = TAP_DLL.get()
            && let Err(error) = ensure_tap(dll)
        {
            eprintln!("hyaline: reloading the helper after an Explorer restart failed: {error}");
        }
        return LRESULT(0);
    }
    // SAFETY: the unmodified message goes to the default procedure.
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
