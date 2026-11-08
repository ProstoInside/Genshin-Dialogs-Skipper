//! Хоткей: виртуальные коды клавиш, их названия и правила назначения.

use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetKeyNameTextW, MapVirtualKeyW, MAPVK_VK_TO_VSC,
};

/// END по умолчанию.
pub const DEFAULT_HOTKEY: u32 = 0x23;
pub const VK_ESCAPE: u32 = 0x1B;

/// Клавиша зажата сейчас (бит 0x8000) или была нажата с прошлого опроса (бит 0x0001).
/// Второй бит ловит очень короткие нажатия, которые целиком помещаются между опросами.
pub fn is_down(vk: u32) -> bool {
    unsafe { GetAsyncKeyState(vk as i32) as u16 & 0x8001 != 0 }
}

/// Можно ли назначить клавишу хоткеем.
/// Запрещены клавиши, которые жмёт сам скрипт (F, Space, ЛКМ) — иначе он
/// выключал бы себя собственными нажатиями. Также ПКМ (клик по интерфейсу),
/// Esc (отмена), Win, "общие" Shift/Ctrl/Alt (есть левые/правые варианты)
/// и служебные коды, которые не соответствуют физическим клавишам.
pub fn is_bindable(vk: u32) -> bool {
    (0x01..=0xFE).contains(&vk)
        && !matches!(
            vk,
            0x01 | 0x02 // ЛКМ, ПКМ
                | 0x10
                ..=0x12 // Shift, Ctrl, Alt (общие)
                | 0x1B // Esc
                | 0x20 // Space
                | 0x46 // F
                | 0x5B | 0x5C // Win
                | 0xE5 // VK_PROCESSKEY (IME)
                | 0xE7 // VK_PACKET (синтетический Unicode-ввод, не физическая клавиша)
        )
}

/// Короткое название для кейкапа.
pub fn name(vk: u32) -> String {
    let fixed = match vk {
        0x04 => "MMB",
        0x05 => "MB4",
        0x06 => "MB5",
        0x08 => "BKSP",
        0x09 => "TAB",
        0x0D => "ENTER",
        0x13 => "PAUSE",
        0x14 => "CAPS",
        0x21 => "PGUP",
        0x22 => "PGDN",
        0x23 => "END",
        0x24 => "HOME",
        0x25 => "←",
        0x26 => "↑",
        0x27 => "→",
        0x28 => "↓",
        0x2C => "PRTSC",
        0x2D => "INS",
        0x2E => "DEL",
        0x6A => "NUM *",
        0x6B => "NUM +",
        0x6D => "NUM -",
        0x6E => "NUM .",
        0x6F => "NUM /",
        0x90 => "NUMLK",
        0x91 => "SCRLK",
        0xA0 => "LSHIFT",
        0xA1 => "RSHIFT",
        0xA2 => "LCTRL",
        0xA3 => "RCTRL",
        0xA4 => "LALT",
        0xA5 => "RALT",
        _ => "",
    };
    if !fixed.is_empty() {
        return fixed.to_owned();
    }

    match vk {
        0x30..=0x39 | 0x41..=0x5A => char::from(vk as u8).to_string(),
        0x60..=0x69 => format!("NUM {}", vk - 0x60),
        0x70..=0x87 => format!("F{}", vk - 0x6F),
        _ => system_name(vk).unwrap_or_else(|| format!("0x{vk:02X}")),
    }
}

/// Название от Windows (для OEM-клавиш вроде `~`, `[`, `;`).
fn system_name(vk: u32) -> Option<String> {
    unsafe {
        let scan = MapVirtualKeyW(vk, MAPVK_VK_TO_VSC);
        if scan == 0 {
            return None;
        }
        let mut buf = [0u16; 32];
        let len = GetKeyNameTextW((scan << 16) as i32, &mut buf);
        if len <= 0 {
            return None;
        }
        Some(String::from_utf16_lossy(&buf[..len as usize]).to_uppercase())
    }
}
