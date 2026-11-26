//! Hyaline: a tiny, fast Windows 11 taskbar customizer.
//!
//! Applies the look picked in the tray menu to every taskbar: an accent on the taskbar windows
//! plus the Explorer helper hiding their XAML background. Both are kept in place across theme,
//! display and Explorer restarts; the helper restores Windows' own look when this process exits.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[rustfmt::skip]
#[allow(clippy::all, clippy::pedantic, clippy::undocumented_unsafe_blocks, nonstandard_style, dead_code)]
mod bindings;
mod autostart;
mod config_file;
mod effect;
mod explorer;
mod inject;
mod observe;
mod tap_client;
mod tray;
mod watcher;

use bindings::{
    COINIT_APARTMENTTHREADED, CoInitializeEx, CreateMutexW, CreateWindowExW, DefWindowProcW,
    DispatchMessageW, ERROR_ALREADY_EXISTS, ERROR_NOT_FOUND, GetLastError, GetMessageW,
    GetModuleHandleW, HINSTANCE, HWND, KillTimer, LPARAM, LRESULT, MB_ICONERROR, MB_OK, MSG,
    MessageBoxW, PBT_APMRESUMEAUTOMATIC, PBT_APMRESUMESUSPEND, PostQuitMessage, RegisterClassW,
    RegisterWindowMessageW, SPI_SETHIGHCONTRAST, SetTimer, TranslateMessage, WM_DISPLAYCHANGE,
    WM_DWMCOMPOSITIONCHANGED, WM_POWERBROADCAST, WM_SETTINGCHANGE, WM_THEMECHANGED, WM_TIMER,
    WNDCLASSW, WPARAM,
};
use hyaline_core::appearance::{Appearance, Mode};
use hyaline_core::config::Config;
use hyaline_core::states::{Looks, Situation, Toggled};
use hyaline_core::tap::{
    Command, HELPER_READY, HOST_WINDOW_CLASS, INSTANCE_MUTEX, InitData, RestartGuard,
    retry_delay_ms,
};
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;
use windows_core::{HRESULT, HSTRING, PCWSTR, w};

static TAP_DLL: OnceLock<PathBuf> = OnceLock::new();
static TASKBAR_CREATED: AtomicU32 = AtomicU32::new(0);
/// Retries of loading the helper since the last success or Explorer start.
static ATTEMPT: AtomicU32 = AtomicU32::new(0);
const RETRY_TIMER: usize = 1;
/// One-shot timers that re-apply the accent after Explorer has re-applied its own: a quick
/// one, and a late one for when Explorer is busy and its own re-apply lands later.
const REAPPLY_TIMERS: [(usize, u32); 2] = [(2, 250), (3, 1500)];
/// Waits for a freshly loaded helper's window, which appears once Explorer's XAML tree reports
/// the taskbar (asynchronous, slower at sign-in).
const WAIT_TIMER: usize = 4;
static WAIT_ATTEMPT: AtomicU32 = AtomicU32::new(0);

thread_local! {
    /// The look currently picked. Only the UI thread touches it.
    static SETTINGS: Cell<Looks> = Cell::new(Looks::default());
    /// The look last applied to each taskbar window.
    static APPLIED: RefCell<Vec<(HWND, Appearance)>> = const { RefCell::new(Vec::new()) };
    /// Whether the helper was last told to hide the background.
    static CLEARED: Cell<Option<bool>> = const { Cell::new(None) };
    /// Set by Exit: nothing may be applied any more.
    static QUITTING: Cell<bool> = const { Cell::new(false) };
    /// True while the tray menu or the color picker is open (they run nested message loops,
    /// so another icon click could otherwise open a second one).
    static MENU_OPEN: Cell<bool> = const { Cell::new(false) };
    /// Whether the running Explorer has the helper loaded (so its restart may be our fault).
    static INJECTED: Cell<bool> = const { Cell::new(false) };
    static RESTARTS: RefCell<RestartGuard> = RefCell::new(RestartGuard::default());
    static STARTED: Instant = Instant::now();
    /// Whether the "older helper loaded" notice was shown for the running Explorer.
    static STALE_NOTIFIED: Cell<bool> = const { Cell::new(false) };
}

