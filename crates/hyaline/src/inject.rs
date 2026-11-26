//! Loads `hyaline_tap.dll` into Explorer through XAML Diagnostics.

use crate::bindings::{FreeLibrary, GetProcAddress, LoadLibraryW};
use hyaline_core::tap::InitData;
use std::path::Path;
use windows_core::{Error, GUID, HRESULT, HSTRING, PCWSTR, Result, s, w};

/// COM class of the helper; must equal `CLSID_TAP` in `crates/hyaline-tap/src/lib.rs`.
const CLSID_TAP: GUID = GUID::from_u128(0x6a1d_3b7e_9c2f_4e5a_8b10_3d7f_4c2a_9e11);

/// `InitializeXamlDiagnosticsEx` from `um/xamlom.h`.
type InitializeXamlDiagnosticsEx =
    unsafe extern "system" fn(PCWSTR, u32, PCWSTR, PCWSTR, GUID, PCWSTR) -> HRESULT;

/// Asks Explorer's XAML runtime to load `dll` and hand it `init`.
pub fn inject(dll: &Path, explorer_pid: u32, init: InitData) -> Result<()> {
    // SAFETY: constant, NUL-terminated module name.
    let xaml = unsafe { LoadLibraryW(w!("Windows.UI.Xaml.dll")) };
    if xaml.0.is_null() {
        return Err(Error::from_thread());
    }
    let result = (|| {
        // SAFETY: valid module handle and constant export name.
        let export = unsafe { GetProcAddress(xaml, s!("InitializeXamlDiagnosticsEx")) }
            .ok_or_else(Error::from_thread)?;
        // SAFETY: the export has exactly this signature (`um/xamlom.h`).
        let initialize: InitializeXamlDiagnosticsEx = unsafe { std::mem::transmute(export) };
        let dll = HSTRING::from(dll);
        let data = HSTRING::from(init.encode());
        // SAFETY: all strings are NUL-terminated and outlive the call.
        unsafe {
            initialize(
                w!("VisualDiagConnection1"),
                explorer_pid,
                w!(""),
                PCWSTR(dll.as_ptr()),
                CLSID_TAP,
                PCWSTR(data.as_ptr()),
            )
        }
        .ok()
    })();
    // Not needed after the call; keeps our own memory footprint small.
    // SAFETY: `xaml` was loaded above and is released once.
    let _ = unsafe { FreeLibrary(xaml) };
    result
}
