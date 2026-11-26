#[inline]
pub unsafe fn AppendMenuW<P3>(
    hmenu: HMENU,
    uflags: u32,
    uidnewitem: usize,
    lpnewitem: P3,
) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn AppendMenuW(hmenu : HMENU, uflags : u32, uidnewitem : usize, lpnewitem : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AppendMenuW(hmenu, uflags, uidnewitem, lpnewitem.param().abi()) }
}
#[inline]
pub unsafe fn CheckMenuRadioItem(
    hmenu: HMENU,
    first: u32,
    last: u32,
    check: u32,
    flags: u32,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CheckMenuRadioItem(hmenu : HMENU, first : u32, last : u32, check : u32, flags : u32) -> windows_core::BOOL);
    unsafe { CheckMenuRadioItem(hmenu, first, last, check, flags) }
}
#[inline]
pub unsafe fn ChooseColorW(param0: *mut CHOOSECOLORW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseColorW(param0 : *mut CHOOSECOLORW) -> windows_core::BOOL);
    unsafe { ChooseColorW(param0 as _) }
}
#[inline]
pub unsafe fn CloseHandle(hobject: HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> windows_core::BOOL);
    unsafe { CloseHandle(hobject) }
}
#[inline]
pub unsafe fn CoCreateInstance<P1, T>(
    rclsid: *const windows_core::GUID,
    punkouter: P1,
    dwclscontext: u32,
) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        CoCreateInstance(
            rclsid,
            punkouter.param().abi(),
            dwclscontext,
            &T::IID,
            &mut result__,
        )
        .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoInitializeEx(
    pvreserved: Option<*const core::ffi::c_void>,
    dwcoinit: u32,
) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
    unsafe { CoInitializeEx(pvreserved.unwrap_or(core::mem::zeroed()) as _, dwcoinit) }
}
#[inline]
pub unsafe fn CreateMutexW<P2>(
    lpmutexattributes: Option<*const SECURITY_ATTRIBUTES>,
    binitialowner: bool,
    lpname: P2,
) -> HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexW(lpmutexattributes : *const SECURITY_ATTRIBUTES, binitialowner : windows_core::BOOL, lpname : windows_core::PCWSTR) -> HANDLE);
    unsafe {
        CreateMutexW(
            lpmutexattributes.unwrap_or(core::mem::zeroed()) as _,
            binitialowner.into(),
            lpname.param().abi(),
        )
    }
}
#[inline]
pub unsafe fn CreatePopupMenu() -> HMENU {
    windows_core::link!("user32.dll" "system" fn CreatePopupMenu() -> HMENU);
    unsafe { CreatePopupMenu() }
}
#[inline]
pub unsafe fn CreateWindowExW<P1, P2>(
    dwexstyle: u32,
    lpclassname: P1,
    lpwindowname: P2,
    dwstyle: u32,
    x: i32,
    y: i32,
    nwidth: i32,
    nheight: i32,
    hwndparent: Option<HWND>,
    hmenu: Option<HMENU>,
    hinstance: Option<HINSTANCE>,
    lpparam: Option<*const core::ffi::c_void>,
) -> HWND
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateWindowExW(dwexstyle : u32, lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : HWND, hmenu : HMENU, hinstance : HINSTANCE, lpparam : *const core::ffi::c_void) -> HWND);
    unsafe {
        CreateWindowExW(
            dwexstyle,
            lpclassname.param().abi(),
            lpwindowname.param().abi(),
            dwstyle,
            x,
            y,
            nwidth,
            nheight,
            hwndparent.unwrap_or(core::mem::zeroed()) as _,
            hmenu.unwrap_or(core::mem::zeroed()) as _,
            hinstance.unwrap_or(core::mem::zeroed()) as _,
            lpparam.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn DefWindowProcW(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    windows_core::link!("user32.dll" "system" fn DefWindowProcW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> LRESULT);
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
#[inline]
pub unsafe fn DestroyIcon(hicon: HICON) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyIcon(hicon : HICON) -> windows_core::BOOL);
    unsafe { DestroyIcon(hicon) }
}
#[inline]
pub unsafe fn DestroyMenu(hmenu: HMENU) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyMenu(hmenu : HMENU) -> windows_core::BOOL);
    unsafe { DestroyMenu(hmenu) }
}
#[inline]
pub unsafe fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT {
    windows_core::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> LRESULT);
    unsafe { DispatchMessageW(lpmsg) }
}
#[inline]
pub unsafe fn DwmGetWindowAttribute(
    hwnd: HWND,
    dwattribute: u32,
    pvattribute: *mut core::ffi::c_void,
    cbattribute: u32,
) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmGetWindowAttribute(hwnd : HWND, dwattribute : u32, pvattribute : *mut core::ffi::c_void, cbattribute : u32) -> windows_core::HRESULT);
    unsafe { DwmGetWindowAttribute(hwnd, dwattribute, pvattribute as _, cbattribute) }
}
#[inline]
pub unsafe fn EnumWindows(lpenumfunc: WNDENUMPROC, lparam: LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumWindows(lpenumfunc : WNDENUMPROC, lparam : LPARAM) -> windows_core::BOOL);
    unsafe { EnumWindows(lpenumfunc, lparam) }
}
#[inline]
pub unsafe fn FindWindowExW<P2, P3>(
    hwndparent: Option<HWND>,
    hwndchildafter: Option<HWND>,
    lpszclass: P2,
    lpszwindow: P3,
) -> HWND
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn FindWindowExW(hwndparent : HWND, hwndchildafter : HWND, lpszclass : windows_core::PCWSTR, lpszwindow : windows_core::PCWSTR) -> HWND);
    unsafe {
        FindWindowExW(
            hwndparent.unwrap_or(core::mem::zeroed()) as _,
            hwndchildafter.unwrap_or(core::mem::zeroed()) as _,
            lpszclass.param().abi(),
            lpszwindow.param().abi(),
        )
    }
}
#[inline]
pub unsafe fn FindWindowW<P0, P1>(lpclassname: P0, lpwindowname: P1) -> HWND
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn FindWindowW(lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR) -> HWND);
    unsafe { FindWindowW(lpclassname.param().abi(), lpwindowname.param().abi()) }
}
#[inline]
pub unsafe fn FreeLibrary(hlibmodule: HMODULE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> windows_core::BOOL);
    unsafe { FreeLibrary(hlibmodule) }
}
#[inline]
pub unsafe fn GetAncestor(hwnd: HWND, gaflags: u32) -> HWND {
    windows_core::link!("user32.dll" "system" fn GetAncestor(hwnd : HWND, gaflags : u32) -> HWND);
    unsafe { GetAncestor(hwnd, gaflags) }
}
#[inline]
pub unsafe fn GetClassNameW(hwnd: HWND, lpclassname: windows_core::PWSTR, nmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetClassNameW(hwnd : HWND, lpclassname : windows_core::PWSTR, nmaxcount : i32) -> i32);
    unsafe { GetClassNameW(hwnd, lpclassname, nmaxcount) }
}
#[inline]
pub unsafe fn GetCursorPos(lppoint: *mut POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetCursorPos(lppoint : *mut POINT) -> windows_core::BOOL);
    unsafe { GetCursorPos(lppoint as _) }
}
#[inline]
pub unsafe fn GetForegroundWindow() -> HWND {
    windows_core::link!("user32.dll" "system" fn GetForegroundWindow() -> HWND);
    unsafe { GetForegroundWindow() }
}
#[inline]
pub unsafe fn GetLastError() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetLastError() -> u32);
    unsafe { GetLastError() }
}
#[inline]
pub unsafe fn GetMessageW(
    lpmsg: *mut MSG,
    hwnd: Option<HWND>,
    wmsgfiltermin: u32,
    wmsgfiltermax: u32,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMessageW(lpmsg : *mut MSG, hwnd : HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> windows_core::BOOL);
    unsafe {
        GetMessageW(
            lpmsg as _,
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            wmsgfiltermin,
            wmsgfiltermax,
        )
    }
}
#[inline]
pub unsafe fn GetModuleHandleW<P0>(lpmodulename: P0) -> HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : windows_core::PCWSTR) -> HMODULE);
    unsafe { GetModuleHandleW(lpmodulename.param().abi()) }
}
#[inline]
pub unsafe fn GetMonitorInfoW(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMonitorInfoW(hmonitor : HMONITOR, lpmi : *mut MONITORINFO) -> windows_core::BOOL);
    unsafe { GetMonitorInfoW(hmonitor, lpmi as _) }
}
#[inline]
pub unsafe fn GetProcAddress<P1>(hmodule: HMODULE, lpprocname: P1) -> FARPROC
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : windows_core::PCSTR) -> FARPROC);
    unsafe { GetProcAddress(hmodule, lpprocname.param().abi()) }
}
#[inline]
pub unsafe fn GetShellWindow() -> HWND {
    windows_core::link!("user32.dll" "system" fn GetShellWindow() -> HWND);
    unsafe { GetShellWindow() }
}
#[inline]
pub unsafe fn GetSystemMetrics(nindex: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetSystemMetrics(nindex : i32) -> i32);
    unsafe { GetSystemMetrics(nindex) }
}
#[inline]
pub unsafe fn GetWindow(hwnd: HWND, ucmd: u32) -> HWND {
    windows_core::link!("user32.dll" "system" fn GetWindow(hwnd : HWND, ucmd : u32) -> HWND);
    unsafe { GetWindow(hwnd, ucmd) }
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[inline]
pub unsafe fn GetWindowLongPtrW(hwnd: HWND, nindex: i32) -> isize {
    windows_core::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd : HWND, nindex : i32) -> isize);
    unsafe { GetWindowLongPtrW(hwnd, nindex) }
}
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongW as GetWindowLongPtrW;
#[inline]
pub unsafe fn GetWindowLongW(hwnd: HWND, nindex: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowLongW(hwnd : HWND, nindex : i32) -> i32);
    unsafe { GetWindowLongW(hwnd, nindex) }
}
#[inline]
pub unsafe fn GetWindowRect(hwnd: HWND, lprect: *mut RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetWindowRect(hwnd : HWND, lprect : *mut RECT) -> windows_core::BOOL);
    unsafe { GetWindowRect(hwnd, lprect as _) }
}
#[inline]
pub unsafe fn GetWindowThreadProcessId(hwnd: HWND, lpdwprocessid: Option<*mut u32>) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetWindowThreadProcessId(hwnd : HWND, lpdwprocessid : *mut u32) -> u32);
    unsafe { GetWindowThreadProcessId(hwnd, lpdwprocessid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IsIconic(hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsIconic(hwnd : HWND) -> windows_core::BOOL);
    unsafe { IsIconic(hwnd) }
}
#[inline]
pub unsafe fn IsWindowVisible(hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWindowVisible(hwnd : HWND) -> windows_core::BOOL);
    unsafe { IsWindowVisible(hwnd) }
}
#[inline]
pub unsafe fn IsZoomed(hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsZoomed(hwnd : HWND) -> windows_core::BOOL);
    unsafe { IsZoomed(hwnd) }
}
#[inline]
pub unsafe fn KillTimer(hwnd: Option<HWND>, uidevent: usize) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn KillTimer(hwnd : HWND, uidevent : usize) -> windows_core::BOOL);
    unsafe { KillTimer(hwnd.unwrap_or(core::mem::zeroed()) as _, uidevent) }
}
#[inline]
pub unsafe fn LoadImageW<P1>(
    hinst: Option<HINSTANCE>,
    name: P1,
    r#type: u32,
    cx: i32,
    cy: i32,
    fuload: u32,
) -> HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadImageW(hinst : HINSTANCE, name : windows_core::PCWSTR, r#type : u32, cx : i32, cy : i32, fuload : u32) -> HANDLE);
    unsafe {
        LoadImageW(
            hinst.unwrap_or(core::mem::zeroed()) as _,
            name.param().abi(),
            r#type,
            cx,
            cy,
            fuload,
        )
    }
}
#[inline]
pub unsafe fn LoadLibraryW<P0>(lplibfilename: P0) -> HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_core::PCWSTR) -> HMODULE);
    unsafe { LoadLibraryW(lplibfilename.param().abi()) }
}
#[inline]
pub unsafe fn MessageBoxW<P1, P2>(hwnd: Option<HWND>, lptext: P1, lpcaption: P2, utype: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn MessageBoxW(hwnd : HWND, lptext : windows_core::PCWSTR, lpcaption : windows_core::PCWSTR, utype : u32) -> i32);
    unsafe {
        MessageBoxW(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            lptext.param().abi(),
            lpcaption.param().abi(),
            utype,
        )
    }
}
#[inline]
pub unsafe fn MonitorFromWindow(hwnd: HWND, dwflags: u32) -> HMONITOR {
    windows_core::link!("user32.dll" "system" fn MonitorFromWindow(hwnd : HWND, dwflags : u32) -> HMONITOR);
    unsafe { MonitorFromWindow(hwnd, dwflags) }
}
#[inline]
pub unsafe fn OpenEventW<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenEventW(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> HANDLE);
    unsafe { OpenEventW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[inline]
pub unsafe fn OpenProcess(dwdesiredaccess: u32, binherithandle: bool, dwprocessid: u32) -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwprocessid : u32) -> HANDLE);
    unsafe { OpenProcess(dwdesiredaccess, binherithandle.into(), dwprocessid) }
}
#[inline]
pub unsafe fn PostMessageW(
    hwnd: Option<HWND>,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PostMessageW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> windows_core::BOOL);
    unsafe {
        PostMessageW(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            msg,
            wparam,
            lparam,
        )
    }
}
#[inline]
pub unsafe fn PostQuitMessage(nexitcode: i32) {
    windows_core::link!("user32.dll" "system" fn PostQuitMessage(nexitcode : i32));
    unsafe { PostQuitMessage(nexitcode) }
}
#[inline]
pub unsafe fn QueryFullProcessImageNameW(
    hprocess: HANDLE,
    dwflags: u32,
    lpexename: windows_core::PWSTR,
    lpdwsize: *mut u32,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryFullProcessImageNameW(hprocess : HANDLE, dwflags : u32, lpexename : windows_core::PWSTR, lpdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { QueryFullProcessImageNameW(hprocess, dwflags, lpexename, lpdwsize as _) }
}
#[inline]
pub unsafe fn RegDeleteKeyValueW<P1, P2>(hkey: HKEY, lpsubkey: P1, lpvaluename: P2) -> LSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteKeyValueW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR) -> LSTATUS);
    unsafe { RegDeleteKeyValueW(hkey, lpsubkey.param().abi(), lpvaluename.param().abi()) }
}
#[inline]
pub unsafe fn RegGetValueW<P1, P2>(
    hkey: HKEY,
    lpsubkey: P1,
    lpvalue: P2,
    dwflags: u32,
    pdwtype: Option<*mut u32>,
    pvdata: Option<*mut core::ffi::c_void>,
    pcbdata: Option<*mut u32>,
) -> LSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegGetValueW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> LSTATUS);
    unsafe {
        RegGetValueW(
            hkey,
            lpsubkey.param().abi(),
            lpvalue.param().abi(),
            dwflags,
            pdwtype.unwrap_or(core::mem::zeroed()) as _,
            pvdata.unwrap_or(core::mem::zeroed()) as _,
            pcbdata.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn RegSetKeyValueW<P1, P2>(
    hkey: HKEY,
    lpsubkey: P1,
    lpvaluename: P2,
    dwtype: u32,
    lpdata: Option<*const core::ffi::c_void>,
    cbdata: u32,
) -> LSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSetKeyValueW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> LSTATUS);
    unsafe {
        RegSetKeyValueW(
            hkey,
            lpsubkey.param().abi(),
            lpvaluename.param().abi(),
            dwtype,
            lpdata.unwrap_or(core::mem::zeroed()) as _,
            cbdata,
        )
    }
}
#[inline]
pub unsafe fn RegisterClassW(lpwndclass: *const WNDCLASSW) -> ATOM {
    windows_core::link!("user32.dll" "system" fn RegisterClassW(lpwndclass : *const WNDCLASSW) -> ATOM);
    unsafe { RegisterClassW(lpwndclass) }
}
#[inline]
pub unsafe fn RegisterWindowMessageW<P0>(lpstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring : windows_core::PCWSTR) -> u32);
    unsafe { RegisterWindowMessageW(lpstring.param().abi()) }
}
#[inline]
pub unsafe fn SendMessageTimeoutW(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    fuflags: u32,
    utimeout: u32,
    lpdwresult: Option<*mut usize>,
) -> LRESULT {
    windows_core::link!("user32.dll" "system" fn SendMessageTimeoutW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM, fuflags : u32, utimeout : u32, lpdwresult : *mut usize) -> LRESULT);
    unsafe {
        SendMessageTimeoutW(
            hwnd,
            msg,
            wparam,
            lparam,
            fuflags,
            utimeout,
            lpdwresult.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn SetForegroundWindow(hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetForegroundWindow(hwnd : HWND) -> windows_core::BOOL);
    unsafe { SetForegroundWindow(hwnd) }
}
#[inline]
pub unsafe fn SetTimer(
    hwnd: Option<HWND>,
    nidevent: usize,
    uelapse: u32,
    lptimerfunc: TIMERPROC,
) -> usize {
    windows_core::link!("user32.dll" "system" fn SetTimer(hwnd : HWND, nidevent : usize, uelapse : u32, lptimerfunc : TIMERPROC) -> usize);
    unsafe {
        SetTimer(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            nidevent,
            uelapse,
            lptimerfunc,
        )
    }
}
#[inline]
pub unsafe fn SetWinEventHook(
    eventmin: u32,
    eventmax: u32,
    hmodwineventproc: Option<HMODULE>,
    pfnwineventproc: WINEVENTPROC,
    idprocess: u32,
    idthread: u32,
    dwflags: u32,
) -> HWINEVENTHOOK {
    windows_core::link!("user32.dll" "system" fn SetWinEventHook(eventmin : u32, eventmax : u32, hmodwineventproc : HMODULE, pfnwineventproc : WINEVENTPROC, idprocess : u32, idthread : u32, dwflags : u32) -> HWINEVENTHOOK);
    unsafe {
        SetWinEventHook(
            eventmin,
            eventmax,
            hmodwineventproc.unwrap_or(core::mem::zeroed()) as _,
            pfnwineventproc,
            idprocess,
            idthread,
            dwflags,
        )
    }
}
#[inline]
pub unsafe fn Shell_NotifyIconW(
    dwmessage: u32,
    lpdata: *const NOTIFYICONDATAW,
) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn Shell_NotifyIconW(dwmessage : u32, lpdata : *const NOTIFYICONDATAW) -> windows_core::BOOL);
    unsafe { Shell_NotifyIconW(dwmessage, lpdata) }
}
#[inline]
pub unsafe fn SystemParametersInfoW(
    uiaction: u32,
    uiparam: u32,
    pvparam: *mut core::ffi::c_void,
    fwinini: u32,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SystemParametersInfoW(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32) -> windows_core::BOOL);
    unsafe { SystemParametersInfoW(uiaction, uiparam, pvparam as _, fwinini) }
}
#[inline]
pub unsafe fn TrackPopupMenu(
    hmenu: HMENU,
    uflags: u32,
    x: i32,
    y: i32,
    nreserved: Option<i32>,
    hwnd: HWND,
    prcrect: Option<*const RECT>,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TrackPopupMenu(hmenu : HMENU, uflags : u32, x : i32, y : i32, nreserved : i32, hwnd : HWND, prcrect : *const RECT) -> windows_core::BOOL);
    unsafe {
        TrackPopupMenu(
            hmenu,
            uflags,
            x,
            y,
            nreserved.unwrap_or(core::mem::zeroed()) as _,
            hwnd,
            prcrect.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn TranslateMessage(lpmsg: *const MSG) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> windows_core::BOOL);
    unsafe { TranslateMessage(lpmsg) }
}
#[inline]
pub unsafe fn UnhookWinEvent(hwineventhook: HWINEVENTHOOK) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnhookWinEvent(hwineventhook : HWINEVENTHOOK) -> windows_core::BOOL);
    unsafe { UnhookWinEvent(hwineventhook) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ATOM(pub u16);
pub const AppVisibility: windows_core::GUID =
    windows_core::GUID::from_u128(0x7e5fe3d9_985f_4908_91f9_ee19f9fd1514);
pub const CC_FULLOPEN: i32 = 2;
pub const CC_RGBINIT: i32 = 1;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: HWND,
    pub hInstance: HWND,
    pub rgbResult: COLORREF,
    pub lpCustColors: *mut COLORREF,
    pub Flags: u32,
    pub lCustData: LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: HWND,
    pub hInstance: HWND,
    pub rgbResult: COLORREF,
    pub lpCustColors: *mut COLORREF,
    pub Flags: u32,
    pub lCustData: LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
pub type CLSCTX = u32;
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1;
pub type COINIT = i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct COLORREF(pub u32);
pub const DWMWA_CLOAKED: DWMWINDOWATTRIBUTE = 14;
pub type DWMWINDOWATTRIBUTE = i32;
pub const ERROR_ALREADY_EXISTS: i32 = 183;
pub const ERROR_FILE_NOT_FOUND: i32 = 2;
pub const ERROR_NOT_FOUND: i32 = 1168;
pub const EVENT_OBJECT_CLOAKED: i32 = 32791;
pub const EVENT_OBJECT_DESTROY: i32 = 32769;
pub const EVENT_OBJECT_HIDE: i32 = 32771;
pub const EVENT_OBJECT_LOCATIONCHANGE: i32 = 32779;
pub const EVENT_OBJECT_UNCLOAKED: i32 = 32792;
pub const EVENT_SYSTEM_FOREGROUND: i32 = 3;
pub const EVENT_SYSTEM_MINIMIZEEND: i32 = 23;
pub const EVENT_SYSTEM_MINIMIZESTART: i32 = 22;
pub const EVENT_SYSTEM_MOVESIZEEND: i32 = 11;
pub const EVENT_SYSTEM_SWITCHEND: i32 = 21;
pub const EVENT_SYSTEM_SWITCHSTART: i32 = 20;
pub const E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80004005_u32 as _);
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
pub const GA_ROOT: i32 = 2;
pub const GWL_EXSTYLE: i32 = -20;
pub const GW_OWNER: i32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HBRUSH(pub *mut core::ffi::c_void);
pub const HCF_HIGHCONTRASTON: i32 = 1;
pub type HCURSOR = HICON;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HICON(pub *mut core::ffi::c_void);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIGHCONTRASTW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszDefaultScheme: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HINSTANCE(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HKEY(pub *mut core::ffi::c_void);
pub const HKEY_CURRENT_USER: HKEY = HKEY(-2147483647 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMENU(pub *mut core::ffi::c_void);
pub type HMODULE = HINSTANCE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMONITOR(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HWINEVENTHOOK(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HWND(pub *mut core::ffi::c_void);
pub const HWND_MESSAGE: HWND = HWND(-3 as _);
windows_core::imp::define_interface!(
    IAppVisibility,
    IAppVisibility_Vtbl,
    0x2246ea2d_caea_4444_a3c4_6de827e44313
);
windows_core::imp::interface_hierarchy!(IAppVisibility, windows_core::IUnknown);
impl IAppVisibility {
    pub unsafe fn GetAppVisibilityOnMonitor(
        &self,
        hmonitor: HMONITOR,
    ) -> windows_core::Result<MONITOR_APP_VISIBILITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAppVisibilityOnMonitor)(
                windows_core::Interface::as_raw(self),
                hmonitor,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn IsLauncherVisible(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLauncherVisible)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Advise<P0>(&self, pcallback: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IAppVisibilityEvents>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(
                windows_core::Interface::as_raw(self),
                pcallback.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Unadvise)(
                windows_core::Interface::as_raw(self),
                dwcookie,
            )
        }
    }
}
#[repr(C)]
pub struct IAppVisibility_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAppVisibilityOnMonitor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HMONITOR,
        *mut MONITOR_APP_VISIBILITY,
    ) -> windows_core::HRESULT,
    pub IsLauncherVisible: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IAppVisibility_Impl: windows_core::IUnknownImpl {
    fn GetAppVisibilityOnMonitor(
        &self,
        hmonitor: HMONITOR,
    ) -> windows_core::Result<MONITOR_APP_VISIBILITY>;
    fn IsLauncherVisible(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Advise(
        &self,
        pcallback: windows_core::Ref<IAppVisibilityEvents>,
    ) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl IAppVisibility_Vtbl {
    pub const fn new<Identity: IAppVisibility_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAppVisibilityOnMonitor<
            Identity: IAppVisibility_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hmonitor: HMONITOR,
            pmode: *mut MONITOR_APP_VISIBILITY,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppVisibility_Impl::GetAppVisibilityOnMonitor(
                    this,
                    core::mem::transmute_copy(&hmonitor),
                ) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLauncherVisible<
            Identity: IAppVisibility_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pfvisible: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppVisibility_Impl::IsLauncherVisible(this) {
                    Ok(ok__) => {
                        pfvisible.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Advise<Identity: IAppVisibility_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pcallback: *mut core::ffi::c_void,
            pdwcookie: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppVisibility_Impl::Advise(this, core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IAppVisibility_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwcookie: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppVisibility_Impl::Unadvise(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAppVisibilityOnMonitor: GetAppVisibilityOnMonitor::<Identity, OFFSET>,
            IsLauncherVisible: IsLauncherVisible::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppVisibility as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppVisibility {}
windows_core::imp::define_interface!(
    IAppVisibilityEvents,
    IAppVisibilityEvents_Vtbl,
    0x6584ce6b_7d82_49c2_89c9_c6bc02ba8c38
);
windows_core::imp::interface_hierarchy!(IAppVisibilityEvents, windows_core::IUnknown);
impl IAppVisibilityEvents {
    pub unsafe fn AppVisibilityOnMonitorChanged(
        &self,
        hmonitor: HMONITOR,
        previousmode: MONITOR_APP_VISIBILITY,
        currentmode: MONITOR_APP_VISIBILITY,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).AppVisibilityOnMonitorChanged)(
                windows_core::Interface::as_raw(self),
                hmonitor,
                previousmode,
                currentmode,
            )
        }
    }
    pub unsafe fn LauncherVisibilityChange(
        &self,
        currentvisiblestate: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).LauncherVisibilityChange)(
                windows_core::Interface::as_raw(self),
                currentvisiblestate.into(),
            )
        }
    }
}
#[repr(C)]
pub struct IAppVisibilityEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AppVisibilityOnMonitorChanged: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HMONITOR,
        MONITOR_APP_VISIBILITY,
        MONITOR_APP_VISIBILITY,
    ) -> windows_core::HRESULT,
    pub LauncherVisibilityChange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
pub trait IAppVisibilityEvents_Impl: windows_core::IUnknownImpl {
    fn AppVisibilityOnMonitorChanged(
        &self,
        hmonitor: HMONITOR,
        previousmode: MONITOR_APP_VISIBILITY,
        currentmode: MONITOR_APP_VISIBILITY,
    ) -> windows_core::Result<()>;
    fn LauncherVisibilityChange(
        &self,
        currentvisiblestate: windows_core::BOOL,
    ) -> windows_core::Result<()>;
}
impl IAppVisibilityEvents_Vtbl {
    pub const fn new<Identity: IAppVisibilityEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AppVisibilityOnMonitorChanged<
            Identity: IAppVisibilityEvents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hmonitor: HMONITOR,
            previousmode: MONITOR_APP_VISIBILITY,
            currentmode: MONITOR_APP_VISIBILITY,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppVisibilityEvents_Impl::AppVisibilityOnMonitorChanged(
                    this,
                    core::mem::transmute_copy(&hmonitor),
                    core::mem::transmute_copy(&previousmode),
                    core::mem::transmute_copy(&currentmode),
                )
                .into()
            }
        }
        unsafe extern "system" fn LauncherVisibilityChange<
            Identity: IAppVisibilityEvents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentvisiblestate: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppVisibilityEvents_Impl::LauncherVisibilityChange(
                    this,
                    core::mem::transmute_copy(&currentvisiblestate),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AppVisibilityOnMonitorChanged: AppVisibilityOnMonitorChanged::<Identity, OFFSET>,
            LauncherVisibilityChange: LauncherVisibilityChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppVisibilityEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppVisibilityEvents {}
pub const IMAGE_ICON: i32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPARAM(pub isize);
pub type LPCCHOOKPROC = Option<
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> usize,
>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LRESULT(pub isize);
pub const LR_DEFAULTCOLOR: i32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LSTATUS(pub i32);
pub const MB_ICONERROR: i32 = 16;
pub const MB_OK: i32 = 0;
pub const MF_BYCOMMAND: i32 = 0;
pub const MF_CHECKED: i32 = 8;
pub const MF_GRAYED: i32 = 1;
pub const MF_POPUP: i32 = 16;
pub const MF_SEPARATOR: i32 = 2048;
pub const MF_STRING: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}
pub type MONITOR_APP_VISIBILITY = i32;
pub const MONITOR_DEFAULTTONEAREST: i32 = 2;
pub const MONITOR_DEFAULTTONULL: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}
pub const NIF_ICON: i32 = 2;
pub const NIF_INFO: i32 = 16;
pub const NIF_MESSAGE: i32 = 1;
pub const NIF_TIP: i32 = 4;
pub const NIIF_INFO: i32 = 1;
pub const NIIF_WARNING: i32 = 2;
pub const NIM_ADD: i32 = 0;
pub const NIM_DELETE: i32 = 2;
pub const NIM_MODIFY: i32 = 1;
pub const NIM_SETVERSION: i32 = 4;
pub const NIN_KEYSELECT: i32 = 1025;
pub const NIN_SELECT: i32 = 1024;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: HICON,
}
#[cfg(target_arch = "x86")]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(target_arch = "x86")]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: HICON,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NOTIFYICON_VERSION_4: i32 = 4;
pub const OBJID_WINDOW: i32 = 0;
pub const PBT_APMRESUMEAUTOMATIC: i32 = 18;
pub const PBT_APMRESUMESUSPEND: i32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
pub const PROCESS_QUERY_LIMITED_INFORMATION: i32 = 4096;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub const REG_SZ: u32 = 1;
pub const RRF_RT_REG_DWORD: i32 = 16;
pub const RRF_RT_REG_SZ: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: windows_core::BOOL,
}
pub const SMTO_ABORTIFHUNG: i32 = 2;
pub const SM_CXSMICON: i32 = 49;
pub const SM_CYSMICON: i32 = 50;
pub const SPI_GETHIGHCONTRAST: i32 = 66;
pub const SPI_SETHIGHCONTRAST: i32 = 67;
pub const SYNCHRONIZE: i32 = 1048576;
pub type TIMERPROC =
    Option<unsafe extern "system" fn(param0: HWND, param1: u32, param2: usize, param3: u32)>;
