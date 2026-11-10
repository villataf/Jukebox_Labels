//! Grafiche e palette di colori in stile anni 70, 80 e 90.

use crate::fonts;
use crate::model::{Label, LABEL_H as H, LABEL_W as W};
use crate::scene::*;
use std::f32::consts::{PI, TAU};

pub struct Palette {
    pub id: &'static str,
    pub name: &'static str,
    pub era: &'static str,
    pub bg: Rgb,
    pub band: Rgb,
    pub a1: Rgb,
    pub a2: Rgb,
    pub a3: Rgb,
    /// Titoli sullo sfondo.
    pub text: Rgb,
    /// Testo dentro la fascia centrale.
    pub band_text: Rgb,
    /// Artisti sullo sfondo (etichette con due artisti).
    pub accent: Rgb,
}

macro_rules! pal {
    ($id:expr, $name:expr, $era:expr, $bg:expr, $band:expr, $a1:expr, $a2:expr, $a3:expr, $text:expr, $bt:expr, $acc:expr) => {
        Palette {
            id: $id,
            name: $name,
            era: $era,
            bg: Rgb::hex($bg),
            band: Rgb::hex($band),
            a1: Rgb::hex($a1),
            a2: Rgb::hex($a2),
            a3: Rgb::hex($a3),
            text: Rgb::hex($text),
            band_text: Rgb::hex($bt),
            accent: Rgb::hex($acc),
        }
    };
}

pub static PALETTES: &[Palette] = &[
    pal!("crema", "Crema e Rosso", "Classico", 0xFBF6E9, 0xC8102E, 0x1F1F1F, 0xE8B04B, 0x2C5F8A, 0x1F1F1F, 0xFFFFFF, 0xC8102E),
    pal!("tramonto", "Tramonto", "Anni 70", 0xFFF1D6, 0x8B3A0E, 0xE86A17, 0xF2B134, 0xC0392B, 0x5A2A0A, 0xFFF1D6, 0x8B3A0E),
    pal!("avocado", "Avocado e Senape", "Anni 70", 0xF3EBCF, 0x5E6B2A, 0xC9A227, 0xA0522D, 0x8A9A3B, 0x3D3B1A, 0xF3EBCF, 0x5E6B2A),
    pal!("disco", "Disco Oro", "Anni 70", 0x2B1240, 0xD4AF37, 0xE0457B, 0xF7931E, 0x7B3FA0, 0xFCE9B5, 0x2B1240, 0xF7931E),
    pal!("cioccolato", "Cioccolato e Oro", "Anni 70", 0x6B3A1E, 0xF2C14E, 0xE2001A, 0xFFFFFF, 0xF2C14E, 0xFFFFFF, 0x6B3A1E, 0xF2C14E),
    pal!("arancio", "Arancio e Blu", "Anni 70", 0xFF8A00, 0x1E2A78, 0xFFD400, 0xE2001A, 0xFFFFFF, 0x1E2A78, 0xFFD400, 0xFFFFFF),
    pal!("miami", "Miami Notte", "Anni 80", 0x0E1A40, 0xFF2E88, 0x00E5FF, 0xFFD23F, 0x7A5CFF, 0xFFFFFF, 0xFFFFFF, 0xFF2E88),
    pal!("synth", "Synthwave", "Anni 80", 0x1A0933, 0x7B2CBF, 0xFF3CAC, 0xFF9E00, 0x2DE2E6, 0xFFE3F6, 0xFFFFFF, 0xFF3CAC),
    pal!("memphis", "Memphis", "Anni 80", 0xFFFDF5, 0x111111, 0xFF5FA2, 0x2EC4B6, 0xFFD23F, 0x111111, 0xFFD23F, 0xE6007E),
    pal!("limone", "Limone Fluo", "Anni 80", 0xFFE600, 0xE2001A, 0xFF7A00, 0x00A0E3, 0x14213D, 0x14213D, 0xFFFFFF, 0xE2001A),
    pal!("verdearancio", "Verde e Arancio", "Anni 80", 0x7ED957, 0x1A1A1A, 0xFF8C1A, 0xFFE600, 0xE2001A, 0x1A1A1A, 0xFFFFFF, 0xD6001C),
    pal!("lime", "Lime Acceso", "Anni 80", 0xB4E61E, 0x0B7A3B, 0xFFFFFF, 0xFFE600, 0xE2001A, 0x0B3D1E, 0xFFFFFF, 0x006B2E),
    pal!("fucsia", "Fucsia Pop", "Anni 80", 0xFF4FA3, 0xFFE600, 0x00C2E0, 0xFFFFFF, 0x2B0B3F, 0xFFFFFF, 0x2B0B3F, 0xFFE600),
    pal!("ciano", "Ciano Estate", "Anni 80", 0x2EC5F0, 0xFFE600, 0xE2001A, 0xFFFFFF, 0x0A3D91, 0x0A2A66, 0xE2001A, 0xFFFFFF),
    pal!("confetto", "Rosa Confetto", "Anni 80", 0xFFC9DD, 0x1BA0D8, 0xE4002B, 0xFFE600, 0x111111, 0x111111, 0xFFFFFF, 0xE4002B),
    pal!("festival", "Giallo e Blu", "Anni 80", 0xFFD400, 0x0052A5, 0xE2001A, 0xFFFFFF, 0x0052A5, 0x0B2E6B, 0xFFFFFF, 0xE2001A),
    pal!("notte", "Notte Fluo", "Anni 80", 0x151515, 0x1FA34A, 0xE8202A, 0xFFE600, 0x2B2B2B, 0xFFFFFF, 0xFFFFFF, 0xFFE600),
    pal!("ska", "Bianco e Nero Ska", "Anni 80", 0xFFFFFF, 0x111111, 0x111111, 0xE2001A, 0xFFFFFF, 0x111111, 0xFFFFFF, 0xE2001A),
    pal!("sport", "Giacca a Vento", "Anni 90", 0xF5F5F5, 0x5B2A86, 0x00A99D, 0xFF5F1F, 0xFFD100, 0x1B1B1B, 0xFFFFFF, 0x5B2A86),
    pal!("vapor", "Vaporwave", "Anni 90", 0xFDE2F3, 0x8E7CC3, 0x7FDBDA, 0xFF9AC1, 0xFFF3B0, 0x4B3F72, 0xFFFFFF, 0x6A4FB3),
    pal!("grunge", "Grunge", "Anni 90", 0xE6DCC8, 0x3B4A2F, 0x8C2F1B, 0xC9A227, 0x5B4636, 0x2A2A2A, 0xE6DCC8, 0x8C2F1B),
    pal!("rave", "Rave Acido", "Anni 90", 0x111111, 0xC6FF00, 0xFF00A8, 0x00B3FF, 0xFFFFFF, 0xF2F2F2, 0x111111, 0xC6FF00),
    pal!("gameboy", "Console Verde", "Anni 90", 0x9BBC0F, 0x306230, 0x0F380F, 0x8BAC0F, 0x306230, 0x0F380F, 0x9BBC0F, 0x0F380F),
];

