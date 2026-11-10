//! Disegno vettoriale dell'etichetta in millimetri (origine in alto a sinistra).
//! La stessa scena viene usata per l'anteprima e per il PDF, così sono identici.

use crate::fonts::{faces, fallbacks};
use ttf_parser::OutlineBuilder;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub const fn hex(h: u32) -> Rgb {
        Rgb((h >> 16) as u8, (h >> 8) as u8, h as u8)
    }
    pub fn mix(self, o: Rgb, t: f32) -> Rgb {
        let l = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round().clamp(0.0, 255.0) as u8;
        Rgb(l(self.0, o.0), l(self.1, o.1), l(self.2, o.2))
    }
    pub fn unit(self) -> (f32, f32, f32) {
        (self.0 as f32 / 255.0, self.1 as f32 / 255.0, self.2 as f32 / 255.0)
    }
    pub const WHITE: Rgb = Rgb(255, 255, 255);
    pub const BLACK: Rgb = Rgb(0, 0, 0);
}

#[derive(Clone, Copy, Debug)]
pub enum Seg {
    M(f32, f32),
    L(f32, f32),
    Q(f32, f32, f32, f32),
    C(f32, f32, f32, f32, f32, f32),
    Z,
}

pub type Path = Vec<Seg>;

pub struct Shape {
    pub path: Path,
    pub fill: Option<Rgb>,
    pub stroke: Option<(Rgb, f32)>,
}

pub enum Item {
    Shape(Shape),
    /// Un'immagine dell'utente, stirata nel rettangolo (x, y, w, h) in mm.
    Image { img: std::sync::Arc<crate::images::Img>, x: f32, y: f32, w: f32, h: f32 },
}

pub struct Scene {
    pub w: f32,
    pub h: f32,
    pub items: Vec<Item>,
}

impl Scene {
    pub fn new(w: f32, h: f32) -> Scene {
        Scene { w, h, items: Vec::new() }
    }
    pub fn fill(&mut self, path: Path, c: Rgb) {
        if !path.is_empty() {
            self.items.push(Item::Shape(Shape { path, fill: Some(c), stroke: None }));
        }
    }
    pub fn stroke(&mut self, path: Path, c: Rgb, width: f32) {
        if !path.is_empty() {
            self.items.push(Item::Shape(Shape { path, fill: None, stroke: Some((c, width)) }));
        }
    }
    pub fn image(&mut self, img: std::sync::Arc<crate::images::Img>, x: f32, y: f32, w: f32, h: f32) {
        self.items.push(Item::Image { img, x, y, w, h });
    }
}

// ---------------------------------------------------------------- forme

pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Path {
    vec![Seg::M(x, y), Seg::L(x + w, y), Seg::L(x + w, y + h), Seg::L(x, y + h), Seg::Z]
}

pub fn polygon(pts: &[(f32, f32)]) -> Path {
    let mut p = Vec::with_capacity(pts.len() + 1);
    for (i, &(x, y)) in pts.iter().enumerate() {
        p.push(if i == 0 { Seg::M(x, y) } else { Seg::L(x, y) });
    }
    p.push(Seg::Z);
    p
}

pub fn polyline(pts: &[(f32, f32)]) -> Path {
    pts.iter()
        .enumerate()
        .map(|(i, &(x, y))| if i == 0 { Seg::M(x, y) } else { Seg::L(x, y) })
        .collect()
}

pub fn line(x1: f32, y1: f32, x2: f32, y2: f32) -> Path {
    vec![Seg::M(x1, y1), Seg::L(x2, y2)]
}

const KAPPA: f32 = 0.552_284_8;

