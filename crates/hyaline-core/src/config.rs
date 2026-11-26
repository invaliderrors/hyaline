//! The settings file, `config.toml`: a versioned TOML document.

use crate::appearance::Appearance;
use serde::{Deserialize, Serialize};

/// Format version written to and required in the file.
const VERSION: u32 = 1;

/// Everything Hyaline remembers between runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Config {
    pub appearance: Appearance,
}

/// On-disk layout.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    version: u32,
    appearance: Appearance,
}

impl Config {
    /// Parses a settings file. Any problem (bad TOML, unknown version, unknown keys,
    /// out-of-range values) is an error describing it.
    ///
    /// # Errors
    /// A human-readable description of the first problem found.
    pub fn parse(text: &str) -> Result<Self, String> {
        let file: File = toml::from_str(text).map_err(|e| e.message().to_string())?;
        if file.version != VERSION {
            return Err(format!(
                "unsupported version {}, expected {VERSION}",
                file.version
            ));
        }
        if file.appearance.opacity > 100 {
            return Err(format!(
                "opacity {} is out of range 0-100",
                file.appearance.opacity
            ));
        }
        Ok(Self {
            appearance: file.appearance,
        })
    }

    /// Writes the settings as a TOML document.
    #[must_use]
    pub fn to_toml(&self) -> String {
        let file = File {
            version: VERSION,
            appearance: self.appearance,
        };
        // Serializing these plain values cannot fail.
        toml::to_string(&file).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::appearance::{Appearance, Mode, Rgb};

    const DEFAULT_TEXT: &str = r##"
version = 1

[appearance]
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
        let config = Config {
            appearance: Appearance {
                mode: Mode::Acrylic,
                color: Rgb {
                    r: 0x12,
                    g: 0x34,
                    b: 0x56,
                },
                opacity: 40,
            },
        };
        assert_eq!(Config::parse(&config.to_toml()), Ok(config));
    }

    #[test]
    fn invalid_files_are_rejected() {
        let cases = [
            DEFAULT_TEXT.replace("version = 1", "version = 2"),
            DEFAULT_TEXT.replace("\"clear\"", "\"neon\""),
            DEFAULT_TEXT.replace("\"#000000\"", "\"red\""),
            DEFAULT_TEXT.replace("opacity = 0", "opacity = 101"),
            DEFAULT_TEXT.replace(
                "opacity = 0",
                "opacity = 0
speed = 1",
            ),
            "version = 1".to_string(),
            "not toml at all".to_string(),
        ];
        for text in cases {
            assert!(Config::parse(&text).is_err(), "{text}");
        }
    }
}
