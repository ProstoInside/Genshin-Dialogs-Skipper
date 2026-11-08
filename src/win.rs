//! Системные функции: проверка окна игры, права администратора, перезапуск.

use std::{os::windows::process::CommandExt, process::Command};

use windows::{
    core::PWSTR,
    Win32::{
        Foundation::CloseHandle,
        System::Threading::{
            OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
            PROCESS_QUERY_LIMITED_INFORMATION,
        },
        UI::{
            Shell::IsUserAnAdmin,
            WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId},
        },
    },
};

const GENSHIN_EXES: &[&str] = &["genshinimpact.exe", "yuanshen.exe"];
const GENSHIN_TITLES: &[&str] = &["Genshin Impact", "原神"];

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn is_admin() -> bool {
    unsafe { IsUserAnAdmin().as_bool() }
}

/// Активно ли сейчас окно Genshin Impact.
/// Сначала проверяем имя процесса, а если античит не даёт открыть процесс —
/// сверяем заголовок окна.
pub fn is_genshin_foreground() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();

        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid as *mut u32));
        if pid == 0 {
            return false;
        }

        if let Some(exe) = process_exe_name(pid) {
            return GENSHIN_EXES.contains(&exe.as_str());
        }

        let mut buf = [0u16; 256];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len <= 0 {
            return false;
        }
        let title = String::from_utf16_lossy(&buf[..len as usize]);
        GENSHIN_TITLES.contains(&title.trim())
    }
}

unsafe fn process_exe_name(pid: u32) -> Option<String> {
    let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;

    let mut buf = [0u16; 1024];
    let mut len = buf.len() as u32;
    let res = QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut len);
    let _ = CloseHandle(handle);
    res.ok()?;

    let path = String::from_utf16_lossy(&buf[..len as usize]);
    path.rsplit(['\\', '/'])
        .next()
        .map(|name| name.to_ascii_lowercase())
}

/// Перезапуск текущего exe с правами администратора (через UAC).
/// Возвращает `true`, если новый процесс запущен и текущий можно закрывать.
pub fn relaunch_as_admin() -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let exe = exe.to_string_lossy().replace('\'', "''");

    Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!("Start-Process -FilePath '{exe}' -Verb RunAs"),
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
