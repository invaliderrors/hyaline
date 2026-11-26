//! Taskbar appearances and their encoding as a Win32 accent policy.

/// `WINDOWCOMPOSITIONATTRIB::WCA_ACCENT_POLICY` for `SetWindowCompositionAttribute`.
pub const WCA_ACCENT_POLICY: u32 = 19;

/// What the taskbar looks like.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Appearance {
    /// Fully transparent: the wallpaper shows through.
    Clear,
}

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

/// `ACCENT_ENABLE_TRANSPARENTGRADIENT`.
const STATE_TRANSPARENT_GRADIENT: u32 = 2;
/// Use `gradient_color` instead of the system accent color.
const FLAG_USE_GRADIENT_COLOR: u32 = 2;

impl Appearance {
    /// The accent policy that produces this appearance.
    #[must_use]
    pub const fn accent(self) -> AccentPolicy {
        match self {
            Self::Clear => AccentPolicy {
                state: STATE_TRANSPARENT_GRADIENT,
                flags: FLAG_USE_GRADIENT_COLOR,
                gradient_color: 0,
                animation_id: 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_is_transparent_gradient_with_given_color() {
        assert_eq!(
            Appearance::Clear.accent(),
            AccentPolicy {
                state: 2,
                flags: 2,
                gradient_color: 0,
                animation_id: 0,
            }
        );
        assert_eq!(size_of::<AccentPolicy>(), 16);
    }
}
