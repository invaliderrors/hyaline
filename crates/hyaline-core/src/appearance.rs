//! Taskbar appearances and their encoding as a Win32 accent policy.

/// `WINDOWCOMPOSITIONATTRIB::WCA_ACCENT_POLICY` for `SetWindowCompositionAttribute`.
pub const WCA_ACCENT_POLICY: u32 = 19;

/// Win32 `ACCENT_POLICY` (undocumented; layout from the Windows composition API).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AccentPolicy {
    pub state: u32,
    pub flags: u32,
    /// `0xAABBGGRR`.
    pub gradient_color: u32,
    pub animation_id: u32,
}

/// How the taskbar is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Windows' own look; Hyaline changes nothing.
    Normal,
    /// Solid color.
    Opaque,
    /// See-through, optionally tinted by color and opacity.
    Clear,
    /// Blurred background, tinted. Needs "Transparency effects" on.
    Blur,
    /// Acrylic material, tinted. Needs "Transparency effects" on.
    Acrylic,
}

/// A color, written `#RRGGBB`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Parses `#RRGGBB` (hex digits in any case).
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let hex = text.strip_prefix('#')?;
        if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        let channel = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).ok();
        Some(Self {
            r: channel(0)?,
            g: channel(2)?,
            b: channel(4)?,
        })
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl TryFrom<String> for Rgb {
    type Error = String;
    fn try_from(text: String) -> Result<Self, String> {
        Self::parse(&text).ok_or_else(|| format!("invalid color {text:?}, expected \"#RRGGBB\""))
    }
}

impl From<Rgb> for String {
    fn from(rgb: Rgb) -> Self {
        rgb.to_string()
    }
}

/// The complete taskbar look the user picked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Appearance {
    pub mode: Mode,
    pub color: Rgb,
    /// Tint strength in percent, 0–100. Ignored by `Normal` and `Opaque`.
    pub opacity: u8,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            mode: Mode::Clear,
            color: Rgb { r: 0, g: 0, b: 0 },
            opacity: 0,
        }
    }
}

/// Use `gradient_color` instead of the system accent color.
const FLAG_USE_GRADIENT_COLOR: u32 = 2;

impl Appearance {
    /// The accent policy that produces this look, or `None` for Windows' own look.
    #[must_use]
    pub fn accent(&self) -> Option<AccentPolicy> {
        let alpha = (u32::from(self.opacity.min(100)) * 255 + 50) / 100;
        let (state, alpha) = match self.mode {
            Mode::Normal => return None,
            Mode::Opaque => (1, 255),
            Mode::Clear => (2, alpha),
            Mode::Blur => (3, alpha),
            // Acrylic with a fully transparent tint renders incorrectly.
            Mode::Acrylic => (4, alpha.max(1)),
        };
        let Rgb { r, g, b } = self.color;
        Some(AccentPolicy {
            state,
            flags: FLAG_USE_GRADIENT_COLOR,
            gradient_color: alpha << 24 | u32::from(b) << 16 | u32::from(g) << 8 | u32::from(r),
            animation_id: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLOR: Rgb = Rgb {
        r: 0x11,
        g: 0x22,
        b: 0x33,
    };

    fn look(mode: Mode, opacity: u8) -> Appearance {
        Appearance {
            mode,
            color: COLOR,
            opacity,
        }
    }

    fn accent(state: u32, gradient_color: u32) -> AccentPolicy {
        AccentPolicy {
            state,
            flags: 2,
            gradient_color,
            animation_id: 0,
        }
    }

    #[test]
    fn normal_has_no_accent() {
        assert_eq!(look(Mode::Normal, 50).accent(), None);
    }

    #[test]
    fn clear_uses_opacity_as_alpha() {
        assert_eq!(look(Mode::Clear, 0).accent(), Some(accent(2, 0x0033_2211)));
        assert_eq!(look(Mode::Clear, 50).accent(), Some(accent(2, 0x8033_2211)));
        assert_eq!(
            look(Mode::Clear, 100).accent(),
            Some(accent(2, 0xFF33_2211))
        );
    }

    #[test]
    fn opaque_ignores_opacity() {
        assert_eq!(look(Mode::Opaque, 0).accent(), Some(accent(1, 0xFF33_2211)));
    }

    #[test]
    fn blur_and_acrylic_use_their_states() {
        assert_eq!(look(Mode::Blur, 25).accent(), Some(accent(3, 0x4033_2211)));
        assert_eq!(
            look(Mode::Acrylic, 75).accent(),
            Some(accent(4, 0xBF33_2211))
        );
    }

    #[test]
    fn acrylic_alpha_is_never_zero() {
        assert_eq!(
            look(Mode::Acrylic, 0).accent(),
            Some(accent(4, 0x0133_2211))
        );
    }

    #[test]
    fn default_is_fully_clear() {
        assert_eq!(
            Appearance::default(),
            Appearance {
                mode: Mode::Clear,
                color: Rgb { r: 0, g: 0, b: 0 },
                opacity: 0,
            }
        );
    }

    #[test]
    fn rgb_parses_hex_and_round_trips() {
        let rgb = Rgb::parse("#1A2b3C").unwrap();
        assert_eq!(
            rgb,
            Rgb {
                r: 0x1A,
                g: 0x2B,
                b: 0x3C
            }
        );
        assert_eq!(rgb.to_string(), "#1A2B3C");
    }

    #[test]
    fn rgb_rejects_malformed_text() {
        for bad in ["", "1A2B3C", "#1A2B3", "#GG0000", "#1A2B3C4", "#+1A2B3"] {
            assert_eq!(Rgb::parse(bad), None, "{bad}");
        }
    }

    #[test]
    fn accent_policy_matches_win32_size() {
        assert_eq!(size_of::<AccentPolicy>(), 16);
    }
}