/// Outcome of [`ensure_tap`].
enum Ensure {
    /// The helper's windows exist.
    Ready(Vec<HWND>),
    /// No configured look needs the helper.
    Idle,
    /// Explorer's taskbar or XAML runtime is not up yet; try loading again shortly.
    NotReady,
    /// The helper is loaded but its window is not up yet; wait for it.
    Pending,
    /// A helper from another Hyaline build is loaded; it is left alone until Explorer restarts.
    Stale,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            report_fatal(&error.to_string());
            ExitCode::FAILURE
        }
    }
}

/// Shows a fatal error; release builds have no console.
fn report_fatal(text: &str) {
    eprintln!("hyaline: {text}");
    let text = HSTRING::from(text);
    // SAFETY: NUL-terminated strings alive for the modal call.
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(text.as_ptr()),
            w!("Hyaline"),
            (MB_OK | MB_ICONERROR).cast_unsigned(),
        )
    };
}

fn run() -> Result<(), Box<dyn Error>> {
    let mutex_name = HSTRING::from(INSTANCE_MUTEX);
    // SAFETY: default security, NUL-terminated name. The handle lives until the process exits,
    // which is what keeps a second instance out.
    let _instance = unsafe { CreateMutexW(None, false, PCWSTR(mutex_name.as_ptr())) };
    // SAFETY: no arguments.
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS.cast_unsigned() {
        return Ok(());
    }
    let dll = std::env::current_exe()?.with_file_name("hyaline_tap.dll");
    if !dll.is_file() {
        return Err(format!("{} not found", dll.display()).into());
    }
    let _ = TAP_DLL.set(dll);
    // SAFETY: constant, NUL-terminated message name.
    let taskbar_created = unsafe { RegisterWindowMessageW(w!("TaskbarCreated")) };
    TASKBAR_CREATED.store(taskbar_created, Ordering::Relaxed);
    // SAFETY: once per thread, before any COM use.
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED.cast_unsigned()) }.ok()?;
    let listener = create_listener_window()?;
    let (config, notice) = config_file::load();
    SETTINGS.set(config.looks);
    watcher::start(listener);
    tray::add(listener);
    if let Some(notice) = notice {
        tray::notify(listener, "Hyaline settings", &notice, true);
    }
    if let Err(error) = apply_settings(listener) {
        tray::remove(listener);
        return Err(error);
    }
    let mut msg = MSG::default();
    // SAFETY: `msg` is a valid out pointer; GetMessageW returns 0 on WM_QUIT and -1 on error.
    while unsafe { GetMessageW(&raw mut msg, None, 0, 0) }.0 > 0 {
        // SAFETY: `msg` was filled by GetMessageW.
        unsafe {
            let _ = TranslateMessage(&raw const msg);
            DispatchMessageW(&raw const msg);
        }
    }
    Ok(())
}

/// Brings the taskbar to the current settings, loading the helper when needed.
fn apply_settings(listener: HWND) -> Result<(), Box<dyn Error>> {
    let Some(dll) = TAP_DLL.get() else {
        return Ok(());
    };
    if RESTARTS.with_borrow(RestartGuard::paused) {
        return Ok(());
    }
    let needed = SETTINGS.get().needs_helper() && !effect::high_contrast();
    match ensure_tap(dll, needed)? {
        Ensure::Ready(windows) => {
            ATTEMPT.store(0, Ordering::Relaxed);
            attach(&windows)?;
            refresh(listener, true)?;
            schedule_reapply(listener);
        }
        Ensure::Idle => ATTEMPT.store(0, Ordering::Relaxed),
        Ensure::NotReady => schedule_retry(listener),
        Ensure::Pending => {
            ATTEMPT.store(0, Ordering::Relaxed);
            WAIT_ATTEMPT.store(0, Ordering::Relaxed);
            schedule_wait(listener);
        }
        Ensure::Stale => {
            if !STALE_NOTIFIED.replace(true) {
                tray::notify(
                    listener,
                    "Hyaline",
                    "Restart Explorer or sign out to finish updating Hyaline.",
                    false,
                );
            }
        }
    }
    Ok(())
}

