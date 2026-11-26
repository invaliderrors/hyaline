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
pub unsafe fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT {
    windows_core::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> LRESULT);
    unsafe { DispatchMessageW(lpmsg) }
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
pub unsafe fn LoadLibraryW<P0>(lplibfilename: P0) -> HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_core::PCWSTR) -> HMODULE);
    unsafe { LoadLibraryW(lplibfilename.param().abi()) }
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
pub unsafe fn TranslateMessage(lpmsg: *const MSG) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> windows_core::BOOL);
    unsafe { TranslateMessage(lpmsg) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ATOM(pub u16);
pub const ERROR_ALREADY_EXISTS: i32 = 183;
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
pub struct HMENU(pub *mut core::ffi::c_void);
pub type HMODULE = HINSTANCE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HWND(pub *mut core::ffi::c_void);
pub const HWND_MESSAGE: HWND = HWND(-3 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPARAM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LRESULT(pub isize);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: windows_core::BOOL,
}
pub const SMTO_ABORTIFHUNG: i32 = 2;
pub const SYNCHRONIZE: i32 = 1048576;
pub type TIMERPROC =
    Option<unsafe extern "system" fn(param0: HWND, param1: u32, param2: usize, param3: u32)>;
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
pub type WNDPROC = Option<
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT,
>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WPARAM(pub usize);