pub fn ellipse(cx: f32, cy: f32, rx: f32, ry: f32) -> Path {
    let (kx, ky) = (rx * KAPPA, ry * KAPPA);
    vec![
        Seg::M(cx + rx, cy),
        Seg::C(cx + rx, cy + ky, cx + kx, cy + ry, cx, cy + ry),
        Seg::C(cx - kx, cy + ry, cx - rx, cy + ky, cx - rx, cy),
        Seg::C(cx - rx, cy - ky, cx - kx, cy - ry, cx, cy - ry),
        Seg::C(cx + kx, cy - ry, cx + rx, cy - ky, cx + rx, cy),
        Seg::Z,
    ]
}

pub fn circle(cx: f32, cy: f32, r: f32) -> Path {
    ellipse(cx, cy, r, r)
}

pub fn rrect(x: f32, y: f32, w: f32, h: f32, r: f32) -> Path {
    let r = r.min(w / 2.0).min(h / 2.0);
    let k = r * (1.0 - KAPPA);
    vec![
        Seg::M(x + r, y),
        Seg::L(x + w - r, y),
        Seg::C(x + w - k, y, x + w, y + k, x + w, y + r),
        Seg::L(x + w, y + h - r),
        Seg::C(x + w, y + h - k, x + w - k, y + h, x + w - r, y + h),
        Seg::L(x + r, y + h),
        Seg::C(x + k, y + h, x, y + h - k, x, y + h - r),
        Seg::L(x, y + r),
        Seg::C(x, y + k, x + k, y, x + r, y),
        Seg::Z,
    ]
}

pub fn star(cx: f32, cy: f32, r_out: f32, r_in: f32, n: usize) -> Path {
    let mut pts = Vec::new();
    for i in 0..n * 2 {
        let r = if i % 2 == 0 { r_out } else { r_in };
        let a = std::f32::consts::PI * i as f32 / n as f32 - std::f32::consts::FRAC_PI_2;
        pts.push((cx + r * a.cos(), cy + r * a.sin()));
    }
    polygon(&pts)
}

/// Mezzo cerchio superiore (un "sole" che tramonta) con base sulla retta y = cy.
pub fn half_disc(cx: f32, cy: f32, r: f32) -> Path {
    let k = r * KAPPA;
    vec![
        Seg::M(cx - r, cy),
        Seg::C(cx - r, cy - k, cx - k, cy - r, cx, cy - r),
        Seg::C(cx + k, cy - r, cx + r, cy - k, cx + r, cy),
        Seg::Z,
    ]
}

/// Piccolo generatore pseudo-casuale, sempre uguale: le grafiche non cambiano a ogni ridisegno.
pub struct Rand(u32);
impl Rand {
    pub fn new(seed: u32) -> Rand {
        Rand(seed.wrapping_mul(2_654_435_761).wrapping_add(12345))
    }
    pub fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 % 10_000) as f32 / 10_000.0
    }
    pub fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.next()
    }
}

// ---------------------------------------------------------------- testo

struct Outline<'a> {
    out: &'a mut Path,
    ox: f32,
    oy: f32,
    s: f32,
}

impl OutlineBuilder for Outline<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.out.push(Seg::M(self.ox + x * self.s, self.oy - y * self.s));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.out.push(Seg::L(self.ox + x * self.s, self.oy - y * self.s));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.out.push(Seg::Q(
            self.ox + x1 * self.s,
            self.oy - y1 * self.s,
            self.ox + x * self.s,
            self.oy - y * self.s,
        ));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.out.push(Seg::C(
            self.ox + x1 * self.s,
            self.oy - y1 * self.s,
            self.ox + x2 * self.s,
            self.oy - y2 * self.s,
            self.ox + x * self.s,
            self.oy - y * self.s,
        ));
    }
    fn close(&mut self) {
        self.out.push(Seg::Z);
    }
}

/// Per ogni lettera: quale font la contiene e quale glifo usare.
fn glyphs(font: usize, text: &str) -> Vec<(usize, ttf_parser::GlyphId)> {
    let faces = faces();
    let mut v = Vec::new();
    for ch in text.chars() {
        let ch = if ch.is_control() { ' ' } else { ch };
        let mut found = None;
        for f in std::iter::once(font).chain(fallbacks()) {
            if let Some(g) = faces[f].glyph_index(ch) {
                found = Some((f, g));
                break;
            }
        }
        if let Some(g) = found.or_else(|| faces[font].glyph_index('?').map(|g| (font, g))) {
            v.push(g);
        }
    }
    v
}