pub struct Design {
    pub id: &'static str,
    pub name: &'static str,
    pub era: &'static str,
    pub palette: &'static str,
    pub font: &'static str,
    build: fn(&mut Scene, &Ctx, &Z),
}

macro_rules! des {
    ($id:expr, $name:expr, $era:expr, $pal:expr, $font:expr, $f:expr) => {
        Design { id: $id, name: $name, era: $era, palette: $pal, font: $font, build: $f }
    };
}

pub static DESIGNS: &[Design] = &[
    des!("classico", "Juke-box Classico", "Classico", "crema", "bebas", classico),
    des!("foto", "Foto Quadrata", "Classico", "confetto", "archivo", foto),
    des!("raggi", "Raggi di Sole", "Anni 70", "arancio", "bowlby", raggi),
    des!("tramonto", "Strisce Tramonto", "Anni 70", "tramonto", "shrikhand", tramonto),
    des!("groovy", "Onda Groovy", "Anni 70", "avocado", "righteous", groovy),
    des!("arcobaleno", "Arcobaleno Diagonale", "Anni 70", "disco", "pacifico", arcobaleno),
    des!("disco", "Disco Volante", "Anni 70", "fucsia", "titan", disco),
    des!("arlecchino", "Arlecchino", "Anni 70", "arancio", "alfaslab", arlecchino),
    des!("doppio", "Doppio Colore", "Anni 80", "verdearancio", "fjalla", doppio),
    des!("vela", "Vela d'Estate", "Anni 80", "limone", "anton", vela),
    des!("ruota", "Ruota della Fortuna", "Anni 80", "notte", "archivo", ruota),
    des!("biglietto", "Biglietto Perforato", "Anni 80", "confetto", "russo", biglietto),
    des!("pannello", "Pannello Juke-box", "Anni 80", "lime", "squada", pannello),
    des!("festival", "Festival Stelle", "Anni 80", "festival", "passion", festival),
    des!("ska", "Scacchi Ska", "Anni 80", "ska", "anton", ska),
    des!("lampo", "Lampo Pop", "Anni 80", "ciano", "bangers", lampo),
    des!("neon", "Griglia Neon", "Anni 80", "synth", "monoton", neon),
    des!("memphis", "Coriandoli Memphis", "Anni 80", "memphis", "bungee", memphis),
    des!("cromo", "Cromo Lucido", "Anni 80", "miami", "audiowide", cromo),
    des!("arcade", "Sala Giochi 8-bit", "Anni 80", "gameboy", "pressstart", arcade),
    des!("rave", "Zig-zag Rave", "Anni 90", "rave", "bangers", rave),
    des!("grunge", "Nastro Grunge", "Anni 90", "grunge", "marker", grunge),
    des!("vapor", "Scacchi Vapor", "Anni 90", "vapor", "vt323", vapor),
    des!("sport", "Giacca a Vento", "Anni 90", "sport", "russo", sport),
];

pub fn design_index(id: &str) -> usize {
    DESIGNS.iter().position(|d| d.id == id).unwrap_or(0)
}

pub fn palette_index(id: &str) -> usize {
    PALETTES.iter().position(|p| p.id == id).unwrap_or(0)
}

pub struct Ctx<'a> {
    pub pal: &'a Palette,
    /// Carattere degli artisti e dei titoli.
    pub font: usize,
    pub font_titles: usize,
    pub artist: String,
    pub artist_b: String,
    pub title_a: String,
    pub title_b: String,
    pub slogan: String,
    pub image: Option<std::sync::Arc<crate::images::Img>>,
}

/// Le zone dell'etichetta: la fascia centrale è più sottile quando ci sono due artisti.
pub struct Z {
    pub double: bool,
    pub b0: f32,
    pub b1: f32,
}

/// Disegna l'etichetta completa a 78 × 25 mm.
pub fn render_label(l: &Label) -> Scene {
    let fix = |s: &str| if l.uppercase { s.trim().to_uppercase() } else { s.trim().to_string() };
    let font = fonts::index_of(&l.font);
    let ctx = Ctx {
        pal: &PALETTES[palette_index(&l.palette)],
        font,
        font_titles: if l.font_titles.is_empty() { font } else { fonts::index_of(&l.font_titles) },
        artist: fix(&l.artist),
        artist_b: if l.two_artists { fix(&l.artist_b) } else { String::new() },
        title_a: fix(&l.title_a),
        title_b: fix(&l.title_b),
        slogan: if l.two_artists { l.slogan.trim().to_uppercase() } else { String::new() },
        image: if l.design == "foto" { crate::images::load(&l.image) } else { None },
    };
    let z = if l.two_artists { Z { double: true, b0: 10.75, b1: 14.25 } } else { Z { double: false, b0: 9.2, b1: 15.8 } };
    let mut s = Scene::new(W, H);
    s.fill(rect(0.0, 0.0, W, H), ctx.pal.bg);
    (DESIGNS[design_index(&l.design)].build)(&mut s, &ctx, &z);
    s
}

/// Campione di un carattere: il suo nome scritto con lui stesso.
pub fn render_font_sample(font: usize, pal: &Palette) -> Scene {
    let (w, h) = (52.0, 11.0);
    let mut s = Scene::new(w, h);
    s.fill(rect(0.0, 0.0, w, h), pal.bg);
    s.fill(rect(0.0, h - 1.2, w, 1.2), pal.band);
    let t = TextSpec { cx: w / 2.0, cy: 4.9, cap: 3.6, max_w: w - 4.0, spacing: 0.0, color: pal.text, fx: Fx::None };
    place_text(&mut s, font, fonts::FONTS[font].name, &t, pal.bg);
    s
}

// ---------------------------------------------------------------- impaginazione comune

struct Lay {
    /// Centro orizzontale dei testi (si sposta quando c'è un pannello a sinistra).
    cx: f32,
    top: f32,
    art_a: f32,
    mid: f32,
    art_b: f32,
    bot: f32,
    title_cap: f32,
    artist_cap: f32,
    title_w: f32,
    artist_w: f32,
    spacing: f32,
    title_col: Rgb,
    /// Artista nella fascia (un solo artista) o scritta della fascia (due artisti).
    band_col: Rgb,
    /// Artisti fuori dalla fascia (due artisti).
    artist_col: Rgb,
    lower_title_col: Rgb,
    lower_artist_col: Rgb,
    title_fx: Fx,
    band_fx: Fx,
    artist_fx: Fx,
}

