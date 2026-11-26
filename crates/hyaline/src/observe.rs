//! Reads what is happening on each taskbar's monitor: Start/Search/Task View in the
//! foreground, and maximized or visible windows.

use crate::bindings::{
    CloseHandle, DWMWA_CLOAKED, DwmGetWindowAttribute, EnumWindows, GW_OWNER, GWL_EXSTYLE,
    GetClassNameW, GetForegroundWindow, GetMonitorInfoW, GetShellWindow, GetWindow,
    GetWindowLongPtrW, GetWindowRect, GetWindowThreadProcessId, HMONITOR, HWND, IsIconic,
    IsWindowVisible, IsZoomed, LPARAM, MONITOR_DEFAULTTONEAREST, MONITOR_DEFAULTTONULL,
    MONITORINFO, MonitorFromWindow, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    QueryFullProcessImageNameW, RECT, WS_EX_TOOLWINDOW,
};
use hyaline_core::states::Observed;
use hyaline_core::windows::{WindowFacts, WindowKind, classify, foreground, is_shell_class};
use windows_core::{BOOL, PWSTR};

/// `PROCESS_NAME_WIN32`: a Win32 path rather than a native one.
const PROCESS_NAME_WIN32: u32 = 0;

/// What is happening on the monitor of each of `taskbars`, in the same order.
pub fn observe(taskbars: &[HWND], launcher_visible: bool, alt_tab: bool) -> Vec<Observed> {
    let monitors: Vec<HMONITOR> = taskbars
        .iter()
        .map(|&taskbar| monitor_of(taskbar))
        .collect();
    let mut observed = vec![Observed::default(); taskbars.len()];

    // SAFETY: no arguments.
    let active = unsafe { GetForegroundWindow() };
    let surface = foreground(
        &class_name(active),
        &exe_name(active),
        launcher_visible,
        alt_tab,
    );
    let active_monitor = monitor_of(active);
    for (observation, monitor) in observed.iter_mut().zip(&monitors) {
        // Task View covers every monitor; Start and Search open on one.
        observation.task_view = surface.task_view;
        if *monitor == active_monitor {
            observation.start = surface.start;
            observation.search = surface.search;
        }
    }

    let work_areas: Vec<RECT> = monitors.iter().map(|&monitor| work_area(monitor)).collect();
    let mut scan = Scan {
        monitors: &monitors,
        work_areas: &work_areas,
        observed: &mut observed,
        // SAFETY: no arguments.
        shell: unsafe { GetShellWindow() },
    };
    // SAFETY: `scan` outlives the synchronous enumeration and is only used by `visit`.
    let _ = unsafe { EnumWindows(Some(visit), LPARAM((&raw mut scan) as isize)) };
    observed
}

struct Scan<'a> {
    monitors: &'a [HMONITOR],
    work_areas: &'a [RECT],
    observed: &'a mut [Observed],
    shell: HWND,
}

unsafe extern "system" fn visit(hwnd: HWND, scan: LPARAM) -> BOOL {
    // SAFETY: `scan` is the `Scan` passed by `observe`, alive for the whole enumeration.
    let scan = unsafe { &mut *(scan.0 as *mut Scan) };
    // Windows parked off-screen belong to no monitor.
    // SAFETY: any window handle; returns null when the window is on no monitor.
    let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONULL.cast_unsigned()) };
    let Some(index) = scan.monitors.iter().position(|&m| m == monitor) else {
        return BOOL(1);
    };
    match classify(facts(hwnd, scan.shell, &scan.work_areas[index])) {
        Some(WindowKind::Maximized) => scan.observed[index].maximized_window = true,
        Some(WindowKind::Visible) => scan.observed[index].visible_window = true,
        None => {}
    }
    BOOL(1)
}

fn facts(hwnd: HWND, shell: HWND, work: &RECT) -> WindowFacts {
    // SAFETY: querying any window handle is safe; a stale one just reports false.
    if !unsafe { IsWindowVisible(hwnd) }.as_bool() {
        return WindowFacts::default();
    }
    let mut cloaked = 0u32;
    // SAFETY: `cloaked` is a 4-byte out buffer of the stated size.
    let cloaked = unsafe {
        DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED.cast_unsigned(),
            (&raw mut cloaked).cast(),
            4,
        )
    }
    .is_ok()
        && cloaked != 0;
    let mut rect = RECT::default();
    // SAFETY: `rect` is a valid out pointer.
    let _ = unsafe { GetWindowRect(hwnd, &raw mut rect) };
    // SAFETY: plain queries on a window handle.
    let (minimized, maximized, ex_style, owner) = unsafe {
        (
            IsIconic(hwnd).as_bool(),
            IsZoomed(hwnd).as_bool(),
            GetWindowLongPtrW(hwnd, GWL_EXSTYLE),
            GetWindow(hwnd, GW_OWNER.cast_unsigned()),
        )
    };
    WindowFacts {
        visible: true,
        cloaked,
        minimized,
        maximized,
        fills_work_area: rect.left <= work.left
            && rect.top <= work.top
            && rect.right >= work.right
            && rect.bottom >= work.bottom,
        tool_window: ex_style & WS_EX_TOOLWINDOW as isize != 0,
        owned: !owner.0.is_null(),
        zero_size: rect.right <= rect.left || rect.bottom <= rect.top,
        shell: hwnd == shell || is_shell_class(&class_name(hwnd)),
    }
}

fn monitor_of(hwnd: HWND) -> HMONITOR {
    // SAFETY: any window handle; the nearest monitor is always returned.
    unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST.cast_unsigned()) }
}

fn work_area(monitor: HMONITOR) -> RECT {
    let mut info = MONITORINFO {
        cbSize: u32::try_from(size_of::<MONITORINFO>()).unwrap_or(u32::MAX),
        ..Default::default()
    };
    // SAFETY: `info` is initialized with its size.
    let _ = unsafe { GetMonitorInfoW(monitor, &raw mut info) };
    info.rcWork
}

fn class_name(hwnd: HWND) -> String {
    let mut class = [0u16; 256];
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)] // 256 fits in i32
    // SAFETY: the buffer and its length match.
    let len = unsafe { GetClassNameW(hwnd, PWSTR(class.as_mut_ptr()), class.len() as i32) };
    String::from_utf16_lossy(&class[..usize::try_from(len).unwrap_or(0)])
}

/// File name of the process that owns `hwnd` (empty if it cannot be read).
fn exe_name(hwnd: HWND) -> String {
    let mut pid = 0;
    // SAFETY: `pid` is a valid out pointer.
    unsafe { GetWindowThreadProcessId(hwnd, Some(&raw mut pid)) };
    // SAFETY: plain values; failure returns a null handle.
    let process = unsafe {
        OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION.cast_unsigned(),
            false,
            pid,
        )
    };
    if process.0.is_null() {
        return String::new();
    }
    let mut path = [0u16; 512];
    let mut len = u32::try_from(path.len()).unwrap_or(0);
    // SAFETY: the buffer and its length match; the handle was opened above.
    let read = unsafe {
        QueryFullProcessImageNameW(
            process,
            PROCESS_NAME_WIN32,
            PWSTR(path.as_mut_ptr()),
            &raw mut len,
        )
    }
    .as_bool();
    // SAFETY: opened above, closed once.
    let _ = unsafe { CloseHandle(process) };
    if !read {
        return String::new();
    }
    let path = String::from_utf16_lossy(&path[..len as usize]);
    path.rsplit('\\').next().unwrap_or_default().to_string()
}
