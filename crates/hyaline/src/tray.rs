//! The notification-area icon, its popup menu, notices and the color picker.

use crate::bindings::{
    AppendMenuW, CC_FULLOPEN, CC_RGBINIT, CHOOSECOLORW, COLORREF, CheckMenuRadioItem, ChooseColorW,
    CreatePopupMenu, DestroyIcon, DestroyMenu, GetModuleHandleW, GetSystemMetrics, HICON,
    HKEY_CURRENT_USER, HMENU, HWND, IMAGE_ICON, LPARAM, LR_DEFAULTCOLOR, LoadImageW, MF_BYCOMMAND,
    MF_CHECKED, MF_GRAYED, MF_POPUP, MF_SEPARATOR, MF_STRING, NIF_ICON, NIF_INFO, NIF_MESSAGE,
    NIF_TIP, NIIF_INFO, NIIF_WARNING, NIM_ADD, NIM_DELETE, NIM_MODIFY, NIM_SETVERSION,
    NIN_KEYSELECT, NIN_SELECT, NOTIFYICON_VERSION_4, NOTIFYICONDATAW, POINT, PostMessageW,
    RRF_RT_REG_DWORD, RegGetValueW, SM_CXSMICON, SM_CYSMICON, SetForegroundWindow,
    Shell_NotifyIconW, TPM_RETURNCMD, TPM_RIGHTBUTTON, TrackPopupMenu, WM_APP, WM_CONTEXTMENU,
    WM_NULL, WPARAM,
};
use hyaline_core::appearance::{Mode, Rgb};
use hyaline_core::states::{Looks, Situation, Toggled};
use windows_core::{HSTRING, PCWSTR, w};

/// Message the shell sends to our window for icon clicks.
pub const MESSAGE: u32 = WM_APP.cast_unsigned() + 1;
const ICON_ID: u32 = 1;

const MODES: [(Mode, &str); 5] = [
    (Mode::Normal, "Normal"),
    (Mode::Opaque, "Opaque"),
    (Mode::Clear, "Clear"),
    (Mode::Blur, "Blur"),
    (Mode::Acrylic, "Acrylic"),
];
const OPACITIES: [u8; 5] = [0, 25, 50, 75, 100];

/// Situations in menu order, with their labels.
const SITUATIONS: [(Situation, &str); 6] = [
    (Situation::Desktop, "Desktop"),
    (Situation::VisibleWindow, "Visible window"),
    (Situation::MaximizedWindow, "Maximized window"),
    (Situation::Start, "Start menu"),
    (Situation::Search, "Search"),
    (Situation::TaskView, "Task View"),
];

/// What the user picked in the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Toggle(Situation),
    Mode(Situation, Mode),
    Color(Situation),
    Opacity(Situation, u8),
    Autostart,
    Exit,
}

// Menu ids: 1000 + situation * 100 + item; global items above 9000.
const ID_SITUATIONS: usize = 1000;
const ITEM_TOGGLE: usize = 0;
const ITEM_MODE: usize = 10;
const ITEM_COLOR: usize = 20;
const ITEM_OPACITY: usize = 30;
const ID_AUTOSTART: usize = 9000;
const ID_EXIT: usize = 9001;

/// The menu id for `choice`.
fn encode(choice: Choice) -> usize {
    let situation_base = |situation: Situation| {
        let index = SITUATIONS
            .iter()
            .position(|(s, _)| *s == situation)
            .unwrap_or(0);
        ID_SITUATIONS + index * 100
    };
    let mode_index = |mode: Mode| MODES.iter().position(|(m, _)| *m == mode).unwrap_or(0);
    let opacity_index = |opacity: u8| OPACITIES.iter().position(|o| *o == opacity).unwrap_or(0);
    match choice {
        Choice::Toggle(situation) => situation_base(situation) + ITEM_TOGGLE,
        Choice::Mode(situation, mode) => situation_base(situation) + ITEM_MODE + mode_index(mode),
        Choice::Color(situation) => situation_base(situation) + ITEM_COLOR,
        Choice::Opacity(situation, opacity) => {
            situation_base(situation) + ITEM_OPACITY + opacity_index(opacity)
        }
        Choice::Autostart => ID_AUTOSTART,
        Choice::Exit => ID_EXIT,
    }
}

