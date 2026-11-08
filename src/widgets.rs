//! Кастомные виджеты: карточки, переключатели, кейкапы, кнопка запуска, выбор языка.

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use eframe::egui::{
    self, pos2, vec2, Align2, Color32, CursorIcon, FontId, Margin, Painter, PointerButton, Pos2,
    Rect, Response, Sense, Stroke, Ui,
};

use crate::{i18n::Lang, theme::*};

/// Только клик мышью. Обычный `clicked()` срабатывает ещё и на Enter/Пробел для
/// виджета в фокусе — а скрипт сам спамит Пробелом, и без этого режиме "везде"
/// он переключал бы настройки в собственном окне.
fn clicked(resp: &Response) -> bool {
    resp.clicked_by(PointerButton::Primary)
}

pub fn card<R>(ui: &mut Ui, stroke: Color32, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::none()
        .fill(CARD)
        .stroke(Stroke::new(1.0_f32, stroke))
        .rounding(14.0)
        .inner_margin(Margin::symmetric(16.0, 14.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add(ui)
        })
        .inner
}

/// Многострочный текст, каждая строка по центру.
pub fn centered_lines(ui: &mut Ui, text: &str, font: FontId, color: Color32) {
    let line_h = font.size * 1.35;
    let lines: Vec<&str> = text.lines().collect();
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), line_h * lines.len() as f32),
        Sense::hover(),
    );
    for (i, line) in lines.iter().enumerate() {
        let pos = pos2(rect.center().x, rect.top() + line_h * (i as f32 + 0.5));
        ui.painter()
            .text(pos, Align2::CENTER_CENTER, line, font.clone(), color);
    }
}

pub enum Badge<'a> {
    Key(&'a str),
    Focus,
}

fn paint_keycap(p: &Painter, rect: Rect, fill: Color32, stroke: Color32) {
    p.rect_filled(rect.translate(vec2(0.0, 2.0)), 8.0, KEY_SHADOW);
    p.rect(rect, 8.0, fill, Stroke::new(1.0_f32, stroke));
}

fn paint_badge(p: &Painter, rect: Rect, badge: &Badge) {
    paint_keycap(p, rect, KEY_BG, GOLD_DIM);
    let c = rect.center();
    match badge {
        Badge::Key(text) => {
            let size = match text.chars().count() {
                0..=3 => 14.0,
                4..=5 => 11.5,
                _ => 10.0,
            };
            p.text(c, Align2::CENTER_CENTER, text, bold(size), GOLD_BRIGHT);
        }
        Badge::Focus => {
            let s = Stroke::new(1.6_f32, GOLD_BRIGHT);
            p.circle_stroke(c, 7.0, s);
            p.circle_filled(c, 2.4, GOLD_BRIGHT);
            for (dx, dy) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
                let d = vec2(dx, dy);
                p.line_segment([c + d * 8.5, c + d * 11.0], s);
            }
        }
    }
}

fn paint_switch(p: &Painter, rect: Rect, t: f32, hovered: bool) {
    let r = rect.height() / 2.0;
    p.rect_filled(rect, r, mix(TRACK_OFF, GOLD, t));
    let x = egui::lerp(rect.left() + r..=rect.right() - r, t);
    let knob = if hovered {
        Color32::WHITE
    } else {
        Color32::from_rgb(0xF4, 0xF1, 0xEA)
    };
    p.circle_filled(pos2(x, rect.center().y + 1.0), r - 3.0, KEY_SHADOW);
    p.circle_filled(pos2(x, rect.center().y), r - 3.0, knob);
}

