//! Процедурная иконка GDS (золотой значок "⏭" на тёмно-синем фоне).
//! Используется и в `build.rs` (ICO для exe), и в рантайме (иконка окна, логотип).
//! Только std — без зависимостей.

/// Отрисовать иконку `size`×`size` в RGBA (straight alpha).
pub fn render(size: u32) -> Vec<u8> {
    const SS: u32 = 4; // суперсэмплинг 4×4 для сглаживания
    let mut out = Vec::with_capacity((size * size * 4) as usize);
    let samples = (SS * SS) as f32;

    for py in 0..size {
        for px in 0..size {
            let (mut r, mut g, mut b, mut a) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
            for sy in 0..SS {
                for sx in 0..SS {
                    let u = (px as f32 + (sx as f32 + 0.5) / SS as f32) / size as f32;
                    let v = (py as f32 + (sy as f32 + 0.5) / SS as f32) / size as f32;
                    let [cr, cg, cb, ca] = sample(u, v);
                    r += cr * ca;
                    g += cg * ca;
                    b += cb * ca;
                    a += ca;
                }
            }
            if a > 0.0 {
                r /= a;
                g /= a;
                b /= a;
            }
            a /= samples;
            out.extend([to_u8(r), to_u8(g), to_u8(b), to_u8(a)]);
        }
    }
    out
}

fn to_u8(x: f32) -> u8 {
    (x.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn mix3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn sample(u: f32, v: f32) -> [f32; 4] {
    const M: f32 = 0.03; // внешний отступ
    const R: f32 = 0.22; // радиус скругления
    const BORDER: f32 = 0.03;

    if !in_round_rect(u, v, M, 1.0 - M, R) {
        return [0.0; 4];
    }

    let gold_dark = [0.62, 0.50, 0.30];
    let gold = [0.83, 0.74, 0.56];
    let gold_light = [0.98, 0.90, 0.70];

    // Золотая рамка
    if !in_round_rect(u, v, M + BORDER, 1.0 - M - BORDER, R - BORDER) {
        let c = mix3(gold_light, gold_dark, v);
        return [c[0], c[1], c[2], 1.0];
    }

    // Значок "⏭"
    if in_glyph(u, v) {
        let c = mix3(gold_light, gold, ((v - 0.3) / 0.4).clamp(0.0, 1.0));
        return [c[0], c[1], c[2], 1.0];
    }

    // Фон: вертикальный градиент + мягкое свечение сверху
    let top = [0.18, 0.20, 0.38];
    let bottom = [0.06, 0.07, 0.14];
    let mut c = mix3(top, bottom, v);
    let d = ((u - 0.5).powi(2) + (v - 0.15).powi(2)).sqrt();
    let glow = (1.0 - d / 0.6).clamp(0.0, 1.0).powi(2) * 0.18;
    c = mix3(c, gold, glow);
    [c[0], c[1], c[2], 1.0]
}

fn in_round_rect(u: f32, v: f32, lo: f32, hi: f32, r: f32) -> bool {
    if u < lo || u > hi || v < lo || v > hi {
        return false;
    }
    let cx = u.clamp(lo + r, hi - r);
    let cy = v.clamp(lo + r, hi - r);
    (u - cx).powi(2) + (v - cy).powi(2) <= r * r
}

fn in_triangle(p: (f32, f32), a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> bool {
    let cross = |o: (f32, f32), x: (f32, f32), y: (f32, f32)| {
        (x.0 - o.0) * (y.1 - o.1) - (x.1 - o.1) * (y.0 - o.0)
    };
    let d1 = cross(a, b, p);
    let d2 = cross(b, c, p);
    let d3 = cross(c, a, p);
    let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(neg && pos)
}

fn in_glyph(u: f32, v: f32) -> bool {
    let p = (u, v);
    let (top, bot, mid) = (0.31, 0.69, 0.50);
    in_triangle(p, (0.24, top), (0.24, bot), (0.47, mid))
        || in_triangle(p, (0.47, top), (0.47, bot), (0.70, mid))
        || (0.70..=0.77).contains(&u) && (top..=bot).contains(&v)
}