impl Lay {
    fn new(c: &Ctx, z: &Z) -> Lay {
        let p = c.pal;
        let mid = (z.b0 + z.b1) / 2.0;
        let mut l = Lay {
            cx: W / 2.0,
            top: 4.9,
            art_a: 7.6,
            mid,
            art_b: 17.4,
            bot: 20.1,
            title_cap: 2.8,
            artist_cap: 3.2,
            title_w: 68.0,
            artist_w: 64.0,
            spacing: 0.02,
            title_col: p.text,
            band_col: p.band_text,
            artist_col: p.accent,
            lower_title_col: p.text,
            lower_artist_col: p.accent,
            title_fx: Fx::None,
            band_fx: Fx::None,
            artist_fx: Fx::None,
        };
        if z.double {
            l.top = 3.1;
            l.bot = 21.95;
            l.title_cap = 1.9;
            l.artist_cap = 2.7;
            l.artist_w = 66.0;
        }
        l
    }

    /// Lascia spazio a un pannello illustrato largo `w` mm sul lato sinistro.
    fn panel(&mut self, w: f32) {
        self.cx = (w + W) / 2.0;
        let room = W - w - 4.0;
        self.title_w = self.title_w.min(room);
        self.artist_w = self.artist_w.min(room);
    }

    fn lower(&mut self, c: Rgb) {
        self.lower_title_col = c;
        self.lower_artist_col = c;
    }
}

fn texts(s: &mut Scene, c: &Ctx, z: &Z, l: &Lay, band_bg: Rgb) {
    let mk = |cy, cap, max_w, color, fx| TextSpec { cx: l.cx, cy, cap, max_w, spacing: l.spacing, color, fx };
    let bg = c.pal.bg;
    let ft = c.font_titles;
    place_text(s, ft, &c.title_a, &mk(l.top, l.title_cap, l.title_w, l.title_col, l.title_fx), bg);
    place_text(s, ft, &c.title_b, &mk(l.bot, l.title_cap, l.title_w, l.lower_title_col, l.title_fx), bg);
    if !z.double {
        place_text(s, c.font, &c.artist, &mk(l.mid, l.artist_cap, l.artist_w, l.band_col, l.band_fx), band_bg);
        return;
    }
    place_text(s, c.font, &c.artist, &mk(l.art_a, l.artist_cap, l.artist_w, l.artist_col, l.artist_fx), bg);
    place_text(s, c.font, &c.artist_b, &mk(l.art_b, l.artist_cap, l.artist_w, l.lower_artist_col, l.artist_fx), bg);
    if !c.slogan.is_empty() {
        // come sulle etichette d'epoca: "• SCRITTA • SCRITTA •" quando c'è posto
        let cap = ((z.b1 - z.b0) * 0.42).min(1.5);
        let size = cap / cap_ratio(ft);
        let twice = format!("•  {0}  •  {0}  •", c.slogan);
        let text = if measure(ft, &twice, 0.04) * size <= l.artist_w { twice } else { format!("• {} •", c.slogan) };
        let spec = TextSpec { cx: l.cx, cy: l.mid, cap, max_w: l.artist_w, spacing: 0.04, color: l.band_col, fx: Fx::None };
        place_text(s, ft, &text, &spec, band_bg);
    }
}

// ---------------------------------------------------------------- grafiche classiche

fn classico(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    s.fill(rect(0.0, 0.0, W, 0.9), p.band);
    s.fill(rect(0.0, 1.4, W, 0.35), p.band);
    s.fill(rect(0.0, H - 0.9, W, 0.9), p.band);
    s.fill(rect(0.0, H - 1.75, W, 0.35), p.band);
    let (y0, y1, m) = (z.b0 + 0.1, z.b1 - 0.1, (z.b0 + z.b1) / 2.0);
    s.fill(polygon(&[(4.0, y0), (W - 4.0, y0), (W - 1.0, m), (W - 4.0, y1), (4.0, y1), (1.0, m)]), p.band);
    let r = if z.double { 1.1 } else { 1.5 };
    s.fill(star(4.6, m, r, r * 0.4, 5), p.a2);
    s.fill(star(W - 4.6, m, r, r * 0.4, 5), p.a2);
    let mut l = Lay::new(c, z);
    l.artist_w = 60.0;
    if z.double {
        l.top = 3.6;
        l.bot = 21.4;
    }
    texts(s, c, z, &l, p.band);
}

fn tramonto(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let t = if z.double { 0.55 } else { 1.0 };
    s.fill(rect(0.0, z.b0 - 2.0 * t, W, t), p.a2);
    s.fill(rect(0.0, z.b0 - t, W, t), p.a1);
    s.fill(rect(0.0, z.b0, W, z.b1 - z.b0), p.band);
    s.fill(rect(0.0, z.b1, W, t), p.a1);
    s.fill(rect(0.0, z.b1 + t, W, t), p.a2);
    let r = if z.double { 1.8 } else { 2.6 };
    let base = z.b0 - 2.0 * t;
    for &cx in &[3.6, W - 3.6] {
        s.fill(half_disc(cx, base, r), p.a1);
        s.fill(rect(cx - r - 0.1, base - r * 0.62, 2.0 * r + 0.2, 0.3), p.bg);
        s.fill(rect(cx - r - 0.1, base - r * 0.3, 2.0 * r + 0.2, 0.3), p.bg);
    }
    s.stroke(rrect(0.7, 0.7, W - 1.4, H - 1.4, 2.2), p.a1, 0.35);
    let mut l = Lay::new(c, z);
    l.title_w = 62.0;
    if !z.double {
        l.top = 4.4;
        l.bot = 20.6;
        l.artist_cap = 2.7;
    } else {
        l.artist_w = 60.0;
    }
    texts(s, c, z, &l, p.band);
}

fn wave(y: f32, amp: f32) -> Vec<(f32, f32)> {
    (0..=78).map(|i| (i as f32, y + amp * (i as f32 / 19.5 * TAU).sin())).collect()
}

fn groovy(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let amp = if z.double { 0.5 } else { 0.8 };
    let mut band = wave(z.b0 - 0.2, amp);
    let mut bottom = wave(z.b1 + 0.2, amp);
    bottom.reverse();
    band.extend(bottom);
    s.fill(polygon(&band), p.band);
    if !z.double {
        s.stroke(polyline(&wave(z.b0 - 1.6, amp)), p.a1, 0.5);
        s.stroke(polyline(&wave(z.b1 + 1.6, amp)), p.a2, 0.5);
    }
    let m = (z.b0 + z.b1) / 2.0;
    let k = if z.double { 0.75 } else { 1.0 };
    for &cx in &[5.0, W - 5.0] {
        for i in 0..5 {
            let a = i as f32 / 5.0 * TAU;
            s.fill(circle(cx + 1.25 * k * a.cos(), m + 1.25 * k * a.sin(), 0.95 * k), p.a1);
        }
        s.fill(circle(cx, m, 0.8 * k), p.a2);
    }
    let mut l = Lay::new(c, z);
    l.artist_w = 56.0;
    if !z.double {
        l.top = 4.0;
        l.bot = 21.1;
        l.artist_cap = 3.0;
    }
    texts(s, c, z, &l, p.band);
}

