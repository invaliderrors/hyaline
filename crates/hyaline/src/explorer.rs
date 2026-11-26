//! Finds the Explorer process that owns the taskbar.

use crate::bindings::{
    CloseHandle, FindWindowW, GetWindowThreadProcessId, OpenEventW, SYNCHRONIZE,
};
use hyaline_core::tap::marker_name;
use windows_core::{HSTRING, PCWSTR, w};

/// Process id of the Explorer instance hosting the primary taskbar, if it is running.
pub fn taskbar_process_id() -> Option<u32> {
    // SAFETY: constant, NUL-terminated class name; no window name.
    let taskbar = unsafe { FindWindowW(w!("Shell_TrayWnd"), PCWSTR::null()) };
    if taskbar.0.is_null() {
        return None;
    }
    let mut pid = 0;
    // SAFETY: `pid` is a valid out pointer.
    unsafe { GetWindowThreadProcessId(taskbar, Some(&raw mut pid)) };
    (pid != 0).then_some(pid)
}

/// True if the helper is already loaded into the Explorer process `explorer_pid`.
pub fn has_helper(explorer_pid: u32) -> bool {
    let name = HSTRING::from(marker_name(explorer_pid));
    #[allow(clippy::cast_sign_loss)] // SYNCHRONIZE is a positive access-right bit
    // SAFETY: NUL-terminated name alive for the call; failure returns a null handle.
    let marker = unsafe { OpenEventW(SYNCHRONIZE as u32, false, PCWSTR(name.as_ptr())) };
    if marker.0.is_null() {
        return false;
    }
    // SAFETY: opened above, closed once.
    let _ = unsafe { CloseHandle(marker) };
    true
}