/// Строка настройки: кейкап, заголовок, описание и переключатель.
/// Кликабельна вся строка.
pub fn option_row(ui: &mut Ui, badge: Badge, title: &str, desc: &str, flag: &AtomicBool) {
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 50.0), Sense::click());

    let mut on = flag.load(Relaxed);
    if clicked(&resp) {
        on = !on;
        flag.store(on, Relaxed);
    }

    let hovered = resp.hovered();
    let t = ui.ctx().animate_bool_with_time(resp.id, on, 0.15);
    let hover_t = ui
        .ctx()
        .animate_bool_with_time(resp.id.with("hover"), hovered, 0.1);
    let p = ui.painter();

    if hover_t > 0.0 {
        p.rect_filled(
            rect.expand2(vec2(8.0, 0.0)),
            10.0,
            mix(Color32::TRANSPARENT, HOVER, hover_t),
        );
    }

    let badge_rect =
        Rect::from_min_size(pos2(rect.left(), rect.center().y - 15.0), vec2(58.0, 30.0));
    paint_badge(p, badge_rect, &badge);

    let tx = badge_rect.right() + 14.0;
    let title_color = mix(TEXT_DIM, TEXT, 0.4 + 0.6 * t);
    p.text(
        pos2(tx, rect.center().y + 1.0),
        Align2::LEFT_BOTTOM,
        title,
        semibold(15.0),
        title_color,
    );
    p.text(
        pos2(tx, rect.center().y + 2.0),
        Align2::LEFT_TOP,
        desc,
        regular(12.5),
        TEXT_DIM,
    );

    let sw = Rect::from_min_size(
        pos2(rect.right() - 44.0, rect.center().y - 12.0),
        vec2(44.0, 24.0),
    );
    paint_switch(p, sw, t, hovered);

    resp.on_hover_cursor(CursorIcon::PointingHand);
}

/// Строка перебинда хоткея. Возвращает `true` при клике (начать/отменить захват).
pub fn hotkey_row(
    ui: &mut Ui,
    key: &str,
    capturing: bool,
    title: &str,
    desc: &str,
    button: &str,
    tooltip: &str,
) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 50.0), Sense::click());
    let hovered = resp.hovered();
    let hover_t = ui
        .ctx()
        .animate_bool_with_time(resp.id.with("hover"), hovered, 0.1);
    let time = ui.input(|i| i.time) as f32;
    let p = ui.painter();

    if hover_t > 0.0 {
        p.rect_filled(
            rect.expand2(vec2(8.0, 0.0)),
            10.0,
            mix(Color32::TRANSPARENT, HOVER, hover_t),
        );
    }

    // Кейкап с текущей клавишей; во время захвата — пульсирующая рамка и "…".
    let badge_rect =
        Rect::from_min_size(pos2(rect.left(), rect.center().y - 15.0), vec2(58.0, 30.0));
    if capturing {
        let pulse = 0.5 + 0.5 * (time * 5.0).sin();
        paint_keycap(p, badge_rect, KEY_BG, mix(GOLD_DIM, AMBER, pulse));
        p.text(
            badge_rect.center(),
            Align2::CENTER_CENTER,
            "…",
            bold(16.0),
            AMBER,
        );
    } else {
        paint_badge(p, badge_rect, &Badge::Key(key));
    }

    let tx = badge_rect.right() + 14.0;
    let (title_color, desc_color) = if capturing {
        (AMBER, TEXT_DIM)
    } else {
        (TEXT, TEXT_DIM)
    };
    p.text(
        pos2(tx, rect.center().y + 1.0),
        Align2::LEFT_BOTTOM,
        title,
        semibold(15.0),
        title_color,
    );
    p.text(
        pos2(tx, rect.center().y + 2.0),
        Align2::LEFT_TOP,
        desc,
        regular(12.5),
        desc_color,
    );

    // Кнопка-контур справа (визуальная, кликабельна вся строка)
    let color = if capturing { AMBER } else { GOLD };
    let galley = p.layout_no_wrap(button.to_owned(), semibold(12.5), color);
    let size = vec2(galley.size().x + 22.0, 28.0);
    let btn = Rect::from_min_size(
        pos2(rect.right() - size.x, rect.center().y - size.y / 2.0),
        size,
    );
    let fill = if hovered {
        color.gamma_multiply(0.15)
    } else {
        Color32::TRANSPARENT
    };
    p.rect(
        btn,
        8.0,
        fill,
        Stroke::new(1.0_f32, color.gamma_multiply(0.8)),
    );
    p.galley(btn.center() - galley.size() / 2.0, galley, color);

    if capturing {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(40));
    }
    resp.on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(tooltip)
        .clicked_by(PointerButton::Primary)
}