fn arcobaleno(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let cols = [p.a2, p.a1, p.band, p.a3];
    let w = if z.double { 1.2 } else { 1.7 };
    for (i, &col) in cols.iter().enumerate() {
        let d0 = 1.6 + i as f32 * w;
        let d1 = d0 + w;
        s.fill(polygon(&[(0.0, d0), (d0 * 1.7, 0.0), (d1 * 1.7, 0.0), (0.0, d1)]), col);
        s.fill(polygon(&[(W, H - d0), (W - d0 * 1.7, H), (W - d1 * 1.7, H), (W, H - d1)]), col);
    }
    let m = (z.b0 + z.b1) / 2.0;
    s.fill(rect(6.0, z.b0 - 0.2, W - 12.0, 0.4), p.band);
    s.fill(rect(6.0, z.b1 - 0.2, W - 12.0, 0.4), p.band);
    s.fill(circle(4.5, m, 0.6), p.a1);
    s.fill(circle(W - 4.5, m, 0.6), p.a1);
    let mut l = Lay::new(c, z);
    l.title_w = 56.0;
    l.band_col = if z.double { p.text } else { p.band };
    l.artist_col = p.band;
    l.lower_artist_col = p.band;
    l.artist_w = 58.0;
    if !z.double {
        l.artist_cap = 3.4;
        l.band_fx = Fx::Shadow(p.a1, 0.35, 0.35);
    } else {
        l.artist_fx = Fx::Shadow(p.a1, 0.3, 0.3);
    }
    texts(s, c, z, &l, p.bg);
}

// ---------------------------------------------------------------- grafiche nuove, colori accesi

/// Disco in vinile con note musicali su un pannello a sinistra.
fn disco(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let pw = 18.0;
    s.fill(rect(0.0, 0.0, pw, H), p.a1);
    let (cx, cy) = (9.0, 12.5);
    s.fill(circle(cx, cy, 7.4), Rgb::hex(0x111111));
    for r in [6.6f32, 5.8, 5.0, 4.2] {
        s.stroke(circle(cx, cy, r), Rgb::hex(0x3A3A3A), 0.18);
    }
    s.stroke(polyline(&[(cx - 5.2, cy - 4.0), (cx - 3.2, cy - 5.6)]), Rgb::hex(0x8A8A8A), 0.35);
    s.fill(circle(cx, cy, 2.5), p.band);
    s.fill(circle(cx, cy, 0.45), Rgb::hex(0x111111));
    let note = |s: &mut Scene, x: f32, y: f32, col: Rgb| {
        s.fill(ellipse(x, y, 0.75, 0.55), col);
        s.stroke(line(x + 0.68, y, x + 0.68, y - 2.6), col, 0.25);
        s.stroke(polyline(&[(x + 0.68, y - 2.6), (x + 1.6, y - 1.9)]), col, 0.3);
    };
    note(s, 2.2, 3.4, p.a2);
    note(s, 14.4, 22.8, p.a2);
    note(s, 15.0, 4.0, p.band);
    s.fill(rect(pw, 0.0, 0.6, H), p.a3);
    s.fill(rect(pw + 0.6, z.b0, W - pw, z.b1 - z.b0), p.band);
    s.fill(rect(pw + 0.6, z.b0 - 0.5, W - pw, 0.25), p.a2);
    s.fill(rect(pw + 0.6, z.b1 + 0.25, W - pw, 0.25), p.a2);
    let mut l = Lay::new(c, z);
    l.panel(pw + 0.6);
    l.title_fx = Fx::Shadow(p.a3, 0.3, 0.3);
    l.artist_fx = Fx::Shadow(p.a3, 0.3, 0.3);
    texts(s, c, z, &l, p.band);
}

/// Rombi colorati da Arlecchino a sinistra e coriandoli sul resto.
fn arlecchino(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let pw = 16.0;
    let cols = [p.a1, p.a2, p.band, p.a3];
    let (dw, dh) = (16.0 / 3.0, 25.0 / 4.0);
    for row in 0..9 {
        for col in 0..4 {
            let cx = col as f32 * dw + if row % 2 == 0 { 0.0 } else { dw / 2.0 };
            let cy = row as f32 * dh / 2.0;
            if cx > pw {
                continue;
            }
            let color = cols[(row + col * 2) % 4];
            let pts = [(cx, cy - dh / 2.0), (cx + dw / 2.0, cy), (cx, cy + dh / 2.0), (cx - dw / 2.0, cy)];
            let pts: Vec<(f32, f32)> = pts.iter().map(|&(x, y)| (x.clamp(0.0, pw), y)).collect();
            s.fill(polygon(&pts), color);
        }
    }
    s.fill(rect(pw, 0.0, 0.7, H), p.a3);
    let mut r = Rand::new(11);
    let dots = [p.a1, p.a2, p.band, p.a3];
    for i in 0..40 {
        let x = r.range(pw + 2.0, W - 1.0);
        let y = if i % 2 == 0 { r.range(0.6, z.b0 - 0.8) } else { r.range(z.b1 + 0.8, H - 0.6) };
        s.fill(circle(x, y, r.range(0.35, 0.7)), dots[i % 4].mix(p.bg, 0.55));
    }
    s.fill(rect(pw + 0.7, z.b0, W - pw, z.b1 - z.b0), p.band);
    let mut l = Lay::new(c, z);
    l.panel(pw + 0.7);
    texts(s, c, z, &l, p.band);
}

/// Metà superiore di un colore, metà inferiore di un altro, fascia scura nel mezzo.
fn doppio(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let m = (z.b0 + z.b1) / 2.0;
    s.fill(rect(0.0, m, W, H - m), p.a1);
    s.fill(rect(0.0, z.b0, W, z.b1 - z.b0), p.band);
    s.fill(rect(0.0, z.b0 - 0.6, W, 0.3), p.band);
    s.fill(rect(0.0, z.b1 + 0.3, W, 0.3), p.band);
    let mut l = Lay::new(c, z);
    if z.double {
        l.artist_col = p.a3;
        l.lower_artist_col = p.a3;
    }
    texts(s, c, z, &l, p.band);
}