/// Finds the helper, or loads it (with nothing changed yet) when a configured look needs it.
fn ensure_tap(dll: &Path, needed: bool) -> Result<Ensure, Box<dyn Error>> {
    let windows = tap_client::find_all();
    if !windows.is_empty() {
        INJECTED.set(true);
        return Ok(Ensure::Ready(windows));
    }
    let Some(explorer) = explorer::taskbar_process_id() else {
        return Ok(if needed {
            Ensure::NotReady
        } else {
            Ensure::Idle
        });
    };
    if explorer::has_helper(explorer) {
        INJECTED.set(true);
        if tap_client::stale_present() {
            return Ok(Ensure::Stale);
        }
        // Loaded but its window is not up yet.
        return Ok(Ensure::Pending);
    }
    if !needed {
        return Ok(Ensure::Idle);
    }
    // Load without changing anything: `refresh` applies the look in the right order (accent
    // before hiding the background), and if this process dies first nothing changed.
    let init = InitData {
        host_pid: std::process::id(),
        cleared: false,
    };
    // HRESULT_FROM_WIN32(ERROR_NOT_FOUND): Explorer's XAML runtime is not up yet.
    let xaml_not_ready = HRESULT((0x8007_0000_u32 | ERROR_NOT_FOUND.cast_unsigned()).cast_signed());
    match inject::inject(dll, explorer, init) {
        Ok(()) => {
            INJECTED.set(true);
            Ok(Ensure::Pending)
        }
        Err(error) if error.code() == xaml_not_ready => Ok(Ensure::NotReady),
        Err(error) => Err(error.into()),
    }
}

/// Attaches the helper windows to this process, so they restore the taskbar if it dies.
fn attach(windows: &[HWND]) -> Result<(), Box<dyn Error>> {
    let host_pid = std::process::id();
    for &hwnd in windows {
        tap_client::send(hwnd, Command::Attach { host_pid })?;
    }
    Ok(())
}

/// Observes every monitor and gives each taskbar the look of its situation; only changes are
/// applied unless `force`. Nothing is changed without the helper: if this process died, the
/// helper is what puts Windows' own look back.
fn refresh(listener: HWND, force: bool) -> Result<(), Box<dyn Error>> {
    watcher::handled();
    if QUITTING.get() {
        return Ok(());
    }
    let windows = tap_client::find_all();
    if windows.is_empty() {
        return Ok(());
    }
    let looks = SETTINGS.get();
    let taskbars = effect::taskbars();
    let observed = observe::observe(&taskbars, watcher::launcher_visible(), watcher::alt_tab());
    let high_contrast = effect::high_contrast();
    let windows_look = Appearance {
        mode: Mode::Normal,
        ..Appearance::default()
    };
    let resolved: Vec<Appearance> = observed
        .iter()
        .map(|o| {
            if high_contrast {
                windows_look
            } else {
                looks.resolve(*o)
            }
        })
        .collect();
    let cleared = resolved.iter().any(|look| look.mode != Mode::Normal);
    // A taskbar created since the last refresh (monitor plugged in, taskbar recreated) gets
    // Explorer's accent from Explorer at about the same time as ours.
    let new_taskbar = APPLIED.with_borrow(|applied| {
        taskbars
            .iter()
            .any(|hwnd| !applied.iter().any(|(h, _)| h == hwnd))
    });
    let mut failure: Option<Box<dyn Error>> = None;
    let mut resets_posted = false;
    if cleared {
        match apply_accents(&taskbars, &resolved, force) {
            Ok(posted) => resets_posted = posted,
            Err(error) => failure = Some(error),
        }
    } else {
        // `Restore` below makes Explorer put back every accent.
        APPLIED.with_borrow_mut(Vec::clear);
    }
    let cleared_changed = CLEARED.get() != Some(cleared);
    if !force && cleared && (resets_posted || cleared_changed || new_taskbar) {
        // Explorer handles the posted reset (or the helper's earlier restore) after this
        // returns and may override the accent just applied; apply once more shortly after.
        schedule_reapply(listener);
    }
    if force || cleared_changed {
        let command = if cleared {
            Command::Clear
        } else {
            Command::Restore
        };
        let mut sent = true;
        for &hwnd in &windows {
            if let Err(error) = tap_client::send(hwnd, command) {
                sent = false;
                failure.get_or_insert(error.into());
            }
        }
        // Unchanged after a failure, so the next refresh sends it again.
        if sent {
            CLEARED.set(Some(cleared));
        }
    }
    failure.map_or(Ok(()), Err)
}

