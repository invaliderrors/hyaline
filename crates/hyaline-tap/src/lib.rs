//! Helper DLL loaded into `explorer.exe` that makes the taskbar's XAML background transparent.

#[rustfmt::skip]
#[allow(clippy::all, clippy::pedantic, clippy::undocumented_unsafe_blocks, nonstandard_style, dead_code)]
mod bindings;
mod background;
mod control;
mod guard;
mod site;
mod tree;
mod xamlom;

use bindings::{
    CLASS_E_CLASSNOTAVAILABLE, E_POINTER, E_UNEXPECTED, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
    GET_MODULE_HANDLE_EX_FLAG_PIN, GetModuleHandleExW, HMODULE, IClassFactory, S_FALSE,
};
use std::ffi::c_void;
use std::sync::Once;
use windows_core::{GUID, HRESULT, Interface, PCWSTR};

/// COM class of the helper. `hyaline.exe` passes the same value to `InitializeXamlDiagnosticsEx`.
pub const CLSID_TAP: GUID = GUID::from_u128(0x6a1d_3b7e_9c2f_4e5a_8b10_3d7f_4c2a_9e11);

/// COM entry point: hands out the class factory for [`CLSID_TAP`].
///
/// # Safety
/// Called by COM with pointers valid for the duration of the call.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllGetClassObject(
    clsid: *const GUID,
    iid: *const GUID,
    out: *mut *mut c_void,
) -> HRESULT {
    guard::guard(E_UNEXPECTED, || {
        pin_in_memory();
        if clsid.is_null() || out.is_null() {
            return E_POINTER;
        }
        // SAFETY: checked non-null; COM keeps it valid for the call.
        if unsafe { *clsid } != CLSID_TAP {
            return CLASS_E_CLASSNOTAVAILABLE;
        }
        let factory: IClassFactory = site::Factory.into();
        // SAFETY: `iid` and `out` come from COM and are valid for this call.
        unsafe { factory.query(iid, out) }
    })
}

/// Never unload: Explorer keeps calling this DLL's window procedure and callbacks.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_FALSE
}

/// Keeps this DLL loaded for the rest of Explorer's life. Explorer holds pointers into it
/// (window procedure, wait callback), and the XAML Diagnostics loader could otherwise free it.
fn pin_in_memory() {
    static PIN: Once = Once::new();
    PIN.call_once(|| {
        let mut module = HMODULE::default();
        #[allow(clippy::cast_sign_loss)] // both flags are small positive values
        let flags = (GET_MODULE_HANDLE_EX_FLAG_PIN | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS) as u32;
        // SAFETY: with FROM_ADDRESS the "name" is any address inside this module.
        let _ = unsafe {
            GetModuleHandleExW(
                flags,
                PCWSTR(DllGetClassObject as *const u16),
                &raw mut module,
            )
        };
    });
}