/// The choice for a menu id (`None` for cancel or unknown ids).
fn decode(id: usize) -> Option<Choice> {
    match id {
        ID_AUTOSTART => return Some(Choice::Autostart),
        ID_EXIT => return Some(Choice::Exit),
        _ => {}
    }
    let situation = SITUATIONS.get(id.checked_sub(ID_SITUATIONS)? / 100)?.0;
    let item = (id - ID_SITUATIONS) % 100;
    match item {
        ITEM_TOGGLE => Some(Choice::Toggle(situation)),
        ITEM_COLOR => Some(Choice::Color(situation)),
        _ if (ITEM_MODE..ITEM_MODE + MODES.len()).contains(&item) => {
            Some(Choice::Mode(situation, MODES[item - ITEM_MODE].0))
        }
        _ if (ITEM_OPACITY..ITEM_OPACITY + OPACITIES.len()).contains(&item) => {
            Some(Choice::Opacity(situation, OPACITIES[item - ITEM_OPACITY]))
        }
        _ => None,
    }
}

fn icon_data(hwnd: HWND) -> NOTIFYICONDATAW {
    NOTIFYICONDATAW {
        cbSize: u32::try_from(size_of::<NOTIFYICONDATAW>()).unwrap_or(u32::MAX),
        hWnd: hwnd,
        uID: ICON_ID,
        ..Default::default()
    }
}

/// Copies `text` into a fixed UTF-16 buffer, truncated and NUL-terminated.
fn copy_text(buffer: &mut [u16], text: &str) {
    let room = buffer.len() - 1;
    let mut length = 0;
    for (slot, unit) in buffer.iter_mut().zip(text.encode_utf16().take(room)) {
        *slot = unit;
        length += 1;
    }
    buffer[length] = 0;
}

/// Adds the icon (again after an Explorer restart, which drops all icons).
pub fn add(hwnd: HWND) -> bool {
    // SAFETY: null name = this executable; icon resource 1 is embedded by build.rs.
    let icon = unsafe {
        LoadImageW(
            Some(GetModuleHandleW(PCWSTR::null())),
            // MAKEINTRESOURCE(1): a resource id passed in place of a name.
            PCWSTR(std::ptr::without_provenance(1)),
            IMAGE_ICON.cast_unsigned(),
            GetSystemMetrics(SM_CXSMICON),
            GetSystemMetrics(SM_CYSMICON),
            LR_DEFAULTCOLOR.cast_unsigned(),
        )
    };
    let mut data = icon_data(hwnd);
    data.uFlags = (NIF_ICON | NIF_MESSAGE | NIF_TIP).cast_unsigned();
    data.uCallbackMessage = MESSAGE;
    data.hIcon = HICON(icon.0);
    copy_text(&mut data.szTip, "Hyaline");
    // SAFETY: `data` is fully initialized; the shell copies the icon.
    let added = unsafe { Shell_NotifyIconW(NIM_ADD.cast_unsigned(), &raw const data) }.as_bool();
    // Version 4 delivers keyboard activation (Enter, Space, Shift+F10) as well as clicks.
    data.Anonymous.uVersion = NOTIFYICON_VERSION_4.cast_unsigned();
    // SAFETY: as above; the shell reads the version field.
    let _ = unsafe { Shell_NotifyIconW(NIM_SETVERSION.cast_unsigned(), &raw const data) };
    if !icon.0.is_null() {
        // SAFETY: loaded above and no longer needed.
        let _ = unsafe { DestroyIcon(HICON(icon.0)) };
    }
    added
}

pub fn remove(hwnd: HWND) {
    let data = icon_data(hwnd);
    // SAFETY: identifies our icon by window and id.
    let _ = unsafe { Shell_NotifyIconW(NIM_DELETE.cast_unsigned(), &raw const data) };
}

