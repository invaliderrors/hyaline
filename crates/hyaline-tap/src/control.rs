//! The helper's message-only window. It lives on the taskbar's UI thread, so every
//! [`Command`] it receives can touch XAML safely. It also restores the taskbar when the
//! host process exits or crashes.

use crate::background;
use crate::bindings::{
    CloseHandle, CreateWindowExW, DefWindowProcW, GetModuleHandleW, HANDLE, HINSTANCE, HWND,
    HWND_MESSAGE, INFINITE, LPARAM, LRESULT, OpenProcess, PostMessageW, RegisterClassW,
    RegisterWaitForSingleObject, SYNCHRONIZE, UnregisterWait, WNDCLASSW, WPARAM,
    WT_EXECUTEONLYONCE,
};
use crate::guard::guard;
use hyaline_core::tap::{Command, WINDOW_CLASS};
use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use windows_core::{HSTRING, PCWSTR};

/// A pending wait for the host process to exit.
struct HostWatch {
    wait: HANDLE,
    process: HANDLE,
}

impl Drop for HostWatch {
    fn drop(&mut self) {
        // SAFETY: both handles were returned by the system and are released exactly once.
        unsafe {
            let _ = UnregisterWait(self.wait);
            let _ = CloseHandle(self.process);
        }
    }
}

/// Process id of the current host. Process-wide so taskbar threads created later watch the
/// right process.
static HOST_PID: AtomicU32 = AtomicU32::new(0);

/// Sets the host that windows created later will watch.
pub fn preset_host(host_pid: u32) {
    HOST_PID.store(host_pid, Ordering::Relaxed);
}

thread_local! {
    static WINDOW: Cell<Option<HWND>> = const { Cell::new(None) };
    static HOST: RefCell<Option<HostWatch>> = const { RefCell::new(None) };
}

/// Creates the window on the current (UI) thread once and starts watching the host. If the
/// host cannot be watched (already gone), restores the taskbar: never leave it cleared with
/// nobody able to undo it.
pub fn ensure() {
    if WINDOW.get().is_some() {
        return;
    }
    let class = HSTRING::from(WINDOW_CLASS);
    // SAFETY: null name = the process's own module.
    let module = unsafe { GetModuleHandleW(PCWSTR::null()) };
    let instance = HINSTANCE(module.0);
    let window_class = WNDCLASSW {
        lpfnWndProc: Some(window_proc),
        hInstance: instance,
        lpszClassName: PCWSTR(class.as_ptr()),
        ..Default::default()
    };
    // SAFETY: `window_class` is fully initialized. Returns 0 if another thread already
    // registered the class, which is fine.
    unsafe { RegisterClassW(&raw const window_class) };
    // SAFETY: the class name outlives the call; all other arguments are plain values.
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            PCWSTR(class.as_ptr()),
            PCWSTR::null(),
            0,
            0,
            0,
            0,
            0,
            Some(HWND_MESSAGE),
            None,
            Some(instance),
            None,
        )
    };
    if hwnd.0.is_null() {
        return;
    }
    WINDOW.set(Some(hwnd));
    if !watch_host(hwnd, HOST_PID.load(Ordering::Relaxed)) {
        background::set_cleared(false);
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    guard(LRESULT(1), || match Command::from_message(msg, wparam.0) {
        Some(command) => LRESULT(isize::from(!handle(hwnd, command))),
        // SAFETY: the unmodified message goes to the default procedure.
        None => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    })
}

fn handle(hwnd: HWND, command: Command) -> bool {
    match command {
        Command::Clear => background::set_cleared(true),
        Command::Restore => background::set_cleared(false),
        Command::Attach { host_pid } => {
            preset_host(host_pid);
            let watching = watch_host(hwnd, host_pid);
            if !watching {
                background::set_cleared(false);
            }
            watching
        }
    }
}

fn watch_host(hwnd: HWND, host_pid: u32) -> bool {
    HOST.with_borrow_mut(|host| *host = None);
    #[allow(clippy::cast_sign_loss)] // SYNCHRONIZE is a positive access-right bit
    // SAFETY: plain values; failure returns a null handle.
    let process = unsafe { OpenProcess(SYNCHRONIZE as u32, false, host_pid) };
    if process.0.is_null() {
        return false;
    }
    let mut wait = HANDLE::default();
    #[allow(clippy::cast_sign_loss)] // WT_EXECUTEONLYONCE is a positive flag
    // SAFETY: the window handle travels as an opaque context value and is only used with
    // PostMessageW, which tolerates a window that no longer exists.
    let registered = unsafe {
        RegisterWaitForSingleObject(
            &raw mut wait,
            process,
            Some(on_host_exit),
            Some(hwnd.0.cast_const()),
            INFINITE,
            WT_EXECUTEONLYONCE as u32,
        )
    };
    if !registered.as_bool() {
        // SAFETY: `process` was opened above and is closed once.
        let _ = unsafe { CloseHandle(process) };
        return false;
    }
    HOST.with_borrow_mut(|host| *host = Some(HostWatch { wait, process }));
    true
}

unsafe extern "system" fn on_host_exit(context: *mut c_void, _timed_out: bool) {
    guard((), || {
        let (msg, wparam) = Command::Restore.to_message();
        // SAFETY: posting to a stale window handle fails harmlessly.
        let _ = unsafe { PostMessageW(Some(HWND(context)), msg, WPARAM(wparam), LPARAM(0)) };
    });
}
