//! Цвета, шрифты и фон в стиле Genshin: тёмно-синий + золото.

use std::fs;

use eframe::egui::{
    self, pos2, Color32, FontData, FontDefinitions, FontFamily, FontId, Mesh, Shape, Stroke,
};

pub const BG_TOP: Color32 = Color32::from_rgb(0x1E, 0x22, 0x42);
pub const BG_BOTTOM: Color32 = Color32::from_rgb(0x0D, 0x0F, 0x1E);
pub const CARD: Color32 = Color32::from_rgb(0x1F, 0x23, 0x40);
pub const CARD_STROKE: Color32 = Color32::from_rgb(0x33, 0x38, 0x60);
pub const HOVER: Color32 = Color32::from_rgb(0x28, 0x2D, 0x52);

pub const GOLD: Color32 = Color32::from_rgb(0xD3, 0xBC, 0x8E);
pub const GOLD_BRIGHT: Color32 = Color32::from_rgb(0xF2, 0xDF, 0xB0);
pub const GOLD_DIM: Color32 = Color32::from_rgb(0x8A, 0x77, 0x52);

pub const TEXT: Color32 = Color32::from_rgb(0xEC, 0xE6, 0xDA);
pub const TEXT_DIM: Color32 = Color32::from_rgb(0x8C, 0x91, 0xB5);
pub const INK: Color32 = Color32::from_rgb(0x17, 0x19, 0x30);

pub const GREEN: Color32 = Color32::from_rgb(0x4A, 0xDE, 0x80);
pub const AMBER: Color32 = Color32::from_rgb(0xF5, 0xB9, 0x42);
pub const RED: Color32 = Color32::from_rgb(0xF2, 0x5F, 0x5C);
pub const RED_BTN: Color32 = Color32::from_rgb(0xC9, 0x45, 0x4A);

pub const TRACK_OFF: Color32 = Color32::from_rgb(0x38, 0x3D, 0x68);
pub const KEY_BG: Color32 = Color32::from_rgb(0x2A, 0x2F, 0x55);
pub const KEY_SHADOW: Color32 = Color32::from_rgb(0x12, 0x14, 0x28);

const BOLD: &str = "bold";
const SEMI: &str = "semibold";

pub fn regular(size: f32) -> FontId {
    FontId::proportional(size)
}

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(SEMI.into()))
}

pub fn bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(BOLD.into()))
}

/// Линейное смешивание цветов (в premultiplied-пространстве, корректно и для прозрачных).
pub fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_premultiplied(
        l(a.r(), b.r()),
        l(a.g(), b.g()),
        l(a.b(), b.b()),
        l(a.a(), b.a()),
    )
}

pub fn setup(ctx: &egui::Context) {
    setup_fonts(ctx);

    let mut v = egui::Visuals::dark();
    v.panel_fill = BG_BOTTOM;
    v.window_fill = CARD;
    v.override_text_color = Some(TEXT);
    v.selection.bg_fill = GOLD_DIM;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, CARD_STROKE);
    ctx.set_visuals(v);
}

/// Segoe UI из системы (красивее и с кириллицей). Если её нет — встроенные шрифты egui.
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let dir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".into()) + "\\Fonts\\";
    let defaults = fonts.families[&FontFamily::Proportional].clone();

    let mut load = |name: &str, file: &str| -> Option<String> {
        let bytes = fs::read(format!("{dir}{file}")).ok()?;
        fonts
            .font_data
            .insert(name.to_owned(), FontData::from_owned(bytes));
        Some(name.to_owned())
    };

    let regular = load("segoe", "segoeui.ttf");
    let semi = load("segoe_semibold", "seguisb.ttf");
    let bold = load("segoe_bold", "segoeuib.ttf");

    let family = |primary: Option<String>| -> Vec<String> {
        primary
            .into_iter()
            .chain(defaults.iter().cloned())
            .collect()
    };

    let semi_family = family(semi.or_else(|| bold.clone()).or_else(|| regular.clone()));
    let bold_family = family(bold.or_else(|| regular.clone()));
    fonts
        .families
        .insert(FontFamily::Proportional, family(regular));
    fonts
        .families
        .insert(FontFamily::Name(SEMI.into()), semi_family);
    fonts
        .families
        .insert(FontFamily::Name(BOLD.into()), bold_family);

    ctx.set_fonts(fonts);
}

/// Градиентный фон с мягким золотым свечением сверху.
pub fn paint_background(ctx: &egui::Context) {
    let rect = ctx.screen_rect();
    let painter = ctx.layer_painter(egui::LayerId::background());

    let mut mesh = Mesh::default();
    mesh.colored_vertex(rect.left_top(), BG_TOP);
    mesh.colored_vertex(rect.right_top(), BG_TOP);
    mesh.colored_vertex(rect.right_bottom(), BG_BOTTOM);
    mesh.colored_vertex(rect.left_bottom(), BG_BOTTOM);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    painter.add(Shape::mesh(mesh));

    // Мягкое радиальное свечение: веер треугольников от полупрозрачного
    // центра к прозрачному краю (цвета premultiplied, без видимых колец).
    let center = pos2(rect.center().x, rect.top() - 30.0);
    let (rx, ry) = (300.0, 210.0);
    let mut glow = Mesh::default();
    glow.colored_vertex(center, Color32::from_rgba_premultiplied(26, 22, 15, 26));
    const SEGMENTS: u32 = 48;
    for i in 0..=SEGMENTS {
        let a = std::f32::consts::TAU * i as f32 / SEGMENTS as f32;
        glow.colored_vertex(
            center + egui::vec2(a.cos() * rx, a.sin() * ry),
            Color32::TRANSPARENT,
        );
        if i > 0 {
            glow.add_triangle(0, i, i + 1);
        }
    }
    painter.add(Shape::mesh(glow));
}
