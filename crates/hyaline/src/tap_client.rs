//! Talks to the helper's message-only windows inside Explorer.

use crate::bindings::{
    E_FAIL, FindWindowExW, HWND, HWND_MESSAGE, LPARAM, SMTO_ABORTIFHUNG, SendMessageTimeoutW,
    WPARAM,
};
use hyaline_core::tap::{Command, WINDOW_CLASS};
use windows_core::{Error, HSTRING, PCWSTR, Result};

/// Every helper window. Usually one; one per Explorer UI thread that hosts a taskbar.
pub fn find_all() -> Vec<HWND> {
    let class = HSTRING::from(WINDOW_CLASS);
    let mut windows = Vec::new();
    loop {
        // SAFETY: the class name outlives the call; `after` is a window found by this loop.
        let hwnd = unsafe {
            FindWindowExW(
                Some(HWND_MESSAGE),
                windows.last().copied(),
                PCWSTR(class.as_ptr()),
                PCWSTR::null(),
            )
        };
        if hwnd.0.is_null() {
            return windows;
        }
        windows.push(hwnd);
    }
}

/// Sends `command` and waits up to one second for the helper to apply it.
pub fn send(hwnd: HWND, command: Command) -> Result<()> {
    let (msg, wparam) = command.to_message();
    let mut reply = 0usize;
    #[allow(clippy::cast_sign_loss)] // SMTO_ABORTIFHUNG is a positive flag
    // SAFETY: `reply` is a valid out pointer; a stale `hwnd` makes the call fail, not crash.
    let sent = unsafe {
        SendMessageTimeoutW(
            hwnd,
            msg,
            WPARAM(wparam),
            LPARAM(0),
            SMTO_ABORTIFHUNG as u32,
            1000,
            Some(&raw mut reply),
        )
    };
    if sent.0 == 0 {
        return Err(Error::from_thread());
    }
    if reply != 0 {
        return Err(Error::new(
            E_FAIL,
            format!("helper could not apply {command:?}"),
        ));
    }
    Ok(())
}
