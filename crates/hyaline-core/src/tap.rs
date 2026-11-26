//! Protocol between `hyaline.exe` and the helper DLL loaded into Explorer, and the rules
//! that pick which taskbar XAML elements the helper changes.

/// Window class of the helper's message-only window inside Explorer.
pub const WINDOW_CLASS: &str = "HyalineTapWindow";

/// Window class of `hyaline.exe`'s hidden top-level window.
pub const HOST_WINDOW_CLASS: &str = "HyalineHostWindow";

const WM_APP: u32 = 0x8000;
const MSG_CLEAR: u32 = WM_APP + 1;
const MSG_RESTORE: u32 = WM_APP + 2;
const MSG_ATTACH: u32 = WM_APP + 3;
const MSG_VERSION: u32 = WM_APP + 4;

/// What the helper answers to [`Command::Version`]. Bump it whenever a command changes meaning:
/// Explorer keeps a loaded helper until it restarts, so a newer `hyaline.exe` can meet an older
/// helper (which answers 0, like any window answering an unknown message).
pub const PROTOCOL_VERSION: usize = 1;

/// Posted by the helper to the host window when one of its windows is created (helper loaded,
/// or a new taskbar thread such as a plugged-in monitor's), so the host applies the look
/// right away instead of on its next check.
pub const HELPER_READY: u32 = WM_APP + 16;

/// A request from `hyaline.exe` to the helper, sent as a window message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Make the taskbar background transparent.
    Clear,
    /// Put the original taskbar background back.
    Restore,
    /// A new host process took over; restore automatically when it exits.
    Attach { host_pid: u32 },
    /// Asks for the helper's [`PROTOCOL_VERSION`] (the reply value, not a success code).
    Version,
}

impl Command {
    /// Encodes the command as `(message, wparam)`.
    #[must_use]
    pub const fn to_message(self) -> (u32, usize) {
        match self {
            Self::Clear => (MSG_CLEAR, 0),
            Self::Restore => (MSG_RESTORE, 0),
            Self::Attach { host_pid } => (MSG_ATTACH, host_pid as usize),
            Self::Version => (MSG_VERSION, 0),
        }
    }

    /// Decodes a window message; `None` for anything that is not a valid command.
    #[must_use]
    pub const fn from_message(msg: u32, wparam: usize) -> Option<Self> {
        match msg {
            MSG_CLEAR => Some(Self::Clear),
            MSG_RESTORE => Some(Self::Restore),
            MSG_ATTACH if wparam <= u32::MAX as usize => Some(Self::Attach {
                #[allow(clippy::cast_possible_truncation)] // checked against u32::MAX above
                host_pid: wparam as u32,
            }),
            MSG_VERSION => Some(Self::Version),
            _ => None,
        }
    }
}

/// Data passed to the helper when it is first loaded into Explorer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitData {
    /// Process id of `hyaline.exe`; the helper restores the taskbar when it exits.
    pub host_pid: u32,
    /// Whether the background starts cleared.
    pub cleared: bool,
}

impl InitData {
    /// Encodes as `v1;host=<pid>;cleared=<0|1>`.
    #[must_use]
    pub fn encode(&self) -> String {
        format!(
            "v1;host={};cleared={}",
            self.host_pid,
            u8::from(self.cleared)
        )
    }

    /// Decodes [`InitData::encode`] output; `None` for any other input.
    #[must_use]
    pub fn decode(s: &str) -> Option<Self> {
        let mut parts = s.split(';');
        if parts.next()? != "v1" {
            return None;
        }
        let host_pid = parts.next()?.strip_prefix("host=")?.parse().ok()?;
        let cleared = match parts.next()?.strip_prefix("cleared=")? {
            "0" => false,
            "1" => true,
            _ => return None,
        };
        parts.next().is_none().then_some(Self { host_pid, cleared })
    }
}

