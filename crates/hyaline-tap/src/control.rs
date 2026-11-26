//! The helper's message-only window. It lives on the taskbar's UI thread, so every
//! [`Command`] it receives can touch XAML safely. It also restores the taskbar when the
//! host process exits or crashes.

use crate::background;
use crate::bindings::{
    CloseHandle, CreateWindowExW, DefWindowProcW, EnumWindows, GetClassNameW, GetCurrentProcessId,
    GetModuleHandleW, GetWindowThreadProcessId, HANDLE, HINSTANCE, HWND, HWND_MESSAGE, INFINITE,
    LPARAM, LRESULT, OpenProcess, PostMessageW, RegisterClassW, RegisterWaitForSingleObject,
    SYNCHRONIZE, UnregisterWait, WM_DWMCOMPOSITIONCHANGED, WNDCLASSW, WPARAM, WT_EXECUTEONLYONCE,
};
use crate::guard::guard;
use hyaline_core::tap::{Command, WINDOW_CLASS, is_taskbar_window_class};
use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use windows_core::{BOOL, HSTRING, PCWSTR, PWSTR};

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
        restore();
    }
}

/// Puts the taskbar back to Windows' default: shows the XAML background and has Explorer
/// re-apply its own accent (it does so on `WM_DWMCOMPOSITIONCHANGED`). Returns false if any
/// background element failed.
fn restore() -> bool {
    let restored = background::set_cleared(false);
    reset_accents();
    restored
}

/// Posts `WM_DWMCOMPOSITIONCHANGED` to every taskbar window of this Explorer process.
fn reset_accents() {
    unsafe extern "system" fn visit(hwnd: HWND, _: LPARAM) -> BOOL {
        guard(BOOL(1), || {
            if is_own_taskbar(hwnd) {
                #[allow(clippy::cast_sign_loss)] // positive message id
                // SAFETY: posting to a window of this process; failure is harmless.
                let _ = unsafe {
                    PostMessageW(
                        Some(hwnd),
                        WM_DWMCOMPOSITIONCHANGED as u32,
                        WPARAM(0),
                        LPARAM(0),
                    )
                };
            }
            BOOL(1)
        })
    }
    // SAFETY: `visit` matches WNDENUMPROC and ignores its context.
    let _ = unsafe { EnumWindows(Some(visit), LPARAM(0)) };
}

fn is_own_taskbar(hwnd: HWND) -> bool {
    let mut pid = 0;
    // SAFETY: `pid` is a valid out pointer.
    unsafe { GetWindowThreadProcessId(hwnd, Some(&raw mut pid)) };
    // SAFETY: no arguments.
    if pid != unsafe { GetCurrentProcessId() } {
        return false;
    }
    let mut class = [0u16; 64];
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)] // 64 fits in i32
    // SAFETY: the buffer and its length match.
    let len = unsafe { GetClassNameW(hwnd, PWSTR(class.as_mut_ptr()), class.len() as i32) };
    let len = usize::try_from(len).unwrap_or(0);
    is_taskbar_window_class(&String::from_utf16_lossy(&class[..len]))
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
        Command::Restore => restore(),
        Command::Attach { host_pid } => {
            preset_host(host_pid);
            let watching = watch_host(hwnd, host_pid);
            if !watching {
                restore();
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
