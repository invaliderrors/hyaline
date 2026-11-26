//! Protocol between `hyaline.exe` and the helper DLL loaded into Explorer, and the rules
//! that pick which taskbar XAML elements the helper changes.

/// Window class of the helper's message-only window inside Explorer.
pub const WINDOW_CLASS: &str = "HyalineTapWindow";

const WM_APP: u32 = 0x8000;
const MSG_CLEAR: u32 = WM_APP + 1;
const MSG_RESTORE: u32 = WM_APP + 2;
const MSG_ATTACH: u32 = WM_APP + 3;

/// A request from `hyaline.exe` to the helper, sent as a window message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Make the taskbar background transparent.
    Clear,
    /// Put the original taskbar background back.
    Restore,
    /// A new host process took over; restore automatically when it exits.
    Attach { host_pid: u32 },
}

impl Command {
    /// Encodes the command as `(message, wparam)`.
    #[must_use]
    pub const fn to_message(self) -> (u32, usize) {
        match self {
            Self::Clear => (MSG_CLEAR, 0),
            Self::Restore => (MSG_RESTORE, 0),
            Self::Attach { host_pid } => (MSG_ATTACH, host_pid as usize),
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

/// True for the main taskbar's background rectangles. Their grandparent is the
/// `BackgroundControl`; the thumbnail flyout has look-alikes that must stay untouched.
#[must_use]
pub fn is_taskbar_background(name: &str, grandparent_name: Option<&str>) -> bool {
    matches!(name, "BackgroundFill" | "BackgroundStroke")
        && grandparent_name == Some("BackgroundControl")
}

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
        assert_eq!(Command::from_message(0x8004, 0), None);
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
        assert!(is_taskbar_background(
            "BackgroundFill",
            Some("BackgroundControl")
        ));
        assert!(is_taskbar_background(
            "BackgroundStroke",
            Some("BackgroundControl")
        ));
        assert!(!is_taskbar_background(
            "BackgroundFill",
            Some("HoverFlyoutBackgroundControl")
        ));
        assert!(!is_taskbar_background("BackgroundFill", None));
        assert!(!is_taskbar_background(
            "BackgroundElement",
            Some("BackgroundControl")
        ));
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