/// True for the main taskbar's background rectangles. Their grandparent, given as
/// `(type, name)`, is the `Taskbar.TaskbarBackground` named `BackgroundControl`; the thumbnail
/// flyout has look-alikes that must stay untouched.
#[must_use]
pub fn is_taskbar_background(name: &str, grandparent: Option<(&str, &str)>) -> bool {
    matches!(name, "BackgroundFill" | "BackgroundStroke")
        && grandparent == Some(("Taskbar.TaskbarBackground", "BackgroundControl"))
}

/// True for the window classes of the primary and secondary-monitor taskbars.
#[must_use]
pub fn is_taskbar_window_class(class: &str) -> bool {
    matches!(class, "Shell_TrayWnd" | "Shell_SecondaryTrayWnd")
}

/// Name of the event the helper creates as soon as it is loaded into an Explorer process,
/// so the host never loads it twice into the same process.
#[must_use]
pub fn marker_name(explorer_pid: u32) -> String {
    format!(r"Local\HyalineTap-{explorer_pid}")
}

/// Delay before retry number `attempt` (0-based) of loading the helper, or `None` to give up.
///
/// Explorer's taskbar and XAML runtime come up seconds after Explorer starts (about 11 s
/// measured after a restart), so loading is retried every 500 ms for one minute. The retries
/// only run in that window, never while idle.
#[must_use]
pub const fn retry_delay_ms(attempt: u32) -> Option<u32> {
    if attempt < 120 { Some(500) } else { None }
}

/// Explorer restarts this close together, while the helper was loaded, look like a crash loop.
const CRASH_LOOP_RESTARTS: usize = 3;
const CRASH_LOOP_WINDOW_MS: u64 = 120_000;

/// Stops loading the helper when Explorer keeps restarting with it loaded: the helper may be
/// the cause, and Windows' own look is always safe.
#[derive(Debug, Default, Clone)]
pub struct RestartGuard {
    recent: Vec<u64>,
    paused: bool,
}

impl RestartGuard {
    /// Records an Explorer restart at `now_ms` (any monotonic clock); returns [`Self::paused`].
    pub fn record(&mut self, now_ms: u64) -> bool {
        self.recent
            .retain(|&at| now_ms.saturating_sub(at) <= CRASH_LOOP_WINDOW_MS);
        self.recent.push(now_ms);
        self.paused |= self.recent.len() >= CRASH_LOOP_RESTARTS;
        self.paused
    }

    /// Whether loading the helper is paused.
    #[must_use]
    pub const fn paused(&self) -> bool {
        self.paused
    }