/// Pannello estivo a sinistra: sole, mare e una barca a vela.
fn vela(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let pw = 18.0;
    let sky = p.a2.mix(Rgb::WHITE, 0.55);
    s.fill(rect(0.0, 0.0, pw, H), sky);
    s.fill(circle(13.5, 5.0, 2.8), p.a1);
    s.fill(circle(13.5, 5.0, 2.0), p.bg);
    // mare a onde
    let mut sea: Vec<(f32, f32)> = (0..=18).map(|i| (i as f32, 15.2 + 0.45 * (i as f32 * 1.4).sin())).collect();
    sea.push((pw, H));
    sea.push((0.0, H));
    s.fill(polygon(&sea), p.a2);
    for k in 0..3 {
        let y = 18.0 + k as f32 * 2.2;
        let pts: Vec<(f32, f32)> = (0..=18).map(|i| (i as f32, y + 0.35 * (i as f32 * 1.4 + k as f32).sin())).collect();
        s.stroke(polyline(&pts), Rgb::WHITE, 0.25);
    }
    // barca
    s.fill(polygon(&[(3.0, 14.2), (12.5, 14.2), (11.0, 15.9), (4.4, 15.9)]), p.a3);
    s.stroke(line(7.6, 4.2, 7.6, 14.2), p.a3, 0.3);
    s.fill(polygon(&[(7.9, 4.4), (7.9, 13.6), (12.6, 13.6)]), Rgb::WHITE);
    s.fill(polygon(&[(7.3, 6.0), (7.3, 13.6), (3.6, 13.6)]), p.band);
    // gabbiano
    s.stroke(polyline(&[(2.0, 4.0), (2.8, 3.4), (3.5, 4.0), (4.2, 3.4), (5.0, 4.0)]), p.a3, 0.22);
    s.fill(rect(pw, 0.0, 0.5, H), p.a3);
    s.fill(rect(pw + 0.5, z.b0, W - pw, z.b1 - z.b0), p.band);
    let mut l = Lay::new(c, z);
    l.panel(pw + 0.5);
    texts(s, c, z, &l, p.band);
}

/// Mezza ruota della roulette a sinistra, su fondo scuro.
fn ruota(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let (cx, cy, r) = (-3.0, 12.5, 18.5);
    let n = 14;
    for i in 0..n {
        let a0 = -PI / 2.0 + PI * i as f32 / n as f32;
        let a1 = a0 + PI / n as f32;
        let mut pts = vec![(cx, cy)];
        for k in 0..=6 {
            let a = a0 + (a1 - a0) * k as f32 / 6.0;
            pts.push((cx + r * a.cos(), cy + r * a.sin()));
        }
        let col = if i == n / 2 { p.band } else if i % 2 == 0 { p.a1 } else { p.a3 };
        s.fill(polygon(&pts), col);
    }
    s.stroke(circle(cx, cy, r - 0.4), p.a2, 0.35);
    s.fill(circle(cx, cy, 9.0), p.a2.mix(p.bg, 0.25));
    s.fill(circle(cx, cy, 7.5), p.bg);
    for i in 0..8 {
        let a = -PI / 2.0 + PI * (i as f32 + 0.5) / 8.0;
        s.stroke(line(cx, cy, cx + 7.5 * a.cos(), cy + 7.5 * a.sin()), p.a2, 0.3);
    }
    s.fill(circle(cx, cy, 3.0), p.a2);
    let pw = cx + r + 0.4;
    s.fill(rect(pw, z.b0, W - pw, z.b1 - z.b0), p.band);
    let mut l = Lay::new(c, z);
    l.panel(pw);
    l.title_col = p.a1;
    l.lower_title_col = p.a2;
    l.artist_col = Rgb::WHITE;
    l.lower_artist_col = Rgb::WHITE;
    texts(s, c, z, &l, p.band);
}

/// Biglietto con i bordi perforati e due righe sottili attorno alla fascia.
fn biglietto(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    // dentini sopra e sotto, del colore della carta
    let step = 1.3;
    let mut x = 0.0;
    while x < W {
        s.fill(polygon(&[(x, 0.0), (x + step / 2.0, 0.55), (x + step, 0.0)]), Rgb::WHITE);
        s.fill(polygon(&[(x, H), (x + step / 2.0, H - 0.55), (x + step, H)]), Rgb::WHITE);
        x += step;
    }
    if z.double {
        s.fill(rect(4.0, z.b0 + 0.2, W - 8.0, 0.3), p.a3);
        s.fill(rect(4.0, z.b1 - 0.5, W - 8.0, 0.3), p.a3);
    } else {
        s.fill(rrect(3.0, z.b0, W - 6.0, z.b1 - z.b0, 1.0), p.band);
    }
    let mut l = Lay::new(c, z);
    l.artist_w = 62.0;
    if z.double {
        l.band_col = p.a3;
    }
    texts(s, c, z, &l, if z.double { p.bg } else { p.band });
}

/// Pannello bianco con un piccolo juke-box stilizzato a sinistra.
fn pannello(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let pw = 15.0;
    s.fill(rect(0.0, 0.0, pw, H), Rgb::WHITE);
    s.stroke(rect(0.8, 0.8, pw - 1.6, H - 1.6), p.band, 0.3);
    // juke-box: corpo ad arco, archi colorati, griglia
    let (jx, jw) = (3.0, 9.0);
    s.fill(rrect(jx, 6.0, jw, 15.5, 0.8), p.band);
    s.fill(half_disc(jx + jw / 2.0, 10.0, jw / 2.0), p.band);
    for (i, col) in [p.a2, p.a3, p.a1].iter().enumerate() {
        s.fill(half_disc(jx + jw / 2.0, 10.6, jw / 2.0 - 1.0 - i as f32 * 0.9), *col);
    }
    s.fill(half_disc(jx + jw / 2.0, 10.6, 1.0), p.band);
    s.fill(rect(jx + 1.0, 11.6, jw - 2.0, 3.0), p.a1);
    for k in 0..4 {
        s.fill(rect(jx + 1.3, 16.0 + k as f32 * 1.1, jw - 2.6, 0.45), p.bg);
    }
    s.fill(rect(pw, z.b0, W - pw, z.b1 - z.b0), p.band);
    s.fill(rect(pw, z.b1, W - pw, 0.4), p.a2);
    let mut l = Lay::new(c, z);
    l.panel(pw);
    texts(s, c, z, &l, p.band);
}

/// Due colori forti, stelle che separano le righe.
fn festival(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let m = (z.b0 + z.b1) / 2.0;
    s.fill(rect(0.0, z.b1, W, H - z.b1), p.band);
    s.fill(rect(0.0, z.b0, W, z.b1 - z.b0), p.a1);
    let r = (z.b1 - z.b0) * 0.36;
    s.fill(star(3.2, m, r, r * 0.42, 5), p.a2);
    s.fill(star(W - 3.2, m, r, r * 0.42, 5), p.a2);
    s.fill(star(3.0, 4.6, 1.3, 0.55, 5), p.a1);
    s.fill(star(W - 3.0, 4.6, 1.3, 0.55, 5), p.a1);
    s.fill(star(3.0, 20.4, 1.3, 0.55, 5), p.a2);
    s.fill(star(W - 3.0, 20.4, 1.3, 0.55, 5), p.a2);
    let mut l = Lay::new(c, z);
    l.title_w = 62.0;
    l.artist_w = 62.0;
    l.lower(Rgb::WHITE);
    l.band_col = p.a2;
    texts(s, c, z, &l, p.a1);
}

