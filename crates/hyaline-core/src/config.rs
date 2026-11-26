//! The settings file, `config.toml`: a versioned TOML document. Version 1 (one look) is read
//! as the desktop look of version 2 (one look per situation).

use crate::appearance::Appearance;
use crate::states::{Looks, Toggled};
use serde::{Deserialize, Serialize};

/// Format version written to the file.
const VERSION: u32 = 2;

/// Everything Hyaline remembers between runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Config {
    pub looks: Looks,
}

#[derive(Deserialize)]
struct VersionOnly {
    version: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileV1 {
    version: u32,
    appearance: Appearance,
}

/// A situation section: whether it is on, plus its look.
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(deny_unknown_fields)]
struct Section {
    enabled: bool,
    mode: crate::appearance::Mode,
    color: crate::appearance::Rgb,
    opacity: u8,
}

impl From<Toggled> for Section {
    fn from(t: Toggled) -> Self {
        Self {
            enabled: t.enabled,
            mode: t.look.mode,
            color: t.look.color,
            opacity: t.look.opacity,
        }
    }
}

impl From<Section> for Toggled {
    fn from(s: Section) -> Self {
        Self {
            enabled: s.enabled,
            look: Appearance {
                mode: s.mode,
                color: s.color,
                opacity: s.opacity,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileV2 {
    version: u32,
    desktop: Appearance,
    visible_window: Section,
    maximized_window: Section,
    start: Section,
    search: Section,
    task_view: Section,
}

impl Config {
    /// Parses a settings file (version 1 or 2). Any problem (bad TOML, unknown version, unknown
    /// or missing keys, out-of-range values) is an error describing it.
    ///
    /// # Errors
    /// A human-readable description of the first problem found.
    pub fn parse(text: &str) -> Result<Self, String> {
        let message = |e: toml::de::Error| e.message().to_string();
        let version = toml::from_str::<VersionOnly>(text)
            .map_err(message)?
            .version;
        let looks = match version {
            1 => {
                let file: FileV1 = toml::from_str(text).map_err(message)?;
                Looks {
                    desktop: file.appearance,
                    ..Looks::default()
                }
            }
            2 => {
                let file: FileV2 = toml::from_str(text).map_err(message)?;
                Looks {
                    desktop: file.desktop,
                    visible_window: file.visible_window.into(),
                    maximized_window: file.maximized_window.into(),
                    start: file.start.into(),
                    search: file.search.into(),
                    task_view: file.task_view.into(),
                }
            }
            other => return Err(format!("unsupported version {other}, expected {VERSION}")),
        };
        let looks_list = [
            looks.desktop,
            looks.visible_window.look,
            looks.maximized_window.look,
            looks.start.look,
            looks.search.look,
            looks.task_view.look,
        ];
        if let Some(bad) = looks_list.iter().find(|look| look.opacity > 100) {
            return Err(format!("opacity {} is out of range 0-100", bad.opacity));
        }
        Ok(Self { looks })
    }

    /// Writes the settings as a version 2 TOML document.
    #[must_use]
    pub fn to_toml(&self) -> String {
        let looks = &self.looks;
        let file = FileV2 {
            version: VERSION,
            desktop: looks.desktop,
            visible_window: looks.visible_window.into(),
            maximized_window: looks.maximized_window.into(),
            start: looks.start.into(),
            search: looks.search.into(),
            task_view: looks.task_view.into(),
        };
        // Serializing these plain values cannot fail.
        toml::to_string(&file).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::appearance::{Appearance, Mode, Rgb};
    use crate::states::Toggled;

    const DEFAULT_TEXT: &str = r##"
version = 2

[desktop]
mode = "clear"
color = "#000000"
opacity = 0

[visible_window]
enabled = false
mode = "clear"
color = "#000000"
opacity = 0

[maximized_window]
enabled = true
mode = "normal"
color = "#000000"
opacity = 0

[start]
enabled = false
mode = "clear"
color = "#000000"
opacity = 0

[search]
enabled = false
mode = "clear"
color = "#000000"
opacity = 0

[task_view]
enabled = false
mode = "clear"
color = "#000000"
opacity = 0
"##;

    #[test]
    fn documented_default_text_parses_to_defaults() {
        assert_eq!(Config::parse(DEFAULT_TEXT), Ok(Config::default()));
    }

    #[test]
    fn settings_round_trip() {
        let mut config = Config::default();
        config.looks.desktop = Appearance {
            mode: Mode::Acrylic,
            color: Rgb {
                r: 0x12,
                g: 0x34,
                b: 0x56,
            },
            opacity: 40,
        };
        config.looks.start = Toggled {
            enabled: true,
            look: Appearance {
                mode: Mode::Opaque,
                ..Appearance::default()
            },
        };
        assert_eq!(Config::parse(&config.to_toml()), Ok(config));
    }

    #[test]
    fn version_1_files_become_the_desktop_look() {
        let v1 = r##"
version = 1

[appearance]
mode = "opaque"
color = "#FF0000"
opacity = 0
"##;
        let config = Config::parse(v1).unwrap();
        assert_eq!(config.looks.desktop.mode, Mode::Opaque);
        assert_eq!(config.looks.desktop.color, Rgb { r: 255, g: 0, b: 0 });
        assert_eq!(
            config.looks.maximized_window,
            Config::default().looks.maximized_window
        );
        assert!(config.to_toml().contains("version = 2"));
    }

    #[test]
    fn invalid_files_are_rejected() {
        let cases = [
            DEFAULT_TEXT.replace("version = 2", "version = 3"),
            DEFAULT_TEXT.replace("\"normal\"", "\"neon\""),
            DEFAULT_TEXT.replacen("opacity = 0", "opacity = 101", 3),
            DEFAULT_TEXT.replacen("[desktop]\n", "[desktop]\nenabled = true\n", 1),
            DEFAULT_TEXT.replace("[task_view]", "[tasks]"),
            "version = 2".to_string(),
            "version = 1".to_string(),
            "not toml at all".to_string(),
        ];
        for text in cases {
            assert!(Config::parse(&text).is_err(), "{text}");
        }
    }
}
