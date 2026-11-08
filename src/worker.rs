//! Фоновые потоки: цикл нажатий и глобальный хоткей.

use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering::Relaxed},
        Arc,
    },
    thread,
    time::Duration,
};

use crate::{
    input, keys,
    settings::Settings,
    sound::{self, Sound},
    win,
};
use eframe::egui;

/// Общее состояние между UI и фоновыми потоками. Только атомики — никаких
/// обращений к UI из чужих потоков.
pub struct Shared {
    pub running: AtomicBool,
    pub press_f: AtomicBool,
    pub press_space: AtomicBool,
    pub press_lmb: AtomicBool,
    pub only_genshin: AtomicBool,
    pub genshin_focused: AtomicBool,
    pub presses: AtomicU64,
    /// Виртуальный код клавиши запуска/остановки.
    pub hotkey: AtomicU32,
    /// Ждём нажатия новой клавиши для хоткея.
    pub capturing: AtomicBool,
    /// Звук при включении/выключении.
    pub sound: AtomicBool,
}

impl Shared {
    pub fn new(s: &Settings) -> Self {
        Self {
            running: AtomicBool::new(false),
            press_f: AtomicBool::new(s.press_f),
            press_space: AtomicBool::new(s.press_space),
            press_lmb: AtomicBool::new(s.press_lmb),
            only_genshin: AtomicBool::new(s.only_genshin),
            genshin_focused: AtomicBool::new(false),
            presses: AtomicU64::new(0),
            hotkey: AtomicU32::new(s.hotkey),
            capturing: AtomicBool::new(false),
            sound: AtomicBool::new(s.sound),
        }
    }
    /// Единая точка вкл/выкл (и для хоткея, и для кнопки) — звук играет в обоих случаях.
    pub fn toggle(&self) {
        let was_running = self.running.fetch_xor(true, Relaxed);
        if self.sound.load(Relaxed) {
            sound::play(if was_running { Sound::Off } else { Sound::On });
        }
    }

    fn can_press(&self) -> bool {
        self.running.load(Relaxed)
            && (!self.only_genshin.load(Relaxed) || self.genshin_focused.load(Relaxed))
    }
}

/// Флаг включения действия + само действие (получает длительность удержания).
type Action<'a> = (&'a AtomicBool, fn(Duration));

fn rand_ms(lo: u64, hi: u64) -> Duration {
    Duration::from_millis(fastrand::u64(lo..=hi))
}

/// Один постоянный рабочий поток: никаких гонок при быстром вкл/выкл.
pub fn spawn_clicker(shared: Arc<Shared>, ctx: egui::Context) {
    thread::spawn(move || {
        let mut was_focused = false;
        loop {
            let focused = win::is_genshin_foreground();
            shared.genshin_focused.store(focused, Relaxed);
            if focused != was_focused {
                was_focused = focused;
                ctx.request_repaint();
            }

            if !shared.can_press() {
                thread::sleep(Duration::from_millis(50));
                continue;
            }

            let actions: [Action; 3] = [
                (&shared.press_f, |h| input::tap_key(input::SC_F, h)),
                (&shared.press_space, |h| input::tap_key(input::SC_SPACE, h)),
                (&shared.press_lmb, input::click_left),
            ];

            for (enabled, action) in actions {
                if !shared.can_press() {
                    break;
                }
                if enabled.load(Relaxed) {
                    action(rand_ms(25, 55));
                    shared.presses.fetch_add(1, Relaxed);
                    thread::sleep(rand_ms(20, 40));
                }
            }

            // Пауза между циклами — каждый раз разная.
            thread::sleep(rand_ms(80, 140));
        }
    });
}

/// Глобальный хоткей через опрос GetAsyncKeyState (без хуков) и захват
/// новой клавиши при перебинде.
pub fn spawn_hotkey(shared: Arc<Shared>, ctx: egui::Context) {
    thread::spawn(move || {
        // Первый опрос сбрасывает "старый" бит нажатия — без ложного старта.
        let mut hotkey_was_down = keys::is_down(shared.hotkey.load(Relaxed));
        let mut prev = [false; 256];
        let mut was_capturing = false;

        loop {
            thread::sleep(Duration::from_millis(20));

            if shared.capturing.load(Relaxed) {
                let mut now = [false; 256];
                for (vk, down) in now.iter_mut().enumerate().skip(1) {
                    *down = keys::is_down(vk as u32);
                }
                // Клавиши, зажатые до начала захвата, не считаем.
                if !was_capturing {
                    prev = now;
                    was_capturing = true;
                    continue;
                }
                let pressed = |vk: u32| now[vk as usize] && !prev[vk as usize];

                if pressed(keys::VK_ESCAPE) {
                    shared.capturing.store(false, Relaxed);
                    ctx.request_repaint();
                } else if let Some(vk) =
                    (1..256u32).find(|&vk| pressed(vk) && keys::is_bindable(vk))
                {
                    shared.hotkey.store(vk, Relaxed);
                    shared.capturing.store(false, Relaxed);
                    // Новая клавиша сейчас зажата — не переключаем сразу.
                    hotkey_was_down = true;
                    ctx.request_repaint();
                }
                prev = now;
                continue;
            }
            was_capturing = false;

            let down = keys::is_down(shared.hotkey.load(Relaxed));
            if down && !hotkey_was_down {
                shared.toggle();
                ctx.request_repaint();
            }
            hotkey_was_down = down;
        }
    });
}
