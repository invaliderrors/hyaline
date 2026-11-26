//! Applies the taskbar accent through `SetWindowCompositionAttribute`, an undocumented
//! `user32.dll` export (not in the Windows metadata, so it is loaded by name).

use crate::bindings::{
    E_FAIL, EnumWindows, GetClassNameW, GetModuleHandleW, GetProcAddress, HWND, LPARAM,
};
use hyaline_core::appearance::{AccentPolicy, Appearance, WCA_ACCENT_POLICY};
use hyaline_core::tap::is_taskbar_window_class;
use std::ffi::c_void;
use std::sync::OnceLock;
use windows_core::{BOOL, Error, PWSTR, Result, s, w};

/// `WINDOWCOMPOSITIONATTRIBDATA`.
#[repr(C)]
struct CompositionData {
    attribute: u32,
    data: *mut c_void,
    size: usize,
}

type SetWindowCompositionAttribute = unsafe extern "system" fn(HWND, *mut CompositionData) -> BOOL;

fn set_window_composition_attribute() -> Option<SetWindowCompositionAttribute> {
    static FUNCTION: OnceLock<Option<SetWindowCompositionAttribute>> = OnceLock::new();
    *FUNCTION.get_or_init(|| {
        // SAFETY: user32.dll is always loaded in a GUI process; constant names.
        let user32 = unsafe { GetModuleHandleW(w!("user32.dll")) };
        if user32.0.is_null() {
            return None;
        }
        // SAFETY: valid module and constant export name.
        let export = unsafe { GetProcAddress(user32, s!("SetWindowCompositionAttribute")) }?;
        // SAFETY: the export has this signature on every Windows version that has it.
        Some(unsafe { std::mem::transmute::<unsafe extern "system" fn() -> isize, SetWindowCompositionAttribute>(
                export,
            ) })
    })
}

/// Every taskbar window: the primary and one per additional monitor.
pub fn taskbars() -> Vec<HWND> {
    unsafe extern "system" fn visit(hwnd: HWND, found: LPARAM) -> BOOL {
        let mut class = [0u16; 64];
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)] // 64 fits in i32
        // SAFETY: the buffer and its length match.
        let len = unsafe { GetClassNameW(hwnd, PWSTR(class.as_mut_ptr()), class.len() as i32) };
        let len = usize::try_from(len).unwrap_or(0);
        if is_taskbar_window_class(&String::from_utf16_lossy(&class[..len])) {
            // SAFETY: `found` is the `Vec` passed below, alive for the whole enumeration.
            unsafe { &mut *(found.0 as *mut Vec<HWND>) }.push(hwnd);
        }
        BOOL(1)
    }
    let mut found: Vec<HWND> = Vec::new();
    // SAFETY: `found` outlives the synchronous enumeration.
    let _ = unsafe { EnumWindows(Some(visit), LPARAM(&raw mut found as isize)) };
    found
}

/// Applies `appearance` to every taskbar; nothing for Windows' own look. No taskbar yet is not
/// an error: Explorer may be starting, and the effect is applied again once it is up.
pub fn apply(appearance: &Appearance) -> Result<()> {
    let Some(accent) = appearance.accent() else {
        return Ok(());
    };
    let set = set_window_composition_attribute()
        .ok_or_else(|| Error::new(E_FAIL, "SetWindowCompositionAttribute is not available"))?;
    let bars = taskbars();
    let mut applied = 0;
    for &hwnd in &bars {
        let mut accent: AccentPolicy = accent;
        let mut data = CompositionData {
            attribute: WCA_ACCENT_POLICY,
            data: (&raw mut accent).cast(),
            size: size_of::<AccentPolicy>(),
        };
        // SAFETY: `data` points to a live, correctly sized accent policy for the call.
        if unsafe { set(hwnd, &raw mut data) }.as_bool() {
            applied += 1;
        }
    }
    if applied == 0 && !bars.is_empty() {
        return Err(Error::from_thread());
    }
    Ok(())
}