// ---------------------------------------------------------------- anni 80 e 90

fn neon(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let (hz, y1) = (z.b0 - 0.2, z.b1 + 0.6);
    s.fill(rect(0.0, hz, W, y1 - hz), p.band.mix(p.bg, 0.55));
    for i in 0..6 {
        let t = (i as f32 / 5.0).powf(1.8);
        let y = hz + (y1 - hz) * t;
        s.stroke(line(0.0, y, W, y), p.a1, 0.16);
    }
    let vx = W / 2.0;
    for i in -12..=12 {
        let xb = vx + i as f32 * 6.5;
        let t = (y1 - hz) / (y1 - (hz - 6.0));
        let xt = vx + (xb - vx) * (1.0 - t);
        s.stroke(line(xt, hz, xb, y1), p.a1, 0.16);
    }
    let r = if z.double { 2.0 } else { 3.0 };
    for &cx in &[5.0, W - 5.0] {
        s.fill(half_disc(cx, hz, r), p.a2);
        s.fill(rect(cx - r - 0.1, hz - r * 0.47, 2.0 * r + 0.2, 0.3), p.bg);
        s.fill(rect(cx - r - 0.1, hz - r * 0.23, 2.0 * r + 0.2, 0.35), p.bg);
    }
    s.fill(rect(0.0, 0.6, W, 0.3), p.a1);
    s.fill(rect(0.0, H - 0.9, W, 0.3), p.a1);
    let mut l = Lay::new(c, z);
    l.band_col = Rgb::WHITE;
    l.title_fx = Fx::Glow(p.a1, 0.22);
    l.band_fx = Fx::Glow(p.a1, 0.45);
    l.artist_fx = Fx::Glow(p.a2, 0.3);
    if !z.double {
        l.top = 4.6;
        l.mid = 12.8;
        l.bot = 20.6;
        l.title_w = 64.0;
    } else {
        l.title_w = 60.0;
        l.artist_w = 58.0;
        l.artist_col = Rgb::WHITE;
        l.lower_artist_col = Rgb::WHITE;
    }
    texts(s, c, z, &l, p.bg);
}

fn memphis(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let mut r = Rand::new(7);
    let cols = [p.a1, p.a2, p.a3];
    for i in 0..14 {
        let left = i % 2 == 0;
        let x = if left { r.range(0.8, 6.5) } else { r.range(W - 6.5, W - 0.8) };
        let y = r.range(1.0, H - 1.0);
        let col = cols[i % 3];
        match i % 4 {
            0 => s.fill(circle(x, y, 0.7), col),
            1 => s.fill(polygon(&[(x, y - 0.9), (x + 0.9, y + 0.7), (x - 0.9, y + 0.7)]), col),
            2 => s.stroke(polyline(&[(x - 1.2, y), (x - 0.6, y - 0.6), (x, y), (x + 0.6, y - 0.6), (x + 1.2, y)]), col, 0.3),
            _ => s.fill(rect(x - 0.5, y - 0.5, 1.0, 1.0), col),
        }
    }
    let (y0, h) = (z.b0 + 0.2, z.b1 - z.b0 - 0.4);
    s.fill(rect(8.6, y0 + 0.6, W - 16.0, h), p.a1);
    s.fill(rect(8.0, y0, W - 16.0, h), p.band);
    let mut l = Lay::new(c, z);
    l.title_w = 60.0;
    l.artist_w = 56.0;
    if !z.double {
        l.mid = 12.3;
        l.artist_cap = 2.9;
        for i in 0..3 {
            s.fill(circle(9.6, 10.4 + i as f32 * 1.9, 0.3), p.a2);
            s.fill(circle(W - 9.6, 10.4 + i as f32 * 1.9, 0.3), p.a2);
        }
    }
    texts(s, c, z, &l, p.band);
}

fn cromo(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let (y0, y1) = (z.b0 - 0.2, z.b1 + 0.2);
    let n = 14;
    let light = p.band.mix(Rgb::WHITE, 0.75);
    let dark = p.band.mix(Rgb::BLACK, 0.35);
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let k = if t < 0.5 { t * 2.0 } else { 0.25 + (1.0 - t) * 1.2 };
        let col = light.mix(dark, k.clamp(0.0, 1.0));
        let h = (y1 - y0) / n as f32;
        s.fill(rect(0.0, y0 + h * i as f32, W, h + 0.05), col);
    }
    s.fill(rect(0.0, y0, W, 0.25), Rgb::WHITE);
    s.fill(rect(0.0, y1 - 0.25, W, 0.25), p.a1);
    for (j, &y) in [0.0f32, H - 1.4].iter().enumerate() {
        for i in 0..4 {
            let col = if (i + j) % 2 == 0 { p.a1 } else { p.a1.mix(Rgb::WHITE, 0.5) };
            s.fill(rect(0.0, y + i as f32 * 0.35, W, 0.36), col);
        }
    }
    let mut l = Lay::new(c, z);
    l.band_col = Rgb::WHITE;
    l.band_fx = Fx::Outline(dark.mix(Rgb::BLACK, 0.4), 0.5);
    l.title_fx = Fx::Shadow(p.band, 0.3, 0.3);
    l.artist_fx = Fx::Shadow(p.a1.mix(p.bg, 0.4), 0.3, 0.3);
    if !z.double {
        l.top = 5.2;
        l.bot = 19.9;
    } else {
        l.top = 3.5;
        l.bot = 21.5;
    }
    texts(s, c, z, &l, dark);
}

fn arcade(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let px = if z.double { 0.9 } else { 1.3 };
    let cols = (W / px).ceil() as usize;
    for i in 0..cols {
        s.fill(rect(i as f32 * px, 0.0, px, px), if i % 2 == 0 { p.band } else { p.a1 });
        s.fill(rect(i as f32 * px, H - px, px, px), if i % 2 == 0 { p.a1 } else { p.band });
    }
    let (y0, y1) = (z.b0 - 0.1, z.b1 + 0.1);
    let st = ((y1 - y0) / 5.0).min(1.3);
    s.fill(rect(3.0 * st, y0, W - 6.0 * st, y1 - y0), p.band);
    s.fill(rect(2.0 * st, y0 + st, W - 4.0 * st, y1 - y0 - 2.0 * st), p.band);
    s.fill(rect(st, y0 + 2.0 * st, W - 2.0 * st, (y1 - y0 - 4.0 * st).max(0.1)), p.band);
    let mut l = Lay::new(c, z);
    l.spacing = 0.06;
    l.title_w = 64.0;
    l.artist_w = 66.0;
    if !z.double {
        for &x in &[3.2 * px, W - 4.2 * px] {
            s.fill(rect(x, 4.3, px * 0.6, px * 0.6), p.a1);
            s.fill(rect(x, 19.6, px * 0.6, px * 0.6), p.a1);
        }
        l.top = 5.0;
        l.bot = 20.0;
        l.title_cap = 2.4;
        l.artist_cap = 2.8;
    } else {
        l.title_cap = 1.7;
        l.artist_cap = 2.3;
    }
    texts(s, c, z, &l, p.band);
}

