// В релизе — без консольного окна.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod i18n;
mod icon;
mod input;
mod keys;
mod settings;
mod sound;
mod theme;
mod widgets;
mod win;
mod worker;

use std::{
    sync::{atomic::Ordering::Relaxed, Arc},
    time::{Duration, Instant},
};

use eframe::egui::{self, pos2, vec2, Align2, Color32, Frame, Margin, Rect, Sense};

use i18n::Tr;
use settings::Settings;
use theme::*;
use widgets::Badge;
use worker::Shared;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Off,
    Paused,
    Active,
}

struct App {
    shared: Arc<Shared>,
    settings: Settings,
    is_admin: bool,
    logo: egui::TextureHandle,
    was_running: bool,
    session_start: Instant,
    session_base: u64,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::setup(&cc.egui_ctx);

        let settings = settings::load();
        let shared = Arc::new(Shared::new(&settings));
        worker::spawn_clicker(shared.clone(), cc.egui_ctx.clone());
        worker::spawn_hotkey(shared.clone(), cc.egui_ctx.clone());

        let logo = cc.egui_ctx.load_texture(
            "gds-logo",
            egui::ColorImage::from_rgba_unmultiplied([128, 128], &icon::render(128)),
            egui::TextureOptions::LINEAR,
        );

        Self {
            shared,
            settings,
            is_admin: win::is_admin(),
            logo,
            was_running: false,
            session_start: Instant::now(),
            session_base: 0,
        }
    }

    /// Сохраняем настройки, если пользователь что-то поменял.
    fn sync_settings(&mut self) {
        let s = &self.shared;
        let current = Settings {
            lang: self.settings.lang,
            press_f: s.press_f.load(Relaxed),
            press_space: s.press_space.load(Relaxed),
            press_lmb: s.press_lmb.load(Relaxed),
            only_genshin: s.only_genshin.load(Relaxed),
            hotkey: s.hotkey.load(Relaxed),
            sound: s.sound.load(Relaxed),
        };
        if current != self.settings {
            self.settings = current;
            settings::save(&self.settings);
        }
    }

    fn header(&self, ui: &mut egui::Ui, tr: &Tr) {
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 58.0), Sense::hover());
        let p = ui.painter();

        let logo_rect = Rect::from_min_size(rect.left_top(), vec2(58.0, 58.0));
        p.image(
            self.logo.id(),
            logo_rect,
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            Color32::WHITE,
        );

        let x = logo_rect.right() + 14.0;
        let y = rect.center().y;
        p.text(
            pos2(x, y + 2.0),
            Align2::LEFT_BOTTOM,
            "Genshin Dialogs Skipper",
            bold(21.0),
            GOLD,
        );
        p.text(
            pos2(x, y + 4.0),
            Align2::LEFT_TOP,
            tr.tagline,
            regular(13.0),
            TEXT_DIM,
        );
    }

    fn status_card(&self, ui: &mut egui::Ui, tr: &Tr, state: State, key: &str) {
        let (color, title) = match state {
            State::Off => (RED, tr.status_off),
            State::Paused => (AMBER, tr.status_paused),
            State::Active => (GREEN, tr.status_active),
        };
        let subtitle = match state {
            State::Off => tr.off_hint.replace("{key}", key),
            State::Paused => tr.paused_hint.to_owned(),
            State::Active => {
                let secs = self.session_start.elapsed().as_secs();
                let presses = self.shared.presses.load(Relaxed) - self.session_base;
                format!(
                    "{}: {}   ·   {}: {:02}:{:02}",
                    tr.presses,
                    presses,
                    tr.session,
                    secs / 60,
                    secs % 60
                )
            }
        };
        let focused = self.shared.genshin_focused.load(Relaxed);

        widgets::card(ui, mix(CARD_STROKE, color, 0.35), |ui| {
            let (rect, _) =
                ui.allocate_exact_size(vec2(ui.available_width(), 50.0), Sense::hover());
            let time = ui.input(|i| i.time) as f32;
            let p = ui.painter();

            // Индикатор с пульсацией в активном режиме
            let dot = pos2(rect.left() + 14.0, rect.center().y);
            if state == State::Active {
                let phase = (time * 0.9).fract();
                p.circle_filled(
                    dot,
                    7.0 + phase * 13.0,
                    color.gamma_multiply((1.0 - phase) * 0.45),
                );
            }
            p.circle_filled(dot, 11.0, color.gamma_multiply(0.18));
            p.circle_filled(dot, 6.5, color);

            let x = rect.left() + 40.0;
            let y = rect.center().y;
            p.text(
                pos2(x, y + 2.0),
                Align2::LEFT_BOTTOM,
                title,
                bold(21.0),
                color,
            );
            p.text(
                pos2(x, y + 3.0),
                Align2::LEFT_TOP,
                subtitle,
                regular(13.0),
                TEXT_DIM,
            );

            widgets::focus_pill(p, rect.right_center(), focused);
        });
    }

    fn options_card(&self, ui: &mut egui::Ui, tr: &Tr, key: &str) {
        let s = &self.shared;
        widgets::card(ui, CARD_STROKE, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(
                egui::RichText::new(tr.options_title)
                    .font(bold(11.5))
                    .color(GOLD_DIM),
            );
            ui.add_space(4.0);
            widgets::option_row(ui, Badge::Key("F"), tr.f_title, tr.f_desc, &s.press_f);
            widgets::option_row(
                ui,
                Badge::Key(tr.key_space),
                tr.space_title,
                tr.space_desc,
                &s.press_space,
            );
            widgets::option_row(
                ui,
                Badge::Key(tr.key_lmb),
                tr.lmb_title,
                tr.lmb_desc,
                &s.press_lmb,
            );
            widgets::divider(ui);
            widgets::option_row(
                ui,
                Badge::Focus,
                tr.focus_title,
                tr.focus_desc,
                &s.only_genshin,
            );
            widgets::divider(ui);

            let capturing = s.capturing.load(Relaxed);
            let (title, desc, button) = if capturing {
                (tr.hotkey_capture, tr.hotkey_capture_desc, tr.hotkey_cancel)
            } else {
                (tr.hotkey_title, tr.hotkey_desc, tr.hotkey_change)
            };
            if widgets::hotkey_row(ui, key, capturing, title, desc, button, tr.hotkey_forbidden) {
                s.capturing.store(!capturing, Relaxed);
            }
        });
    }

    fn admin_warning(&self, ui: &mut egui::Ui, tr: &Tr) {
        widgets::card(ui, AMBER.gamma_multiply(0.6), |ui| {
            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 8.0;
                ui.label(
                    egui::RichText::new(format!("⚠  {}", tr.admin_warn))
                        .font(semibold(13.0))
                        .color(AMBER),
                );
                if widgets::outline_button(ui, tr.admin_btn, AMBER) && win::relaunch_as_admin() {
                    std::process::exit(0);
                }
            });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let running = self.shared.running.load(Relaxed);
        if running && !self.was_running {
            self.session_start = Instant::now();
            self.session_base = self.shared.presses.load(Relaxed);
        }
        self.was_running = running;

        let state = if !running {
            State::Off
        } else if self.shared.only_genshin.load(Relaxed)
            && !self.shared.genshin_focused.load(Relaxed)
        {
            State::Paused
        } else {
            State::Active
        };
        let tr = self.settings.lang.tr();
        let key = keys::name(self.shared.hotkey.load(Relaxed));

        theme::paint_background(ctx);

        // Футер: версия + хоткей слева, переключатель языка справа
        egui::TopBottomPanel::bottom("footer")
            .frame(Frame::none().inner_margin(Margin {
                left: 22.0,
                right: 22.0,
                top: 4.0,
                bottom: 14.0,
            }))
            .show_separator_line(false)
            .show(ctx, |ui| {
                let (row, _) =
                    ui.allocate_exact_size(vec2(ui.available_width(), 28.0), Sense::hover());
                ui.painter().text(
                    row.left_center(),
                    Align2::LEFT_CENTER,
                    format!(
                        "v{VERSION}   ·   {}",
                        tr.footer_hotkey.replace("{key}", &key)
                    ),
                    regular(12.0),
                    TEXT_DIM,
                );
                let switch =
                    Rect::from_min_size(pos2(row.right() - 96.0, row.top()), vec2(96.0, 28.0));
                widgets::lang_switch(ui, switch, &mut self.settings.lang);

                let sound_btn = Rect::from_min_size(
                    pos2(switch.left() - 10.0 - 40.0, row.top()),
                    vec2(40.0, 28.0),
                );
                if widgets::sound_button(ui, sound_btn, &self.shared.sound, tr.sound_tip) {
                    // Предпрослушивание при включении звука.
                    sound::play(sound::Sound::On);
                }
            });

        egui::CentralPanel::default()
            .frame(Frame::none().inner_margin(Margin::symmetric(22.0, 18.0)))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = vec2(0.0, 14.0);

                self.header(ui, tr);
                self.status_card(ui, tr, state, &key);

                let label = if running { tr.btn_stop } else { tr.btn_start };
                if widgets::power_button(ui, running, label, &key) {
                    self.shared.toggle();
                }

                self.options_card(ui, tr, &key);

                if self.is_admin {
                    widgets::centered_lines(ui, tr.hint, regular(12.0), TEXT_DIM);
                } else {
                    self.admin_warning(ui, tr);
                }
            });

        self.sync_settings();

        // Анимация пульса и таймер — только в активном режиме.
        if state == State::Active {
            ctx.request_repaint_after(Duration::from_millis(40));
        }
    }
}

fn main() -> eframe::Result {
    let icon = egui::IconData {
        rgba: icon::render(64),
        width: 64,
        height: 64,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("GDS - Genshin Dialogs Skipper")
            .with_inner_size([440.0, 722.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "GDS - Genshin Dialogs Skipper",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