pub fn divider(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 9.0), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.center().y,
        Stroke::new(1.0_f32, CARD_STROKE),
    );
}

/// Большая кнопка запуска/остановки с кейкапом хоткея справа.
pub fn power_button(ui: &mut Ui, running: bool, label: &str, key: &str) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 54.0), Sense::click());
    let ctx = ui.ctx();
    let t = ctx.animate_bool_with_time(resp.id, running, 0.2);
    let hover = ctx.animate_bool_with_time(resp.id.with("hover"), resp.hovered(), 0.12);
    let press = if resp.is_pointer_button_down_on() {
        1.0
    } else {
        0.0
    };

    let base = mix(GOLD, RED_BTN, t);
    let fill = mix(base, Color32::WHITE, hover * 0.12);
    let ink = mix(INK, Color32::WHITE, t);
    let body = rect.translate(vec2(0.0, press * 2.0));

    let p = ui.painter();
    p.rect_filled(
        rect.translate(vec2(0.0, 4.0)),
        14.0,
        mix(base, Color32::BLACK, 0.55),
    );
    p.rect_filled(body, 14.0, fill);
    // тонкий блик по верхнему краю
    p.hline(
        body.x_range().shrink(14.0),
        body.top() + 1.5,
        Stroke::new(1.0_f32, mix(fill, Color32::WHITE, 0.35)),
    );

    p.text(body.center(), Align2::CENTER_CENTER, label, bold(17.0), ink);

    let key_galley = p.layout_no_wrap(key.to_owned(), bold(12.0), ink);
    let cap_w = (key_galley.size().x + 20.0).max(46.0);
    let cap = Rect::from_min_size(
        pos2(body.right() - 15.0 - cap_w, body.center().y - 13.0),
        vec2(cap_w, 26.0),
    );
    let cap_fill = mix(base, Color32::BLACK, 0.18);
    p.rect(
        cap,
        7.0,
        cap_fill,
        Stroke::new(1.0_f32, mix(base, ink, 0.35)),
    );
    p.galley(cap.center() - key_galley.size() / 2.0, key_galley, ink);

    resp.on_hover_cursor(CursorIcon::PointingHand)
        .clicked_by(PointerButton::Primary)
}

/// Индикатор "Genshin" в карточке статуса: зелёная точка, если окно игры активно.
pub fn focus_pill(p: &Painter, right_center: Pos2, focused: bool) {
    let galley = p.layout_no_wrap("Genshin".to_owned(), semibold(12.5), TEXT);
    let size = vec2(galley.size().x + 32.0, 26.0);
    let rect = Rect::from_min_size(right_center - vec2(size.x, size.y / 2.0), size);
    let (dot, text) = if focused {
        (GREEN, TEXT)
    } else {
        (TRACK_OFF, TEXT_DIM)
    };

    p.rect(rect, 13.0, KEY_BG, Stroke::new(1.0_f32, CARD_STROKE));
    p.circle_filled(pos2(rect.left() + 13.0, rect.center().y), 4.0, dot);
    p.text(
        pos2(rect.left() + 22.0, rect.center().y),
        Align2::LEFT_CENTER,
        "Genshin",
        semibold(12.5),
        text,
    );
}