fn rave(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let zz = if z.double { 0.6 } else { 0.9 };
    let (y0, y1) = (z.b0 + 0.2, z.b1 - 0.2);
    let n = 26;
    let step = W / n as f32;
    let mut pts = Vec::new();
    for i in 0..=n {
        pts.push((i as f32 * step, if i % 2 == 0 { y0 } else { y0 - zz }));
    }
    for i in (0..=n).rev() {
        pts.push((i as f32 * step, if i % 2 == 0 { y1 } else { y1 + zz }));
    }
    s.fill(polygon(&pts), p.band);
    let q = 1.25;
    for row in 0..20 {
        for colm in 0..2 {
            if (row + colm) % 2 == 0 {
                let y = row as f32 * q;
                if y < y0 - zz - 0.3 || y > y1 + zz + 0.3 - q {
                    s.fill(rect(colm as f32 * q, y, q, q), p.a1);
                    s.fill(rect(W - (colm + 1) as f32 * q, y, q, q), p.a2);
                }
            }
        }
    }
    let m = (z.b0 + z.b1) / 2.0;
    let k = if z.double { 0.7 } else { 1.0 };
    for &x in &[5.4, W - 5.4] {
        s.fill(circle(x, m, 1.6 * k), p.a2);
        s.fill(circle(x - 0.55 * k, m - 0.5 * k, 0.22 * k), p.bg);
        s.fill(circle(x + 0.55 * k, m - 0.5 * k, 0.22 * k), p.bg);
        s.stroke(
            polyline(&[(x - 0.8 * k, m + 0.4 * k), (x - 0.3 * k, m + 0.9 * k), (x + 0.3 * k, m + 0.9 * k), (x + 0.8 * k, m + 0.4 * k)]),
            p.bg,
            0.22 * k,
        );
    }
    let mut l = Lay::new(c, z);
    l.title_w = 62.0;
    l.artist_w = 58.0;
    l.title_fx = Fx::Shadow(p.a1, 0.4, 0.4);
    l.artist_fx = Fx::Shadow(p.a2, 0.35, 0.35);
    if !z.double {
        l.top = 4.4;
        l.bot = 20.6;
        l.artist_w = 56.0;
    }
    texts(s, c, z, &l, p.band);
}

fn grunge(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let mut r = Rand::new(31);
    for _ in 0..22 {
        let (x, y) = (r.range(0.5, W - 0.5), r.range(0.5, H - 0.5));
        if (z.b0 - 1.0..z.b1 + 1.0).contains(&y) {
            continue;
        }
        s.fill(circle(x, y, r.range(0.08, 0.35)), p.a3.mix(p.bg, 0.45));
    }
    let mut top = Vec::new();
    let mut bot = Vec::new();
    let n = 40;
    for i in 0..=n {
        let x = -1.0 + (W + 2.0) * i as f32 / n as f32;
        let tilt = (x - W / 2.0) * -0.025;
        top.push((x, z.b0 + tilt + r.range(-0.25, 0.25)));
        bot.push((x, z.b1 + tilt + r.range(-0.25, 0.25)));
    }
    bot.reverse();
    top.extend(bot);
    s.fill(polygon(&top), p.band);
    for _ in 0..6 {
        let x = r.range(2.0, W - 8.0);
        let y = r.range(z.b0 + 0.8, z.b1 - 0.8);
        s.stroke(line(x, y, x + r.range(2.0, 6.0), y + r.range(-0.4, 0.4)), p.band.mix(p.band_text, 0.25), 0.12);
    }
    s.stroke(polyline(&[(1.2, 1.5), (W - 1.5, 1.1), (W - 1.0, H - 1.4), (1.0, H - 1.0), (1.2, 1.5)]), p.a1, 0.35);
    let mut l = Lay::new(c, z);
    l.title_w = 64.0;
    if !z.double {
        l.top = 5.0;
        l.bot = 20.2;
    } else {
        l.top = 3.6;
        l.bot = 21.4;
    }
    texts(s, c, z, &l, p.band);
}

fn vapor(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let (y0, y1) = (z.b0 - 0.2, z.b1 + 0.2);
    let n = 24;
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let col = if t < 0.5 { p.a2.mix(p.band, t * 2.0) } else { p.band.mix(p.a1, (t - 0.5) * 2.0) };
        let w = W / n as f32;
        s.fill(rect(w * i as f32, y0, w + 0.05, y1 - y0), col);
    }
    let rows = if z.double { 1 } else { 2 };
    let q = 1.0;
    for i in 0..(W / q) as usize {
        for row in 0..rows {
            if (i + row) % 2 == 0 {
                s.fill(rect(i as f32 * q, row as f32 * q, q, q), p.a1.mix(p.text, 0.25));
                s.fill(rect(i as f32 * q, H - (row + 1) as f32 * q, q, q), p.a2.mix(p.text, 0.25));
            }
        }
    }
    let m = (z.b0 + z.b1) / 2.0;
    let r = ((y1 - y0) * 0.36).min(2.4);
    for &cx in &[5.5, W - 5.5] {
        s.fill(circle(cx, m, r), p.a3);
        for k in 0..3 {
            s.fill(rect(cx - r - 0.1, m + 0.1 + k as f32 * r * 0.3, 2.0 * r + 0.2, r * 0.12), p.band);
        }
    }
    let mut l = Lay::new(c, z);
    l.artist_w = 54.0;
    l.band_fx = Fx::Shadow(p.text, 0.35, 0.35);
    if !z.double {
        l.top = 5.4;
        l.bot = 19.6;
    }
    texts(s, c, z, &l, p.band);
}

// ---------------------------------------------------------------- altre grafiche

/// Riquadro quadrato a sinistra con l'immagine scelta dall'utente.
fn foto(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let pw = 25.0;
    let (fx, fy, fs) = (1.5, 1.5, 22.0);
    s.fill(rect(0.0, 0.0, pw, H), p.band);
    match &c.image {
        Some(img) => s.image(img.clone(), fx, fy, fs, fs),
        None => {
            // segnaposto: montagne e sole, finché non si sceglie un'immagine
            s.fill(rect(fx, fy, fs, fs), p.bg.mix(p.band, 0.2));
            s.fill(circle(fx + 15.5, fy + 6.5, 2.4), p.a2);
            s.fill(
                polygon(&[(fx, fy + 19.0), (fx + 7.0, fy + 9.5), (fx + 11.5, fy + 15.0), (fx + 15.0, fy + 11.5), (fx + fs, fy + 19.0), (fx + fs, fy + fs), (fx, fy + fs)]),
                p.band.mix(p.bg, 0.35),
            );
        }
    }
    s.stroke(rect(fx, fy, fs, fs), Rgb::WHITE, 0.6);
    s.fill(rect(pw, z.b0, W - pw, z.b1 - z.b0), p.band);
    s.fill(rect(pw, z.b0 - 0.6, W - pw, 0.3), p.a1);
    s.fill(rect(pw, z.b1 + 0.3, W - pw, 0.3), p.a1);
    let mut l = Lay::new(c, z);
    l.panel(pw);
    texts(s, c, z, &l, p.band);
}