/// If the tray message (`lparam`) is an activation by mouse or keyboard, where to anchor the
/// menu (screen coordinates from `wparam`).
pub fn activation(wparam: usize, lparam: isize) -> Option<POINT> {
    let event = i32::try_from(lparam & 0xFFFF).ok()?;
    if !matches!(event, WM_CONTEXTMENU | NIN_SELECT | NIN_KEYSELECT) {
        return None;
    }
    // Signed 16-bit coordinates: monitors left of or above the primary are negative.
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let (x, y) = (wparam as u16 as i16, (wparam >> 16) as u16 as i16);
    Some(POINT {
        x: x.into(),
        y: y.into(),
    })
}

/// Shows a notification from the icon.
pub fn notify(hwnd: HWND, title: &str, text: &str, warning: bool) {
    let mut data = icon_data(hwnd);
    data.uFlags = NIF_INFO.cast_unsigned();
    data.dwInfoFlags = if warning { NIIF_WARNING } else { NIIF_INFO }.cast_unsigned();
    copy_text(&mut data.szInfoTitle, title);
    copy_text(&mut data.szInfo, text);
    // SAFETY: `data` is fully initialized.
    let _ = unsafe { Shell_NotifyIconW(NIM_MODIFY.cast_unsigned(), &raw const data) };
}

/// The user's "Transparency effects" setting (on when it cannot be read).
fn transparency_effects_on() -> bool {
    let mut value = 1u32;
    let mut size = 4u32;
    // SAFETY: `value`/`size` describe a 4-byte DWORD buffer.
    let status = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            w!(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"),
            w!("EnableTransparency"),
            RRF_RT_REG_DWORD.cast_unsigned(),
            None,
            Some((&raw mut value).cast()),
            Some(&raw mut size),
        )
    };
    status.0 != 0 || value != 0
}

fn append(menu: HMENU, flags: i32, id: usize, label: &str) {
    let label = HSTRING::from(label);
    // SAFETY: valid menu; the label is copied by the call.
    let _ = unsafe { AppendMenuW(menu, flags.cast_unsigned(), id, PCWSTR(label.as_ptr())) };
}

fn index_u32(index: usize) -> u32 {
    u32::try_from(index).unwrap_or(u32::MAX)
}

/// Shows the menu at `anchor` and returns what the user picked.
pub fn show_menu(hwnd: HWND, anchor: POINT, looks: &Looks, autostart: bool) -> Option<Choice> {
    let effects_on = transparency_effects_on();
    // SAFETY: plain menu creation; destroying the top menu also destroys its submenus.
    let menu = unsafe { CreatePopupMenu() };
    for (situation, label) in SITUATIONS {
        let submenu = situation_menu(situation, looks.toggled(situation), effects_on);
        append(menu, MF_POPUP, submenu.0 as usize, label);
    }
    append(menu, MF_SEPARATOR, 0, "");
    let checked = if autostart { MF_CHECKED } else { 0 };
    append(
        menu,
        MF_STRING | checked,
        ID_AUTOSTART,
        "Start with Windows",
    );
    append(menu, MF_SEPARATOR, 0, "");
    append(menu, MF_STRING, ID_EXIT, "Exit");

    // SAFETY: our own window and menu. SetForegroundWindow and the trailing WM_NULL are the
    // documented way to make tray menus close when clicking elsewhere.
    let picked = unsafe {
        let _ = SetForegroundWindow(hwnd);
        let picked = TrackPopupMenu(
            menu,
            (TPM_RETURNCMD | TPM_RIGHTBUTTON).cast_unsigned(),
            anchor.x,
            anchor.y,
            None,
            hwnd,
            None,
        );
        let _ = PostMessageW(Some(hwnd), WM_NULL.cast_unsigned(), WPARAM(0), LPARAM(0));
        let _ = DestroyMenu(menu);
        picked
    };
    decode(usize::try_from(picked.0).ok()?)
}