/// Applies each taskbar's look. A taskbar resolving to Windows' own look while another does
/// not gets Explorer's accent back (its background stays hidden: per-monitor hiding is not
/// supported). Resetting may make Explorer touch every taskbar, so the others are then
/// re-applied. Returns whether any reset was posted. A taskbar that fails is left out of
/// `APPLIED` (retried on the next refresh) and does not stop the others; the first error is
/// returned after all were tried.
fn apply_accents(
    taskbars: &[HWND],
    resolved: &[Appearance],
    force: bool,
) -> Result<bool, Box<dyn Error>> {
    APPLIED.with_borrow_mut(|applied| {
        applied.retain(|(hwnd, _)| taskbars.contains(hwnd));
        let changed =
            |hwnd: HWND, look: &Appearance| !applied.iter().any(|(h, a)| *h == hwnd && a == look);
        let resets: Vec<HWND> = taskbars
            .iter()
            .zip(resolved)
            .filter(|(hwnd, look)| look.mode == Mode::Normal && (force || changed(**hwnd, look)))
            .map(|(hwnd, _)| *hwnd)
            .collect();
        for &hwnd in &resets {
            effect::reset(hwnd);
        }
        let reapply_all = force || !resets.is_empty();
        let mut done = Vec::with_capacity(taskbars.len());
        let mut failure = None;
        for (&hwnd, &look) in taskbars.iter().zip(resolved) {
            if look.mode != Mode::Normal
                && (reapply_all || changed(hwnd, &look))
                && let Err(error) = effect::apply_to(hwnd, &look)
            {
                failure.get_or_insert(error);
                continue;
            }
            done.push((hwnd, look));
        }
        *applied = done;
        match failure {
            Some(error) => Err(error.into()),
            None => Ok(!resets.is_empty()),
        }
    })
}

fn schedule_reapply(listener: HWND) {
    for (id, delay) in REAPPLY_TIMERS {
        // SAFETY: our own window; re-arming an existing timer just restarts it.
        unsafe { SetTimer(Some(listener), id, delay, None) };
    }
}

fn schedule_wait(listener: HWND) {
    let attempt = WAIT_ATTEMPT.fetch_add(1, Ordering::Relaxed);
    match retry_delay_ms(attempt) {
        // SAFETY: `listener` is our own window; the timer is killed when it fires.
        Some(delay) => unsafe {
            SetTimer(Some(listener), WAIT_TIMER, delay, None);
        },
        None => eprintln!("hyaline: the helper's window never appeared"),
    }
}

/// The helper's window should be up by now: apply the look, or keep waiting.
fn on_wait(listener: HWND) {
    let windows = tap_client::find_all();
    if windows.is_empty() {
        schedule_wait(listener);
        return;
    }
    if let Err(error) = attach(&windows).and_then(|()| refresh(listener, true)) {
        eprintln!("hyaline: applying the look failed: {error}");
    }
    schedule_reapply(listener);
}

fn schedule_retry(listener: HWND) {
    let attempt = ATTEMPT.fetch_add(1, Ordering::Relaxed);
    match retry_delay_ms(attempt) {
        // SAFETY: `listener` is our own window; the timer is killed when it fires.
        Some(delay) => unsafe {
            SetTimer(Some(listener), RETRY_TIMER, delay, None);
        },
        None => eprintln!("hyaline: Explorer did not become ready; waiting for its next restart"),
    }
}

