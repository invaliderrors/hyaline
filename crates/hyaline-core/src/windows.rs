//! Which top-level windows count as "a window on this monitor" for the dynamic looks.

/// Facts about one top-level window, read from Win32 by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::struct_excessive_bools)] // independent Win32 facts
pub struct WindowFacts {
    pub visible: bool,
    /// Hidden by DWM, e.g. on another virtual desktop.
    pub cloaked: bool,
    pub minimized: bool,
    pub maximized: bool,
    /// Covers the monitor's work area without being "maximized" (borderless apps).
    pub fills_work_area: bool,
    pub tool_window: bool,
    /// Has an owner window (dialogs, popups); its owner already counts.
    pub owned: bool,
    pub zero_size: bool,
    /// Desktop or taskbar windows.
    pub shell: bool,
}

/// How a window affects its monitor's taskbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowKind {
    Maximized,
    Visible,
}

/// Whether and how a window counts.
#[must_use]
pub const fn classify(facts: WindowFacts) -> Option<WindowKind> {
    if !facts.visible
        || facts.cloaked
        || facts.minimized
        || facts.tool_window
        || facts.owned
        || facts.zero_size
        || facts.shell
    {
        return None;
    }
    if facts.maximized || facts.fills_work_area {
        Some(WindowKind::Maximized)
    } else {
        Some(WindowKind::Visible)
    }
}

/// Shell surfaces detected from the foreground window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Foreground {
    pub start: bool,
    pub search: bool,
    pub task_view: bool,
}

/// Classifies the foreground window by class and process name (Windows 11, measured in
/// `docs/research/win11-taskbar.md` §7). `launcher_visible` comes from `IAppVisibility`; Start
/// is hosted by the same `SearchHost.exe` window as Search, so the flag tells them apart.
/// `alt_tab` is true between `EVENT_SYSTEM_SWITCHSTART` and `SWITCHEND`: the Alt+Tab
/// switcher uses the same window class as Task View.
#[must_use]
pub fn foreground(class: &str, exe: &str, launcher_visible: bool, alt_tab: bool) -> Foreground {
    if launcher_visible {
        return Foreground {
            start: true,
            ..Foreground::default()
        };
    }
    let exe = exe.to_ascii_lowercase();
    Foreground {
        start: false,
        search: class == "Windows.UI.Core.CoreWindow" && exe == "searchhost.exe",
        task_view: !alt_tab && class == "XamlExplorerHostIslandWindow" && exe == "explorer.exe",
    }
}

/// Window classes of the shell, which never count as windows: the desktop, the taskbars, and
/// system surfaces (Start, Search, flyouts, emoji panel, touch keyboard, Task View, Alt+Tab).
/// Store apps are hosted in `ApplicationFrameWindow`, so a top-level `CoreWindow` is the shell.
#[must_use]
pub fn is_shell_class(class: &str) -> bool {
    matches!(
        class,
        "Progman"
            | "WorkerW"
            | "Shell_TrayWnd"
            | "Shell_SecondaryTrayWnd"
            | "Windows.UI.Core.CoreWindow"
            | "XamlExplorerHostIslandWindow"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normal_window() -> WindowFacts {
        WindowFacts {
            visible: true,
            ..WindowFacts::default()
        }
    }

    #[test]
    fn plain_visible_window_counts_as_visible() {
        assert_eq!(classify(normal_window()), Some(WindowKind::Visible));
    }

    #[test]
    fn maximized_or_work_area_filling_window_counts_as_maximized() {
        let maximized = WindowFacts {
            maximized: true,
            ..normal_window()
        };
        assert_eq!(classify(maximized), Some(WindowKind::Maximized));
        let filling = WindowFacts {
            fills_work_area: true,
            ..normal_window()
        };
        assert_eq!(classify(filling), Some(WindowKind::Maximized));
    }

    #[test]
    fn hidden_cloaked_minimized_tool_owned_empty_and_shell_windows_do_not_count() {
        let exclusions = [
            WindowFacts {
                visible: false,
                ..normal_window()
            },
            WindowFacts {
                cloaked: true,
                ..normal_window()
            },
            WindowFacts {
                minimized: true,
                maximized: true,
                ..normal_window()
            },
            WindowFacts {
                tool_window: true,
                ..normal_window()
            },
            WindowFacts {
                owned: true,
                ..normal_window()
            },
            WindowFacts {
                zero_size: true,
                ..normal_window()
            },
            WindowFacts {
                shell: true,
                maximized: true,
                ..normal_window()
            },
        ];
        for facts in exclusions {
            assert_eq!(classify(facts), None, "{facts:?}");
        }
    }

    #[test]
    fn foreground_search_host_is_start_or_search() {
        let host = ("Windows.UI.Core.CoreWindow", "SearchHost.exe");
        assert_eq!(
            foreground(host.0, host.1, true, false),
            Foreground {
                start: true,
                ..Foreground::default()
            }
        );
        assert_eq!(
            foreground(host.0, host.1, false, false),
            Foreground {
                search: true,
                ..Foreground::default()
            }
        );
    }

    #[test]
    fn foreground_task_view_is_recognized() {
        assert_eq!(
            foreground("XamlExplorerHostIslandWindow", "explorer.exe", false, false),
            Foreground {
                task_view: true,
                ..Foreground::default()
            }
        );
        assert_eq!(
            foreground("XamlExplorerHostIslandWindow", "other.exe", false, false),
            Foreground::default()
        );
    }

    #[test]
    fn alt_tab_is_not_task_view() {
        assert_eq!(
            foreground("XamlExplorerHostIslandWindow", "explorer.exe", false, true),
            Foreground::default()
        );
    }

    #[test]
    fn launcher_flag_alone_means_start() {
        assert_eq!(
            foreground("Chrome_WidgetWin_1", "chrome.exe", true, false),
            Foreground {
                start: true,
                ..Foreground::default()
            }
        );
        assert_eq!(
            foreground("Chrome_WidgetWin_1", "chrome.exe", false, false),
            Foreground::default()
        );
    }

    #[test]
    fn shell_classes_are_recognized() {
        for class in [
            "Progman",
            "WorkerW",
            "Shell_TrayWnd",
            "Shell_SecondaryTrayWnd",
        ] {
            assert!(is_shell_class(class), "{class}");
        }
        assert!(!is_shell_class("Chrome_WidgetWin_1"));
    }
}
