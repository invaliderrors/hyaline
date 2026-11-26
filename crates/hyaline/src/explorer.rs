//! Finds the Explorer process that owns the taskbar.

use crate::bindings::{FindWindowW, GetWindowThreadProcessId};
use windows_core::{PCWSTR, w};

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
