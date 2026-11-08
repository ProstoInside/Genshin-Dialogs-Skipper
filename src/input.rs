//! Низкоуровневый ввод через SendInput со сканкодами (как pydirectinput).

use std::{mem::size_of, thread, time::Duration};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYEVENTF_KEYUP,
    KEYEVENTF_SCANCODE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT, MOUSE_EVENT_FLAGS,
    VIRTUAL_KEY,
};

/// Сканкоды набора 1 (не зависят от раскладки).
pub const SC_F: u16 = 0x21;
pub const SC_SPACE: u16 = 0x39;

fn send(inputs: &[INPUT]) {
    unsafe {
        SendInput(inputs, size_of::<INPUT>() as i32);
    }
}

fn key_input(scan: u16, up: bool) -> INPUT {
    let flags = if up {
        KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP
    } else {
        KEYEVENTF_SCANCODE
    };
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: scan,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn mouse_input(flags: MOUSE_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Нажать и отпустить клавишу с удержанием `hold`.
/// Отпускание отправляется всегда, поэтому клавиша не "залипнет".
pub fn tap_key(scan: u16, hold: Duration) {
    send(&[key_input(scan, false)]);
    thread::sleep(hold);
    send(&[key_input(scan, true)]);
}

/// Клик ЛКМ в текущей позиции курсора.
pub fn click_left(hold: Duration) {
    send(&[mouse_input(MOUSEEVENTF_LEFTDOWN)]);
    thread::sleep(hold);
    send(&[mouse_input(MOUSEEVENTF_LEFTUP)]);
}
