//! Embeds the app icon and generates the Win32 bindings `hyaline.exe` uses into
//! `src/bindings.rs`.

fn main() {
    embed_resources();
    generate_bindings();
}

/// Icon and version information (taken from Cargo.toml).
fn embed_resources() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hyaline.rc");
    println!("cargo:rerun-if-changed=../../assets/logo.ico");
    let version = env!("CARGO_PKG_VERSION");
    let macros = [
        format!("VERSION_MAJOR={}", env!("CARGO_PKG_VERSION_MAJOR")),
        format!("VERSION_MINOR={}", env!("CARGO_PKG_VERSION_MINOR")),
        format!("VERSION_PATCH={}", env!("CARGO_PKG_VERSION_PATCH")),
        format!("VERSION_STR=\"{version}\""),
    ];
    embed_resource::compile("hyaline.rc", &macros)
        .manifest_optional()
        .expect("embedding the app icon failed");
}

fn generate_bindings() {
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--flat",
        "--filter",
        "AppendMenuW",
        "CC_FULLOPEN",
        "CC_RGBINIT",
        "CheckMenuRadioItem",
        "CHOOSECOLORW",
        "ChooseColorW",
        "CloseHandle",
        "CreateMutexW",
        "CreatePopupMenu",
        "CreateWindowExW",
        "DefWindowProcW",
        "DestroyIcon",
        "DestroyMenu",
        "DispatchMessageW",
        "E_FAIL",
        "EnumWindows",
        "ERROR_ALREADY_EXISTS",
        "ERROR_FILE_NOT_FOUND",
        "ERROR_NOT_FOUND",
        "FindWindowExW",
        "FindWindowW",
        "FreeLibrary",
        "GetClassNameW",
        "GetCursorPos",
        "GetLastError",
        "GetMessageW",
        "GetModuleHandleW",
        "GetProcAddress",
        "GetSystemMetrics",
        "GetWindowThreadProcessId",
        "HKEY_CURRENT_USER",
        "HWND_MESSAGE",
        "IMAGE_ICON",
        "KillTimer",
        "LoadImageW",
        "LoadLibraryW",
        "LR_DEFAULTCOLOR",
        "MB_ICONERROR",
        "MB_OK",
        "MessageBoxW",
        "MF_BYCOMMAND",
        "MF_CHECKED",
        "MF_GRAYED",
        "MF_POPUP",
        "MF_SEPARATOR",
        "MF_STRING",
        "NIF_ICON",
        "NIF_INFO",
        "NIF_MESSAGE",
        "NIF_TIP",
        "NIIF_INFO",
        "NIIF_WARNING",
        "NIM_ADD",
        "NIM_DELETE",
        "NIM_MODIFY",
        "NIM_SETVERSION",
        "NIN_KEYSELECT",
        "NIN_SELECT",
        "NOTIFYICON_VERSION_4",
        "NOTIFYICONDATAW",
        "OpenEventW",
        "PostMessageW",
        "PostQuitMessage",
        "REG_SZ",
        "RegDeleteKeyValueW",
        "RegGetValueW",
        "RegisterClassW",
        "RegisterWindowMessageW",
        "RegSetKeyValueW",
        "RRF_RT_REG_DWORD",
        "RRF_RT_REG_SZ",
        "SendMessageTimeoutW",
        "SetForegroundWindow",
        "SetTimer",
        "Shell_NotifyIconW",
        "SM_CXSMICON",
        "SM_CYSMICON",
        "SMTO_ABORTIFHUNG",
        "SYNCHRONIZE",
        "TPM_RETURNCMD",
        "TPM_RIGHTBUTTON",
        "TrackPopupMenu",
        "TranslateMessage",
        "WM_APP",
        "WM_CONTEXTMENU",
        "WM_DISPLAYCHANGE",
        "WM_DWMCOMPOSITIONCHANGED",
        "WM_LBUTTONUP",
        "WM_NULL",
        "WM_RBUTTONUP",
        "WM_SETTINGCHANGE",
        "WM_THEMECHANGED",
        "WM_TIMER",
    ]);
}