/// One situation's submenu: On (except Desktop), Appearance, Color…, Opacity.
fn situation_menu(situation: Situation, toggled: Toggled, effects_on: bool) -> HMENU {
    // SAFETY: plain menu creation; owned by the parent menu once appended.
    let (menu, modes, opacities) =
        unsafe { (CreatePopupMenu(), CreatePopupMenu(), CreatePopupMenu()) };
    for (mode, label) in MODES {
        let needs_effects = matches!(mode, Mode::Blur | Mode::Acrylic) && !effects_on;
        let label = if needs_effects {
            format!("{label} (needs Transparency effects)")
        } else {
            label.to_string()
        };
        append(
            modes,
            MF_STRING,
            encode(Choice::Mode(situation, mode)),
            &label,
        );
    }
    let first = encode(Choice::Mode(situation, MODES[0].0));
    let current = encode(Choice::Mode(situation, toggled.look.mode));
    // SAFETY: the ids exist in `modes`.
    let _ = unsafe {
        CheckMenuRadioItem(
            modes,
            index_u32(first),
            index_u32(first + MODES.len() - 1),
            index_u32(current),
            MF_BYCOMMAND.cast_unsigned(),
        )
    };
    for opacity in OPACITIES {
        let checked = if opacity == toggled.look.opacity {
            MF_CHECKED
        } else {
            0
        };
        let id = encode(Choice::Opacity(situation, opacity));
        append(opacities, MF_STRING | checked, id, &format!("{opacity}%"));
    }
    let look = toggled.look;
    let tinted = !matches!(look.mode, Mode::Normal);
    let has_opacity = !matches!(look.mode, Mode::Normal | Mode::Opaque);
    let grayed = |enabled: bool| if enabled { 0 } else { MF_GRAYED };
    if situation != Situation::Desktop {
        let checked = if toggled.enabled { MF_CHECKED } else { 0 };
        append(
            menu,
            MF_STRING | checked,
            encode(Choice::Toggle(situation)),
            "On",
        );
        append(menu, MF_SEPARATOR, 0, "");
    }
    append(menu, MF_POPUP, modes.0 as usize, "Appearance");
    append(
        menu,
        MF_STRING | grayed(tinted),
        encode(Choice::Color(situation)),
        &format!("Color… ({})", look.color),
    );
    append(
        menu,
        MF_POPUP | grayed(has_opacity),
        opacities.0 as usize,
        "Opacity",
    );
    menu
}

/// Opens the Windows color picker; `None` if the user cancels.
pub fn pick_color(hwnd: HWND, current: Rgb) -> Option<Rgb> {
    let to_colorref =
        |c: Rgb| COLORREF(u32::from(c.b) << 16 | u32::from(c.g) << 8 | u32::from(c.r));
    let mut custom = [COLORREF(0x00FF_FFFF); 16];
    let mut dialog = CHOOSECOLORW {
        lStructSize: u32::try_from(size_of::<CHOOSECOLORW>()).unwrap_or(u32::MAX),
        hwndOwner: hwnd,
        rgbResult: to_colorref(current),
        lpCustColors: custom.as_mut_ptr(),
        Flags: (CC_RGBINIT | CC_FULLOPEN).cast_unsigned(),
        ..Default::default()
    };
    // SAFETY: `dialog` is initialized and `custom` outlives the modal call.
    if !unsafe { ChooseColorW(&raw mut dialog) }.as_bool() {
        return None;
    }
    let [r, g, b, _] = dialog.rgbResult.0.to_le_bytes();
    Some(Rgb { r, g, b })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_choice_survives_its_menu_id() {
        let mut choices = vec![Choice::Autostart, Choice::Exit];
        for (situation, _) in SITUATIONS {
            choices.push(Choice::Toggle(situation));
            choices.push(Choice::Color(situation));
            choices.extend(MODES.iter().map(|(mode, _)| Choice::Mode(situation, *mode)));
            choices.extend(OPACITIES.iter().map(|o| Choice::Opacity(situation, *o)));
        }
        let mut ids: Vec<usize> = choices.iter().map(|c| encode(*c)).collect();
        for (choice, id) in choices.iter().zip(&ids) {
            assert_eq!(decode(*id), Some(*choice), "id {id}");
        }
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), choices.len(), "ids are unique");
    }

    #[test]
    fn cancel_and_unknown_ids_are_nothing() {
        for id in [0, 1, 999, 1099, 1600, 8999, 9002] {
            assert_eq!(decode(id), None, "id {id}");
        }
    }
}
