//! Сохранение настроек в %APPDATA%\GDS\settings.ini (простой key=value).

use std::{env, fs, path::PathBuf};

use crate::{i18n::Lang, keys};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Settings {
    pub lang: Lang,
    pub press_f: bool,
    pub press_space: bool,
    pub press_lmb: bool,
    pub only_genshin: bool,
    pub hotkey: u32,
    pub sound: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            lang: Lang::system(),
            press_f: true,
            press_space: true,
            press_lmb: false,
            only_genshin: true,
            hotkey: keys::DEFAULT_HOTKEY,
            sound: true,
        }
    }
}

fn path() -> Option<PathBuf> {
    env::var_os("APPDATA").map(|dir| PathBuf::from(dir).join("GDS").join("settings.ini"))
}

pub fn load() -> Settings {
    let mut s = Settings::default();
    let Some(text) = path().and_then(|p| fs::read_to_string(p).ok()) else {
        return s;
    };

    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let flag = value.trim() == "1";
        match key.trim() {
            "lang" => {
                if let Some(lang) = Lang::from_code(value) {
                    s.lang = lang;
                }
            }
            "press_f" => s.press_f = flag,
            "press_space" => s.press_space = flag,
            "press_lmb" => s.press_lmb = flag,
            "only_genshin" => s.only_genshin = flag,
            "sound" => s.sound = flag,
            "hotkey" => {
                if let Ok(vk) = value.trim().parse::<u32>() {
                    if keys::is_bindable(vk) {
                        s.hotkey = vk;
                    }
                }
            }
            _ => {}
        }
    }
    s
}

pub fn save(s: &Settings) {
    let Some(path) = path() else { return };
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    let b = |v: bool| if v { "1" } else { "0" };
    let text = format!(
        "lang={}\npress_f={}\npress_space={}\npress_lmb={}\nonly_genshin={}\nhotkey={}\nsound={}\n",
        s.lang.code(),
        b(s.press_f),
        b(s.press_space),
        b(s.press_lmb),
        b(s.only_genshin),
        s.hotkey,
        b(s.sound),
    );
    let _ = fs::write(path, text);
}
