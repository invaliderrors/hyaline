//! Situations on a monitor, the look the user picked for each, and which one wins.

use crate::appearance::{Appearance, Mode};

/// Something happening on a monitor that can change the taskbar's look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Situation {
    Start,
    Search,
    TaskView,
    MaximizedWindow,
    VisibleWindow,
    Desktop,
}

impl Situation {
    /// Every situation, highest priority first.
    pub const ALL: [Self; 6] = [
        Self::Start,
        Self::Search,
        Self::TaskView,
        Self::MaximizedWindow,
        Self::VisibleWindow,
        Self::Desktop,
    ];
}

/// What is currently true on one monitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::struct_excessive_bools)] // independent facts, one per situation
pub struct Observed {
    pub start: bool,
    pub search: bool,
    pub task_view: bool,
    pub maximized_window: bool,
    pub visible_window: bool,
}

/// A situation's look and whether the situation is turned on. Turning it off keeps the look
/// for when it is turned on again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Toggled {
    pub enabled: bool,
    pub look: Appearance,
}

impl Toggled {
    const fn off() -> Self {
        Self {
            enabled: false,
            look: Appearance {
                mode: Mode::Clear,
                color: crate::appearance::Rgb { r: 0, g: 0, b: 0 },
                opacity: 0,
            },
        }
    }
}

/// The look picked for each situation. A situation that is turned off lets the next one down
/// decide; the desktop look is always on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Looks {
    pub desktop: Appearance,
    pub visible_window: Toggled,
    pub maximized_window: Toggled,
    pub start: Toggled,
    pub search: Toggled,
    pub task_view: Toggled,
}

impl Default for Looks {
    fn default() -> Self {
        Self {
            desktop: Appearance::default(),
            visible_window: Toggled::off(),
            maximized_window: Toggled {
                enabled: true,
                look: Appearance {
                    mode: Mode::Normal,
                    ..Appearance::default()
                },
            },
            start: Toggled::off(),
            search: Toggled::off(),
            task_view: Toggled::off(),
        }
    }
}

impl Looks {
    /// A situation's setting; the desktop is always reported as on.
    #[must_use]
    pub const fn toggled(&self, situation: Situation) -> Toggled {
        match situation {
            Situation::Start => self.start,
            Situation::Search => self.search,
            Situation::TaskView => self.task_view,
            Situation::MaximizedWindow => self.maximized_window,
            Situation::VisibleWindow => self.visible_window,
            Situation::Desktop => Toggled {
                enabled: true,
                look: self.desktop,
            },
        }
    }

    /// Changes a situation's setting. The desktop cannot be turned off; only its look changes.
    pub fn set(&mut self, situation: Situation, toggled: Toggled) {
        match situation {
            Situation::Start => self.start = toggled,
            Situation::Search => self.search = toggled,
            Situation::TaskView => self.task_view = toggled,
            Situation::MaximizedWindow => self.maximized_window = toggled,
            Situation::VisibleWindow => self.visible_window = toggled,
            Situation::Desktop => self.desktop = toggled.look,
        }
    }

    /// True if any look in use needs the Explorer helper (anything but Windows' own look).
    #[must_use]
    pub fn needs_helper(&self) -> bool {
        Situation::ALL
            .iter()
            .map(|situation| self.toggled(*situation))
            .any(|toggled| toggled.enabled && toggled.look.mode != Mode::Normal)
    }

