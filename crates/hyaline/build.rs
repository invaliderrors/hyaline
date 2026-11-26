//! Generates the Win32 bindings `hyaline.exe` uses into `src/bindings.rs`.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--flat",
        "--filter",
        "CreateWindowExW",
        "DefWindowProcW",
        "DispatchMessageW",
        "E_FAIL",
        "FindWindowExW",
        "FindWindowW",
        "FreeLibrary",
        "GetMessageW",
        "GetModuleHandleW",
        "GetProcAddress",
        "GetWindowThreadProcessId",
        "HWND_MESSAGE",
        "LoadLibraryW",
        "RegisterClassW",
        "RegisterWindowMessageW",
        "SendMessageTimeoutW",
        "SMTO_ABORTIFHUNG",
        "TranslateMessage",
    ]);
}