/// Raggi di sole che partono dal centro, fascia arrotondata.
fn raggi(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let (cx, cy) = (W / 2.0, (z.b0 + z.b1) / 2.0);
    let n = 28;
    let light = p.bg.mix(p.a1, 0.5);
    for i in (0..n).step_by(2) {
        let a0 = TAU * i as f32 / n as f32;
        let a1 = a0 + TAU / n as f32;
        s.fill(polygon(&[(cx, cy), (cx + 60.0 * a0.cos(), cy + 60.0 * a0.sin()), (cx + 60.0 * a1.cos(), cy + 60.0 * a1.sin())]), light);
    }
    let h = z.b1 - z.b0;
    s.fill(rrect(3.0, z.b0, W - 6.0, h, h / 2.0), p.band);
    s.stroke(rrect(2.3, z.b0 - 0.7, W - 4.6, h + 1.4, h / 2.0 + 0.7), p.a3, 0.35);
    let mut l = Lay::new(c, z);
    l.title_w = 64.0;
    l.artist_w = 60.0;
    l.title_fx = Fx::Outline(p.a3, 0.6);
    l.artist_fx = Fx::Outline(p.a3, 0.6);
    texts(s, c, z, &l, p.band);
}

/// Scacchiera bianca e nera sopra e sotto, come lo ska degli anni 80.
fn ska(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let q = if z.double { 0.9 } else { 1.25 };
    let n = (W / q).ceil() as usize;
    for row in 0..2 {
        for i in 0..n {
            if (i + row) % 2 == 0 {
                s.fill(rect(i as f32 * q, row as f32 * q, q, q), p.a1);
                s.fill(rect(i as f32 * q, H - (row + 1) as f32 * q, q, q), p.a1);
            }
        }
    }
    s.fill(rect(0.0, z.b0, W, z.b1 - z.b0), p.band);
    s.fill(rect(0.0, z.b0 - 0.7, W, 0.35), p.a2);
    s.fill(rect(0.0, z.b1 + 0.35, W, 0.35), p.a2);
    let mut l = Lay::new(c, z);
    if !z.double {
        l.top = 5.3;
        l.bot = 19.7;
        l.title_cap = 2.6;
    }
    texts(s, c, z, &l, p.band);
}

/// Fumetto pop: retino a puntini, fulmini e fascia inclinata.
fn lampo(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    let step = 1.2;
    for (ox, oy, dx, dy) in [(0.0f32, 0.0f32, 1.0f32, 1.0f32), (W, H, -1.0, -1.0)] {
        for i in 0..14 {
            for j in 0..10 {
                let (x, y) = (ox + dx * (i as f32 * step + 0.6), oy + dy * (j as f32 * step + 0.6));
                let d = ((x - ox).powi(2) + (y - oy).powi(2)).sqrt();
                let r = 0.48 * (1.0 - d / 15.0);
                if r > 0.06 {
                    s.fill(circle(x, y, r), p.a2);
                }
            }
        }
    }
    let m = (z.b0 + z.b1) / 2.0;
    let k = if z.double { 0.6 } else { 0.85 };
    let bolt = [(0.0, -5.0), (3.0, -5.0), (1.6, -1.0), (3.6, -1.0), (-0.6, 5.5), (0.6, 0.8), (-1.2, 0.8)];
    for &bx in &[4.0, W - 6.5] {
        let pts: Vec<(f32, f32)> = bolt.iter().map(|&(x, y)| (bx + x * k, m + y * k)).collect();
        s.fill(polygon(&pts), p.band);
        s.stroke(polygon(&pts), p.a3, 0.3);
    }
    s.fill(polygon(&[(10.0, z.b0), (W - 8.0, z.b0), (W - 10.0, z.b1), (8.0, z.b1)]), p.a1);
    let mut l = Lay::new(c, z);
    l.band_col = Rgb::WHITE;
    l.title_w = 60.0;
    l.artist_w = 52.0;
    l.title_fx = Fx::Shadow(p.band, 0.4, 0.4);
    l.artist_fx = Fx::Shadow(p.band, 0.35, 0.35);
    texts(s, c, z, &l, p.a1);
}

/// Blocchi di colore in diagonale, come le giacche a vento degli anni 90.
fn sport(s: &mut Scene, c: &Ctx, z: &Z) {
    let p = c.pal;
    s.fill(polygon(&[(0.0, 0.0), (13.0, 0.0), (5.0, H), (0.0, H)]), p.a1);
    s.fill(polygon(&[(13.0, 0.0), (16.5, 0.0), (8.5, H), (5.0, H)]), p.a3);
    s.fill(polygon(&[(W - 5.0, 0.0), (W, 0.0), (W, H), (W - 13.0, H)]), p.a2);
    s.fill(polygon(&[(W - 8.5, 0.0), (W - 5.0, 0.0), (W - 13.0, H), (W - 16.5, H)]), p.band);
    s.fill(rect(11.0, z.b0, W - 22.0, z.b1 - z.b0), p.band);
    let mut l = Lay::new(c, z);
    l.title_w = 46.0;
    l.artist_w = 50.0;
    texts(s, c, z, &l, p.band);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Item;

    fn segments(l: &Label) -> usize {
        render_label(l)
            .items
            .iter()
            .map(|i| match i {
                Item::Shape(s) => s.path.len(),
                Item::Image { .. } => 0,
            })
            .sum()
    }

    #[test]
    fn cjk_names_are_drawn() {
        let empty = Label::sample("", "", "", "classico", "crema", "bebas");
        let cjk = Label::sample("山下達郎", "夜に駆ける", "爱情", "classico", "crema", "bebas");
        assert!(segments(&cjk) > segments(&empty) + 50);
    }

    #[test]
    fn every_design_draws_both_layouts() {
        for d in DESIGNS {
            let one = Label::sample("Artista", "Titolo", "Retro", d.id, d.palette, d.font);
            let two = Label::sample_two(("A", "Uno"), ("B", "Due"), "Festa", d.id, d.palette, d.font);
            assert!(segments(&one) > 20, "{}", d.id);
            assert!(segments(&two) > 20, "{}", d.id);
        }
    }
}