    /// The look of the highest-priority situation that is both happening and turned on.
    #[must_use]
    pub fn resolve(&self, observed: Observed) -> Appearance {
        let happening = [
            observed.start,
            observed.search,
            observed.task_view,
            observed.maximized_window,
            observed.visible_window,
            true,
        ];
        Situation::ALL
            .iter()
            .zip(happening)
            .map(|(situation, happening)| (self.toggled(*situation), happening))
            .find_map(|(toggled, happening)| (happening && toggled.enabled).then_some(toggled.look))
            .unwrap_or(self.desktop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::appearance::{Appearance, Mode, Rgb};

    fn look(mode: Mode) -> Appearance {
        Appearance {
            mode,
            color: Rgb { r: 1, g: 2, b: 3 },
            opacity: 40,
        }
    }

    fn on(look: Appearance) -> Toggled {
        Toggled {
            enabled: true,
            look,
        }
    }

    fn all_enabled() -> Looks {
        Looks {
            desktop: look(Mode::Clear),
            visible_window: on(look(Mode::Opaque)),
            maximized_window: on(look(Mode::Normal)),
            start: on(look(Mode::Acrylic)),
            search: on(look(Mode::Blur)),
            task_view: on(Appearance {
                opacity: 90,
                ..look(Mode::Opaque)
            }),
        }
    }

    #[test]
    fn nothing_observed_is_desktop() {
        assert_eq!(
            all_enabled().resolve(Observed::default()),
            look(Mode::Clear)
        );
    }

    #[test]
    fn higher_priority_situations_win() {
        let looks = all_enabled();
        let everything = Observed {
            start: true,
            search: true,
            task_view: true,
            maximized_window: true,
            visible_window: true,
        };
        assert_eq!(looks.resolve(everything), look(Mode::Acrylic));
        let no_start = Observed {
            start: false,
            ..everything
        };
        assert_eq!(looks.resolve(no_start), look(Mode::Blur));
        let task_view_down = Observed {
            start: false,
            search: false,
            ..everything
        };
        assert_eq!(looks.resolve(task_view_down), looks.task_view.look);
        let windows = Observed {
            maximized_window: true,
            visible_window: true,
            ..Observed::default()
        };
        assert_eq!(looks.resolve(windows), look(Mode::Normal));
        let visible = Observed {
            visible_window: true,
            ..Observed::default()
        };
        assert_eq!(looks.resolve(visible), look(Mode::Opaque));
    }

    #[test]
    fn disabled_situation_falls_through() {
        let mut looks = all_enabled();
        looks.maximized_window.enabled = false;
        let maximized = Observed {
            maximized_window: true,
            visible_window: true,
            ..Observed::default()
        };
        assert_eq!(looks.resolve(maximized), look(Mode::Opaque));
        looks.visible_window.enabled = false;
        assert_eq!(looks.resolve(maximized), look(Mode::Clear));
    }

    #[test]
    fn helper_is_needed_only_for_enabled_looks_other_than_normal() {
        let normal = look(Mode::Normal);
        let mut looks = Looks {
            desktop: normal,
            visible_window: on(normal),
            maximized_window: on(normal),
            start: Toggled {
                enabled: false,
                look: look(Mode::Clear),
            },
            search: on(normal),
            task_view: on(normal),
        };
        assert!(!looks.needs_helper());
        looks.start.enabled = true;
        assert!(looks.needs_helper());
        looks.start.enabled = false;
        looks.desktop = look(Mode::Opaque);
        assert!(looks.needs_helper());
    }

    #[test]
    fn turning_off_keeps_the_look_for_later() {
        let mut looks = all_enabled();
        looks.start.enabled = false;
        assert_eq!(looks.start.look, look(Mode::Acrylic));
    }

    #[test]
    fn defaults_follow_the_owner_decisions() {
        let looks = Looks::default();
        assert_eq!(looks.desktop, Appearance::default());
        assert!(looks.maximized_window.enabled);
        assert_eq!(looks.maximized_window.look.mode, Mode::Normal);
        for off in [
            looks.visible_window,
            looks.start,
            looks.search,
            looks.task_view,
        ] {
            assert!(!off.enabled);
        }
    }

    #[test]
    fn situations_are_listed_in_priority_order_and_read_back() {
        assert_eq!(
            Situation::ALL,
            [
                Situation::Start,
                Situation::Search,
                Situation::TaskView,
                Situation::MaximizedWindow,
                Situation::VisibleWindow,
                Situation::Desktop,
            ]
        );
        let mut looks = Looks::default();
        looks.set(Situation::Search, on(look(Mode::Blur)));
        assert_eq!(looks.toggled(Situation::Search), on(look(Mode::Blur)));
        looks.set(
            Situation::Desktop,
            Toggled {
                enabled: false,
                look: look(Mode::Opaque),
            },
        );
        assert_eq!(
            looks.toggled(Situation::Desktop),
            on(look(Mode::Opaque)),
            "desktop cannot be turned off"
        );
    }
}