fn try_load(listener: HWND) {
    if let Err(error) = apply_settings(listener) {
        eprintln!("hyaline: loading the helper failed: {error}");
    }
}

/// Stores, saves and applies settings changed in the menu.
fn update(listener: HWND, looks: Looks) {
    SETTINGS.set(looks);
    // Picking a look is the owner asking to try again after a crash-loop pause.
    RESTARTS.with_borrow_mut(RestartGuard::reset);
    if let Err(error) = config_file::save(Config { looks }) {
        tray::notify(
            listener,
            "Hyaline settings",
            &format!("Could not save settings: {error}"),
            true,
        );
    }
    if let Err(error) = apply_settings(listener) {
        tray::notify(
            listener,
            "Hyaline",
            &format!("Could not apply the look: {error}"),
            true,
        );
    }
}

/// Changes one situation's setting. Picking a look for a situation also turns it on.
fn edit(listener: HWND, situation: Situation, change: impl FnOnce(&mut Toggled)) {
    let mut looks = SETTINGS.get();
    let mut toggled = looks.toggled(situation);
    change(&mut toggled);
    looks.set(situation, toggled);
    update(listener, looks);
}

fn on_menu(listener: HWND, anchor: bindings::POINT) {
    if MENU_OPEN.replace(true) {
        return;
    }
    let looks = SETTINGS.get();
    let choice = tray::show_menu(listener, anchor, &looks, autostart::is_enabled());
    match choice {
        None => {}
        Some(tray::Choice::Toggle(situation)) => {
            edit(listener, situation, |t| t.enabled = !t.enabled);
        }
        Some(tray::Choice::Mode(situation, mode)) => edit(listener, situation, |t| {
            t.look.mode = mode;
            t.enabled = true;
        }),
        Some(tray::Choice::Opacity(situation, opacity)) => edit(listener, situation, |t| {
            t.look.opacity = opacity;
            t.enabled = true;
        }),
        Some(tray::Choice::Color(situation)) => {
            let current = looks.toggled(situation).look.color;
            if let Some(color) = tray::pick_color(listener, current) {
                // `edit` reads the settings again: the dialog ran its own message loop.
                edit(listener, situation, |t| {
                    t.look.color = color;
                    t.enabled = true;
                });
            }
        }
        Some(tray::Choice::Autostart) => {
            if let Err(error) = autostart::set(!autostart::is_enabled()) {
                tray::notify(
                    listener,
                    "Hyaline",
                    &format!("Could not change Start with Windows: {error}"),
                    true,
                );
            }
        }
        Some(tray::Choice::Exit) => exit(listener),
    }
    MENU_OPEN.set(false);
}

/// Counts a restart of an Explorer that had the helper loaded; says so once if that pauses.
fn on_explorer_restart(listener: HWND) {
    let now = u64::try_from(STARTED.with(Instant::elapsed).as_millis()).unwrap_or(u64::MAX);
    let paused = RESTARTS.with_borrow_mut(|guard| !guard.paused() && guard.record(now));
    if paused {
        tray::notify(
            listener,
            "Hyaline paused",
            "Explorer keeps restarting, so Hyaline stepped aside. Change any setting to resume.",
            true,
        );
    }
}

/// Puts Windows' own look back, removes the icon and quits.
fn exit(listener: HWND) {
    QUITTING.set(true);
    watcher::stop();
    for hwnd in tap_client::find_all() {
        if let Err(error) = tap_client::send(hwnd, Command::Restore) {
            eprintln!("hyaline: restoring the taskbar failed: {error}");
        }
    }
    tray::remove(listener);
    // SAFETY: ends this thread's message loop.
    unsafe { PostQuitMessage(0) };
}

