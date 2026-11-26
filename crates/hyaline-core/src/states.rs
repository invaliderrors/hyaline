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

/// The look picked for each situation; `None` means the situation is turned off and the next
/// one down decides. The desktop look is always on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Looks {
    pub desktop: Appearance,
    pub visible_window: Option<Appearance>,
    pub maximized_window: Option<Appearance>,
    pub start: Option<Appearance>,
    pub search: Option<Appearance>,
    pub task_view: Option<Appearance>,
}

impl Default for Looks {
    fn default() -> Self {
        Self {
            desktop: Appearance::default(),
            visible_window: None,
            maximized_window: Some(Appearance {
                mode: Mode::Normal,
                ..Appearance::default()
            }),
            start: None,
            search: None,
            task_view: None,
        }
    }
}

impl Looks {
    /// The look for a situation (`None` if turned off).
    #[must_use]
    pub const fn get(&self, situation: Situation) -> Option<Appearance> {
        match situation {
            Situation::Start => self.start,
            Situation::Search => self.search,
            Situation::TaskView => self.task_view,
            Situation::MaximizedWindow => self.maximized_window,
            Situation::VisibleWindow => self.visible_window,
            Situation::Desktop => Some(self.desktop),
        }
    }

    /// Sets the look for a situation. The desktop cannot be turned off, so `None` keeps it.
    pub fn set(&mut self, situation: Situation, look: Option<Appearance>) {
        match situation {
            Situation::Start => self.start = look,
            Situation::Search => self.search = look,
            Situation::TaskView => self.task_view = look,
            Situation::MaximizedWindow => self.maximized_window = look,
            Situation::VisibleWindow => self.visible_window = look,
            Situation::Desktop => {
                if let Some(look) = look {
                    self.desktop = look;
                }
            }
        }
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
            .filter(|(_, happening)| *happening)
            .find_map(|(situation, _)| self.get(*situation))
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

    fn all_enabled() -> Looks {
        Looks {
            desktop: look(Mode::Clear),
            visible_window: Some(look(Mode::Opaque)),
            maximized_window: Some(look(Mode::Normal)),
            start: Some(look(Mode::Acrylic)),
            search: Some(look(Mode::Blur)),
            task_view: Some(Appearance {
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
        assert_eq!(looks.resolve(task_view_down), looks.task_view.unwrap());
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
        let looks = Looks {
            maximized_window: None,
            ..all_enabled()
        };
        let maximized = Observed {
            maximized_window: true,
            visible_window: true,
            ..Observed::default()
        };
        assert_eq!(looks.resolve(maximized), look(Mode::Opaque));
        let looks = Looks {
            visible_window: None,
            ..looks
        };
        assert_eq!(looks.resolve(maximized), look(Mode::Clear));
    }

    #[test]
    fn defaults_follow_the_owner_decisions() {
        let looks = Looks::default();
        assert_eq!(looks.desktop, Appearance::default());
        assert_eq!(looks.maximized_window.map(|a| a.mode), Some(Mode::Normal));
        assert_eq!(looks.visible_window, None);
        assert_eq!(looks.start, None);
        assert_eq!(looks.search, None);
        assert_eq!(looks.task_view, None);
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
        looks.set(Situation::Search, Some(look(Mode::Blur)));
        assert_eq!(looks.get(Situation::Search), Some(look(Mode::Blur)));
        looks.set(Situation::Desktop, None);
        assert_eq!(
            looks.get(Situation::Desktop),
            Some(Appearance::default()),
            "desktop cannot be turned off"
        );
    }
}