/// Переключатель языка RU | EN. Возвращает `true`, если язык изменился.
pub fn lang_switch(ui: &mut Ui, rect: Rect, lang: &mut Lang) -> bool {
    let id = ui.id().with("lang_switch");
    let resp = ui.interact(rect, id, Sense::click());

    let mut changed = false;
    if clicked(&resp) {
        if let Some(pos) = resp.interact_pointer_pos() {
            let new = if pos.x < rect.center().x {
                Lang::Ru
            } else {
                Lang::En
            };
            if new != *lang {
                *lang = new;
                changed = true;
            }
        }
    }

    let target = if *lang == Lang::Ru { 0.0 } else { 1.0 };
    let t = ui.ctx().animate_value_with_time(id, target, 0.18);
    let half = rect.width() / 2.0;
    let p = ui.painter();

    p.rect(
        rect,
        rect.height() / 2.0,
        CARD,
        Stroke::new(1.0_f32, CARD_STROKE),
    );
    let knob = Rect::from_min_size(
        pos2(rect.left() + 2.0 + t * (half - 2.0), rect.top() + 2.0),
        vec2(half - 2.0, rect.height() - 4.0),
    );
    p.rect_filled(knob, knob.height() / 2.0, GOLD);

    for (i, label) in ["RU", "EN"].into_iter().enumerate() {
        let selected = 1.0 - (t - i as f32).abs();
        let center = pos2(rect.left() + half * (i as f32 + 0.5), rect.center().y);
        p.text(
            center,
            Align2::CENTER_CENTER,
            label,
            bold(12.5),
            mix(TEXT_DIM, INK, selected),
        );
    }

    resp.on_hover_cursor(CursorIcon::PointingHand);
    changed
}

/// Кнопка-динамик в футере. Возвращает `true`, если звук только что включили.
pub fn sound_button(ui: &mut Ui, rect: Rect, flag: &AtomicBool, tooltip: &str) -> bool {
    let resp = ui.interact(rect, ui.id().with("sound_button"), Sense::click());
    let mut on = flag.load(Relaxed);
    let mut enabled_now = false;
    if clicked(&resp) {
        on = !on;
        flag.store(on, Relaxed);
        enabled_now = on;
    }

    let color = match (on, resp.hovered()) {
        (true, true) => GOLD_BRIGHT,
        (true, false) => GOLD,
        (false, true) => TEXT,
        (false, false) => TEXT_DIM,
    };
    let p = ui.painter();
    p.rect(
        rect,
        rect.height() / 2.0,
        CARD,
        Stroke::new(1.0_f32, CARD_STROKE),
    );

    // Динамик: корпус + раструб
    let c = rect.center() - vec2(4.0, 0.0);
    p.add(egui::Shape::convex_polygon(
        vec![
            c + vec2(-6.0, -3.0),
            c + vec2(-2.5, -3.0),
            c + vec2(2.5, -7.0),
            c + vec2(2.5, 7.0),
            c + vec2(-2.5, 3.0),
            c + vec2(-6.0, 3.0),
        ],
        color,
        Stroke::NONE,
    ));

    let stroke = Stroke::new(1.6_f32, color);
    if on {
        // Две "волны"
        for radius in [5.0_f32, 9.0] {
            let points: Vec<Pos2> = (0..=10)
                .map(|i| {
                    let a = (-45.0 + 9.0 * i as f32).to_radians();
                    c + vec2(2.5 + a.cos() * radius, a.sin() * radius)
                })
                .collect();
            p.add(egui::Shape::line(points, stroke));
        }
    } else {
        // Крестик
        let x = c + vec2(9.0, 0.0);
        p.line_segment([x + vec2(-3.5, -3.5), x + vec2(3.5, 3.5)], stroke);
        p.line_segment([x + vec2(-3.5, 3.5), x + vec2(3.5, -3.5)], stroke);
    }

    resp.on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(tooltip);
    enabled_now
}

/// Небольшая кнопка-контур (для предупреждения об админ-правах).
pub fn outline_button(ui: &mut Ui, label: &str, color: Color32) -> bool {
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), semibold(13.0), color);
    let size = vec2(galley.size().x + 28.0, 30.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    let fill = if resp.hovered() {
        color.gamma_multiply(0.18)
    } else {
        Color32::TRANSPARENT
    };
    ui.painter()
        .rect(rect, 8.0, fill, Stroke::new(1.0_f32, color));
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, color);
    resp.on_hover_cursor(CursorIcon::PointingHand)
        .clicked_by(PointerButton::Primary)
}
