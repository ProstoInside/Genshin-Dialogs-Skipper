//! Локализация: русский и английский.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    Ru,
    En,
}

impl Lang {
    pub fn tr(self) -> &'static Tr {
        match self {
            Lang::Ru => &RU,
            Lang::En => &EN,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Lang::Ru => "ru",
            Lang::En => "en",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.trim() {
            "ru" => Some(Lang::Ru),
            "en" => Some(Lang::En),
            _ => None,
        }
    }

    /// Язык по умолчанию — по языку интерфейса Windows.
    pub fn system() -> Self {
        const LANG_RUSSIAN: u16 = 0x19;
        let id = unsafe { windows::Win32::Globalization::GetUserDefaultUILanguage() };
        if id & 0x3FF == LANG_RUSSIAN {
            Lang::Ru
        } else {
            Lang::En
        }
    }
}

pub struct Tr {
    pub tagline: &'static str,

    pub status_off: &'static str,
    pub status_active: &'static str,
    pub status_paused: &'static str,
    pub off_hint: &'static str,
    pub paused_hint: &'static str,
    pub presses: &'static str,
    pub session: &'static str,

    pub btn_start: &'static str,
    pub btn_stop: &'static str,

    pub options_title: &'static str,
    pub key_space: &'static str,
    pub key_lmb: &'static str,
    pub f_title: &'static str,
    pub f_desc: &'static str,
    pub space_title: &'static str,
    pub space_desc: &'static str,
    pub lmb_title: &'static str,
    pub lmb_desc: &'static str,
    pub focus_title: &'static str,
    pub focus_desc: &'static str,
    pub hotkey_title: &'static str,
    pub hotkey_desc: &'static str,
    pub hotkey_capture: &'static str,
    pub hotkey_capture_desc: &'static str,
    pub hotkey_forbidden: &'static str,
    pub hotkey_change: &'static str,
    pub hotkey_cancel: &'static str,

    pub hint: &'static str,
    pub admin_warn: &'static str,
    pub admin_btn: &'static str,
    pub footer_hotkey: &'static str,
    pub sound_tip: &'static str,
}

pub static RU: Tr = Tr {
    tagline: "Скип диалогов без сломанного пальца",

    status_off: "Выключен",
    status_active: "Активен",
    status_paused: "Пауза",
    off_hint: "Нажми {key} или кнопку ниже",
    paused_hint: "Окно игры не в фокусе",
    presses: "Нажатий",
    session: "Сессия",

    btn_start: "ЗАПУСТИТЬ",
    btn_stop: "ОСТАНОВИТЬ",

    options_title: "НАСТРОЙКИ",
    key_space: "SPACE",
    key_lmb: "ЛКМ",
    f_title: "Клавиша F",
    f_desc: "Выбор реплик и подбор",
    space_title: "Пробел",
    space_desc: "Пропуск текста и анимаций",
    lmb_title: "Левая кнопка мыши",
    lmb_desc: "Клик для подстраховки",
    focus_title: "Только в Genshin",
    focus_desc: "Пауза, если игра не в фокусе",
    hotkey_title: "Клавиша запуска",
    hotkey_desc: "Включает и выключает скрипт",
    hotkey_capture: "Нажми новую клавишу…",
    hotkey_capture_desc: "Esc — отмена",
    hotkey_forbidden: "F, Пробел и ЛКМ назначить нельзя — их жмёт сам скрипт.\nМожно боковые кнопки мыши (MB4 / MB5).",
    hotkey_change: "Изменить",
    hotkey_cancel: "Отмена",

    hint: "Вне диалогов: Пробел — прыжок, ЛКМ — атака, F — подбор.\nВыключай скрипт после диалога.",
    admin_warn: "Нет прав администратора — игра не примет нажатия",
    admin_btn: "Перезапустить от администратора",
    footer_hotkey: "{key} — вкл / выкл",
    sound_tip: "Звук при включении и выключении",
};

pub static EN: Tr = Tr {
    tagline: "Skip dialogs without breaking a finger",

    status_off: "Disabled",
    status_active: "Active",
    status_paused: "Paused",
    off_hint: "Press {key} or the button below",
    paused_hint: "Game window is not focused",
    presses: "Presses",
    session: "Session",

    btn_start: "START",
    btn_stop: "STOP",

    options_title: "SETTINGS",
    key_space: "SPACE",
    key_lmb: "LMB",
    f_title: "F key",
    f_desc: "Pick replies and loot",
    space_title: "Space",
    space_desc: "Skip text and animations",
    lmb_title: "Left mouse button",
    lmb_desc: "Extra click, just in case",
    focus_title: "Genshin only",
    focus_desc: "Pause when the game is not focused",
    hotkey_title: "Toggle key",
    hotkey_desc: "Turns the script on and off",
    hotkey_capture: "Press a new key…",
    hotkey_capture_desc: "Esc to cancel",
    hotkey_forbidden: "F, Space and LMB can't be used — the script presses them itself.\nSide mouse buttons (MB4 / MB5) are supported.",
    hotkey_change: "Change",
    hotkey_cancel: "Cancel",

    hint: "Outside dialogs: Space = jump, LMB = attack, F = pick up.\nTurn the script off after the dialog.",
    admin_warn: "No admin rights — the game will ignore input",
    admin_btn: "Restart as administrator",
    footer_hotkey: "{key} — on / off",
    sound_tip: "Sound on start and stop",
};
