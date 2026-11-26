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
pub unsafe fn GetProcAddress<P1>(hmodule: HMODULE, lpprocname: P1) -> FARPROC
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : windows_core::PCSTR) -> FARPROC);
    unsafe { GetProcAddress(hmodule, lpprocname.param().abi()) }
}
#[inline]
pub unsafe fn GetSystemMetrics(nindex: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetSystemMetrics(nindex : i32) -> i32);
    unsafe { GetSystemMetrics(nindex) }
}
#[inline]
pub unsafe fn GetWindowThreadProcessId(hwnd: HWND, lpdwprocessid: Option<*mut u32>) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetWindowThreadProcessId(hwnd : HWND, lpdwprocessid : *mut u32) -> u32);
    unsafe { GetWindowThreadProcessId(hwnd, lpdwprocessid.unwrap_or(core::mem::zeroed()) as _) }
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
pub unsafe fn OpenEventW<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenEventW(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> HANDLE);
    unsafe { OpenEventW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
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
pub unsafe fn Shell_NotifyIconW(
    dwmessage: u32,
    lpdata: *const NOTIFYICONDATAW,
) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn Shell_NotifyIconW(dwmessage : u32, lpdata : *const NOTIFYICONDATAW) -> windows_core::BOOL);
    unsafe { Shell_NotifyIconW(dwmessage, lpdata) }
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ATOM(pub u16);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct COLORREF(pub u32);
pub const ERROR_ALREADY_EXISTS: i32 = 183;
pub const ERROR_FILE_NOT_FOUND: i32 = 2;
pub const ERROR_NOT_FOUND: i32 = 1168;
pub const E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80004005_u32 as _);
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HBRUSH(pub *mut core::ffi::c_void);
pub type HCURSOR = HICON;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HICON(pub *mut core::ffi::c_void);
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
pub struct HWND(pub *mut core::ffi::c_void);
pub const HWND_MESSAGE: HWND = HWND(-3 as _);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
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
pub const SYNCHRONIZE: i32 = 1048576;
pub type TIMERPROC =
    Option<unsafe extern "system" fn(param0: HWND, param1: u32, param2: usize, param3: u32)>;
pub const TPM_RETURNCMD: i32 = 256;
pub const TPM_RIGHTBUTTON: i32 = 2;
pub const WM_APP: i32 = 32768;
pub const WM_CONTEXTMENU: i32 = 123;
pub const WM_DISPLAYCHANGE: i32 = 126;
pub const WM_DWMCOMPOSITIONCHANGED: i32 = 798;
pub const WM_LBUTTONUP: i32 = 514;
pub const WM_NULL: i32 = 0;
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