pub const TPM_RETURNCMD: i32 = 256;
pub const TPM_RIGHTBUTTON: i32 = 2;
pub type WINEVENTPROC = Option<
    unsafe extern "system" fn(
        hwineventhook: HWINEVENTHOOK,
        event: u32,
        hwnd: HWND,
        idobject: i32,
        idchild: i32,
        ideventthread: u32,
        dwmseventtime: u32,
    ),
>;
pub const WINEVENT_OUTOFCONTEXT: i32 = 0;
pub const WINEVENT_SKIPOWNPROCESS: i32 = 2;
pub const WM_APP: i32 = 32768;
pub const WM_CONTEXTMENU: i32 = 123;
pub const WM_DISPLAYCHANGE: i32 = 126;
pub const WM_DWMCOMPOSITIONCHANGED: i32 = 798;
pub const WM_LBUTTONUP: i32 = 514;
pub const WM_NULL: i32 = 0;
pub const WM_POWERBROADCAST: i32 = 536;
pub const WM_RBUTTONUP: i32 = 517;
pub const WM_SETTINGCHANGE: i32 = 26;
pub const WM_THEMECHANGED: i32 = 794;
pub const WM_TIMER: i32 = 275;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WNDCLASSW {
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: windows_core::PCWSTR,
    pub lpszClassName: windows_core::PCWSTR,
}
pub type WNDENUMPROC =
    Option<unsafe extern "system" fn(param0: HWND, param1: LPARAM) -> windows_core::BOOL>;
pub type WNDPROC = Option<
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT,
>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WPARAM(pub usize);
pub const WS_EX_TOOLWINDOW: i32 = 128;