fn advance(f: usize, g: ttf_parser::GlyphId) -> f32 {
    let face = &faces()[f];
    let adv = face.glyph_hor_advance(g).unwrap_or(0) as f32 / face.units_per_em() as f32;
    // alcuni font retrò hanno spazi quasi invisibili: le parole restano separate
    if face.glyph_index(' ') == Some(g) {
        adv.max(0.25)
    } else {
        adv
    }
}

/// Larghezza del testo per un carattere alto 1 mm (em), con spaziatura in em.
pub fn measure(font: usize, text: &str, spacing: f32) -> f32 {
    let g = glyphs(font, text);
    let n = g.len();
    g.into_iter().map(|(f, g)| advance(f, g)).sum::<f32>() + spacing * n.saturating_sub(1) as f32
}

/// Altezza delle maiuscole in proporzione all'em.
pub fn cap_ratio(font: usize) -> f32 {
    let face = &faces()[font];
    let upm = face.units_per_em() as f32;
    match face.capital_height() {
        Some(c) if c > 0 => c as f32 / upm,
        _ => 0.7,
    }
}

pub fn text_path(font: usize, text: &str, size: f32, x: f32, baseline: f32, spacing: f32) -> Path {
    let faces = faces();
    let mut out = Vec::new();
    let mut pen = x;
    for (f, g) in glyphs(font, text) {
        let face = &faces[f];
        let s = size / face.units_per_em() as f32;
        let mut b = Outline { out: &mut out, ox: pen, oy: baseline, s };
        face.outline_glyph(g, &mut b);
        pen += advance(f, g) * size + spacing * size;
    }
    out
}

#[derive(Clone, Copy)]
pub enum Fx {
    None,
    /// Ombra spostata (colore, dx, dy).
    Shadow(Rgb, f32, f32),
    /// Contorno dietro alle lettere (colore, spessore).
    Outline(Rgb, f32),
    /// Alone "neon": due contorni sfumati.
    Glow(Rgb, f32),
}

pub struct TextSpec {
    pub cx: f32,
    pub cy: f32,
    pub cap: f32,
    pub max_w: f32,
    pub spacing: f32,
    pub color: Rgb,
    pub fx: Fx,
}

/// Scrive il testo centrato in (cx, cy): rimpicciolisce da solo se è troppo lungo.
pub fn place_text(scene: &mut Scene, font: usize, text: &str, t: &TextSpec, bg: Rgb) {
    let text = text.trim();
    if text.is_empty() {
        return;
    }
    let ratio = cap_ratio(font);
    let mut size = t.cap / ratio;
    let w1 = measure(font, text, t.spacing);
    if w1 * size > t.max_w && w1 > 0.0 {
        size = t.max_w / w1;
    }
    let width = w1 * size;
    let cap_h = size * ratio;
    let x = t.cx - width / 2.0;
    let baseline = t.cy + cap_h / 2.0;
    let path = text_path(font, text, size, x, baseline, t.spacing);
    let k = (size / 4.0).clamp(0.35, 1.0);
    match t.fx {
        Fx::None => {}
        Fx::Shadow(c, dx, dy) => {
            let p = text_path(font, text, size, x + dx * k, baseline + dy * k, t.spacing);
            scene.fill(p.clone(), c);
            scene.stroke(p, c, 0.08);
        }
        Fx::Outline(c, w) => scene.stroke(path.clone(), c, w * k),
        Fx::Glow(c, w) => {
            scene.stroke(path.clone(), c.mix(bg, 0.72), w * 2.2 * k);
            scene.stroke(path.clone(), c.mix(bg, 0.3), w * k);
        }
    }
    scene.fill(path, t.color);
}
