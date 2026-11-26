//! Generates the Win32/COM bindings this DLL uses into `src/bindings.rs`.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--flat",
        "--implement",
        "--filter",
        "CLASS_E_CLASSNOTAVAILABLE",
        "CLASS_E_NOAGGREGATION",
        "CloseHandle",
        "CoTaskMemFree",
        "CreateWindowExW",
        "DefWindowProcW",
        "E_FAIL",
        "E_INVALIDARG",
        "E_NOTIMPL",
        "E_POINTER",
        "E_UNEXPECTED",
        "GetModuleHandleW",
        "HWND_MESSAGE",
        "IClassFactory",
        "INFINITE",
        "IObjectWithSite",
        "OpenProcess",
        "PostMessageW",
        "SYNCHRONIZE",
        "RegisterClassW",
        "RegisterWaitForSingleObject",
        "S_FALSE",
        "UnregisterWait",
        "WT_EXECUTEONLYONCE",
    ]);
}
