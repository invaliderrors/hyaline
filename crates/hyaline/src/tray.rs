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
use hyaline_core::appearance::{Appearance, Mode, Rgb};
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
const ID_MODE: usize = 100;
const ID_COLOR: usize = 200;
const ID_OPACITY: usize = 300;
const ID_AUTOSTART: usize = 400;
const ID_EXIT: usize = 500;

/// What the user picked in the menu.
pub enum Choice {
    Mode(Mode),
    Color,
    Opacity(u8),
    Autostart,
    Exit,
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
pub fn show_menu(
    hwnd: HWND,
    anchor: POINT,
    current: &Appearance,
    autostart: bool,
) -> Option<Choice> {
    let effects_on = transparency_effects_on();
    // SAFETY: plain menu creation; every menu is destroyed below (submenus with their parent).
    let (menu, modes, opacities) =
        unsafe { (CreatePopupMenu(), CreatePopupMenu(), CreatePopupMenu()) };
    for (i, (mode, label)) in MODES.iter().enumerate() {
        let needs_effects = matches!(mode, Mode::Blur | Mode::Acrylic) && !effects_on;
        let label = if needs_effects {
            format!("{label} (needs Transparency effects)")
        } else {
            (*label).to_string()
        };
        append(modes, MF_STRING, ID_MODE + i, &label);
    }
    if let Some(current_index) = MODES.iter().position(|(m, _)| *m == current.mode) {
        let first = index_u32(ID_MODE);
        // SAFETY: the ids exist in `modes`.
        let _ = unsafe {
            CheckMenuRadioItem(
                modes,
                first,
                first + index_u32(MODES.len() - 1),
                first + index_u32(current_index),
                MF_BYCOMMAND.cast_unsigned(),
            )
        };
    }
    for (i, opacity) in OPACITIES.iter().enumerate() {
        let checked = if *opacity == current.opacity {
            MF_CHECKED
        } else {
            0
        };
        append(
            opacities,
            MF_STRING | checked,
            ID_OPACITY + i,
            &format!("{opacity}%"),
        );
    }
    let tinted = !matches!(current.mode, Mode::Normal);
    let has_opacity = !matches!(current.mode, Mode::Normal | Mode::Opaque);
    let grayed = |enabled: bool| if enabled { 0 } else { MF_GRAYED };
    append(menu, MF_POPUP, modes.0 as usize, "Appearance");
    append(
        menu,
        MF_STRING | grayed(tinted),
        ID_COLOR,
        &format!("Color… ({})", current.color),
    );
    append(
        menu,
        MF_POPUP | grayed(has_opacity),
        opacities.0 as usize,
        "Opacity",
    );
    append(menu, MF_SEPARATOR, 0, "");
    append(
        menu,
        MF_STRING | if autostart { MF_CHECKED } else { 0 },
        ID_AUTOSTART,
        "Start with Windows",
    );
    append(menu, MF_SEPARATOR, 0, "");
    append(menu, MF_STRING, ID_EXIT, "Exit");

    // SAFETY: our own window and menu. SetForegroundWindow and the trailing
    // WM_NULL are the documented way to make tray menus close when clicking elsewhere.
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
    let id = usize::try_from(picked.0).ok()?;
    match id {
        ID_COLOR => Some(Choice::Color),
        ID_AUTOSTART => Some(Choice::Autostart),
        ID_EXIT => Some(Choice::Exit),
        _ if (ID_MODE..ID_MODE + MODES.len()).contains(&id) => {
            Some(Choice::Mode(MODES[id - ID_MODE].0))
        }
        _ if (ID_OPACITY..ID_OPACITY + OPACITIES.len()).contains(&id) => {
            Some(Choice::Opacity(OPACITIES[id - ID_OPACITY]))
        }
        _ => None,
    }
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