    /// Resumes and forgets past restarts (the owner asked for a look again).
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Name of the mutex that keeps `hyaline.exe` to a single instance per session.
pub const INSTANCE_MUTEX: &str = r"Local\HyalineInstance";

/// XAML property the helper sets to 0 to hide a background element. Its value is never
/// set by Explorer, so clearing our local value restores the original exactly.
pub const OPACITY: &str = "Opacity";

/// Finds a property's index in a XAML property chain of `(name, index)` pairs.
#[must_use]
pub fn property_index<'a>(
    props: impl IntoIterator<Item = (&'a str, u32)>,
    property: &str,
) -> Option<u32> {
    props
        .into_iter()
        .find_map(|(name, index)| (name == property).then_some(index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_round_trip_through_window_messages() {
        for c in [
            Command::Clear,
            Command::Restore,
            Command::Attach { host_pid: 4242 },
            Command::Version,
        ] {
            let (msg, wparam) = c.to_message();
            assert_eq!(Command::from_message(msg, wparam), Some(c));
        }
        assert_eq!(Command::Clear.to_message(), (0x8001, 0));
        assert_eq!(Command::Attach { host_pid: 7 }.to_message(), (0x8003, 7));
    }

    #[test]
    fn unknown_message_is_not_a_command() {
        assert_eq!(Command::from_message(0x0010, 0), None); // WM_CLOSE
        assert_eq!(Command::from_message(0x8005, 0), None);
    }

    #[test]
    fn version_query_is_a_command() {
        assert_eq!(Command::Version.to_message(), (0x8004, 0));
        assert_eq!(Command::from_message(0x8004, 0), Some(Command::Version));
        assert_eq!(PROTOCOL_VERSION, 1);
    }

    #[test]
    fn two_explorer_restarts_do_not_pause() {
        let mut guard = RestartGuard::default();
        assert!(!guard.record(1_000));
        assert!(!guard.record(2_000));
        assert!(!guard.paused());
    }

    #[test]
    fn third_restart_within_two_minutes_pauses() {
        let mut guard = RestartGuard::default();
        guard.record(0);
        guard.record(60_000);
        assert!(guard.record(120_000));
        assert!(guard.paused());
    }

    #[test]
    fn restarts_spread_over_more_than_two_minutes_do_not_pause() {
        let mut guard = RestartGuard::default();
        for now in [0, 70_000, 140_000, 210_000, 280_000] {
            assert!(!guard.record(now), "{now}");
        }
    }

    #[test]
    fn pause_lasts_until_reset() {
        let mut guard = RestartGuard::default();
        for now in [0, 1, 2] {
            guard.record(now);
        }
        assert!(guard.record(10_000_000));
        guard.reset();
        assert!(!guard.paused());
        assert!(!guard.record(10_000_001));
        assert!(!guard.record(10_000_002));
    }

    #[test]
    fn attach_with_pid_beyond_u32_is_rejected() {
        assert_eq!(Command::from_message(0x8003, usize::MAX), None);
    }

    #[test]
    fn init_data_round_trips() {
        let d = InitData {
            host_pid: 1234,
            cleared: true,
        };
        assert_eq!(d.encode(), "v1;host=1234;cleared=1");
        assert_eq!(InitData::decode(&d.encode()), Some(d));
    }

    #[test]
    fn init_data_rejects_malformed_input() {
        for bad in [
            "",
            "v2;host=1;cleared=1",
            "v1;host=x;cleared=1",
            "v1;host=1;cleared=2",
            "v1;host=1",
        ] {
            assert_eq!(InitData::decode(bad), None, "{bad}");
        }
    }

    #[test]
    fn only_main_taskbar_background_is_targeted() {
        const CONTROL: (&str, &str) = ("Taskbar.TaskbarBackground", "BackgroundControl");
        assert!(is_taskbar_background("BackgroundFill", Some(CONTROL)));
        assert!(is_taskbar_background("BackgroundStroke", Some(CONTROL)));
        assert!(!is_taskbar_background(
            "BackgroundFill",
            Some(("Taskbar.TaskbarBackground", "HoverFlyoutBackgroundControl"))
        ));
        assert!(!is_taskbar_background(
            "BackgroundFill",
            Some(("Some.Other.Control", "BackgroundControl"))
        ));
        assert!(!is_taskbar_background("BackgroundFill", None));
        assert!(!is_taskbar_background("BackgroundElement", Some(CONTROL)));
    }

    #[test]
    fn helper_load_is_retried_for_a_minute_then_given_up() {
        assert_eq!(retry_delay_ms(0), Some(500));
        assert_eq!(retry_delay_ms(119), Some(500));
        assert_eq!(retry_delay_ms(120), None);
    }

    #[test]
    fn primary_and_secondary_taskbars_are_taskbar_windows() {
        assert!(is_taskbar_window_class("Shell_TrayWnd"));
        assert!(is_taskbar_window_class("Shell_SecondaryTrayWnd"));
        assert!(!is_taskbar_window_class("Shell_TrayWndX"));
        assert!(!is_taskbar_window_class("TrayNotifyWnd"));
    }

    #[test]
    fn marker_name_is_per_explorer_process() {
        assert_eq!(marker_name(4321), r"Local\HyalineTap-4321");
    }

    #[test]
    fn property_index_is_first_matching_entry() {
        let chain = [
            ("Opacity", 453),
            ("Fill", 1106),
            ("Opacity", 999),
            ("Stroke", 1109),
        ];
        assert_eq!(property_index(chain, OPACITY), Some(453));
        assert_eq!(property_index(chain, "Fill"), Some(1106));
        assert_eq!(property_index([("Fill", 1106)], OPACITY), None);
    }
}
