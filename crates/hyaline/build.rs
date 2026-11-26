//! Embeds the app icon and generates the Win32 bindings `hyaline.exe` uses into
//! `src/bindings.rs`.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hyaline.rc");
    println!("cargo:rerun-if-changed=../../assets/logo.ico");
    embed_resource::compile("hyaline.rc", embed_resource::NONE)
        .manifest_optional()
        .expect("embedding the app icon failed");
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--flat",
        "--filter",
        "CloseHandle",
        "CreateMutexW",
        "CreateWindowExW",
        "DefWindowProcW",
        "DispatchMessageW",
        "E_FAIL",
        "ERROR_ALREADY_EXISTS",
        "ERROR_NOT_FOUND",
        "FindWindowExW",
        "FindWindowW",
        "FreeLibrary",
        "GetMessageW",
        "GetLastError",
        "GetModuleHandleW",
        "GetProcAddress",
        "GetWindowThreadProcessId",
        "HWND_MESSAGE",
        "KillTimer",
        "LoadLibraryW",
        "OpenEventW",
        "RegisterClassW",
        "RegisterWindowMessageW",
        "SendMessageTimeoutW",
        "SetTimer",
        "SMTO_ABORTIFHUNG",
        "SYNCHRONIZE",
        "TranslateMessage",
        "WM_TIMER",
    ]);
}