/// A hidden top-level window: `TaskbarCreated` is broadcast only to top-level windows. It also
/// owns the tray icon and the timers.
fn create_listener_window() -> Result<HWND, Box<dyn Error>> {
    // SAFETY: null name = this executable's module.
    let instance = HINSTANCE(unsafe { GetModuleHandleW(PCWSTR::null()) }.0);
    let class_name = HSTRING::from(HOST_WINDOW_CLASS);
    let class = PCWSTR(class_name.as_ptr());
    let window_class = WNDCLASSW {
        lpfnWndProc: Some(window_proc),
        hInstance: instance,
        lpszClassName: class,
        ..Default::default()
    };
    // SAFETY: `window_class` is fully initialized; `class_name` outlives both calls.
    if unsafe { RegisterClassW(&raw const window_class) }.0 == 0 {
        return Err(windows_core::Error::from_thread().into());
    }
    // SAFETY: registered class and live strings; the window is never shown.
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class,
            w!("Hyaline"),
            0,
            0,
            0,
            0,
            0,
            None,
            None,
            Some(instance),
            None,
        )
    };
    if hwnd.0.is_null() {
        return Err(windows_core::Error::from_thread().into());
    }
    Ok(hwnd)
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == TASKBAR_CREATED.load(Ordering::Relaxed) {
        // Explorer restarted: it dropped our icon and the helper with it.
        ATTEMPT.store(0, Ordering::Relaxed);
        STALE_NOTIFIED.set(false);
        tray::add(hwnd);
        if INJECTED.replace(false) {
            on_explorer_restart(hwnd);
        }
        try_load(hwnd);
        return LRESULT(0);
    }
    if msg == HELPER_READY {
        // Also sent for a new taskbar's helper window (monitor plugged in): attach it too.
        // SAFETY: our own window and timer id; killing a timer that is not set is harmless.
        let _ = unsafe { KillTimer(Some(hwnd), WAIT_TIMER) };
        on_wait(hwnd);
        return LRESULT(0);
    }
    if msg == watcher::RECOMPUTE {
        if let Err(error) = refresh(hwnd, false) {
            eprintln!("hyaline: updating the look failed: {error}");
        }
        return LRESULT(0);
    }
    if msg == tray::MESSAGE {
        if let Some(anchor) = tray::activation(wparam.0, lparam.0) {
            on_menu(hwnd, anchor);
        }
        return LRESULT(0);
    }
    // Generated message constants are i32; real message ids are small positive values.
    match msg.cast_signed() {
        WM_TIMER if wparam.0 == WAIT_TIMER => {
            // SAFETY: our own window and timer id.
            let _ = unsafe { KillTimer(Some(hwnd), WAIT_TIMER) };
            on_wait(hwnd);
            return LRESULT(0);
        }
        WM_TIMER if wparam.0 == RETRY_TIMER => {
            // SAFETY: our own window and timer id.
            let _ = unsafe { KillTimer(Some(hwnd), RETRY_TIMER) };
            try_load(hwnd);
            return LRESULT(0);
        }
        WM_TIMER if REAPPLY_TIMERS.iter().any(|&(id, _)| id == wparam.0) => {
            // SAFETY: our own window and timer id.
            let _ = unsafe { KillTimer(Some(hwnd), wparam.0) };
            if let Err(error) = refresh(hwnd, true) {
                eprintln!("hyaline: re-applying the look failed: {error}");
            }
            return LRESULT(0);
        }
        WM_SETTINGCHANGE if wparam.0 == SPI_SETHIGHCONTRAST.cast_unsigned() as usize => {
            // Leaving high contrast may need the helper loaded again.
            try_load(hwnd);
            schedule_reapply(hwnd);
        }
        WM_POWERBROADCAST
            if wparam.0 == PBT_APMRESUMEAUTOMATIC.cast_unsigned() as usize
                || wparam.0 == PBT_APMRESUMESUSPEND.cast_unsigned() as usize =>
        {
            schedule_reapply(hwnd);
        }
        // Explorer re-applies its own accent on these; ours must land after it.
        WM_DWMCOMPOSITIONCHANGED | WM_THEMECHANGED | WM_SETTINGCHANGE | WM_DISPLAYCHANGE => {
            schedule_reapply(hwnd);
        }
        _ => {}
    }
    // SAFETY: the unmodified message goes to the default procedure.
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
