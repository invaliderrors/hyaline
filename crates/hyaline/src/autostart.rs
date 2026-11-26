//! "Start with Windows": a per-user `Run` registry value. The registry is the only place
//! this setting lives.

use crate::bindings::{
    ERROR_FILE_NOT_FOUND, HKEY_CURRENT_USER, REG_SZ, RRF_RT_REG_SZ, RegDeleteKeyValueW,
    RegGetValueW, RegSetKeyValueW,
};
use windows_core::{Error, HRESULT, PCWSTR, Result, w};

const RUN_KEY: PCWSTR = w!(r"Software\Microsoft\Windows\CurrentVersion\Run");
const VALUE: PCWSTR = w!("Hyaline");

/// True if Hyaline is registered to start at sign-in.
pub fn is_enabled() -> bool {
    // SAFETY: constant key and value names; no output buffers requested.
    let status = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            RUN_KEY,
            VALUE,
            RRF_RT_REG_SZ.cast_unsigned(),
            None,
            None,
            None,
        )
    };
    status.0 == 0
}

/// Registers or unregisters Hyaline to start at sign-in.
pub fn set(enabled: bool) -> Result<()> {
    let status = if enabled {
        let exe = std::env::current_exe().map_err(|e| Error::new(HRESULT(-1), e.to_string()))?;
        let command: Vec<u16> = format!("\"{}\"", exe.display())
            .encode_utf16()
            .chain([0])
            .collect();
        let bytes = u32::try_from(command.len() * 2).unwrap_or(u32::MAX);
        // SAFETY: `command` is a NUL-terminated UTF-16 string of `bytes` bytes.
        unsafe {
            RegSetKeyValueW(
                HKEY_CURRENT_USER,
                RUN_KEY,
                VALUE,
                REG_SZ,
                Some(command.as_ptr().cast()),
                bytes,
            )
        }
    } else {
        // SAFETY: constant key and value names.
        let status = unsafe { RegDeleteKeyValueW(HKEY_CURRENT_USER, RUN_KEY, VALUE) };
        if status.0 == ERROR_FILE_NOT_FOUND {
            return Ok(());
        }
        status
    };
    if status.0 == 0 {
        Ok(())
    } else {
        // HRESULT_FROM_WIN32
        Err(HRESULT((0x8007_0000_u32 | status.0.cast_unsigned()).cast_signed()).into())
    }
}
