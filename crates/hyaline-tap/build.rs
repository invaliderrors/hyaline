//! Generates the Win32/COM bindings this DLL uses into `src/bindings.rs`.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=exports.def");
    // COM entry points must be PRIVATE exports (LNK4104 otherwise).
    let def =
        std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("exports.def");
    println!("cargo:rustc-cdylib-link-arg=/DEF:{}", def.display());
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
