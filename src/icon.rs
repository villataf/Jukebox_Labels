//! L'icona del programma: un disco in vinile che spunta da dietro un'etichetta da juke-box.
//! Disegnata in vettoriale (1024 × 1024 unità), poi salvata nei formati di ogni sistema.

use crate::raster;
use crate::scene::*;
use std::path::Path;

const S: f32 = 1024.0;

pub fn scene() -> Scene {
    let mut s = Scene::new(S, S);
    let charcoal = Rgb::hex(0x1C1C1F);
    // fondo: quadrato arrotondato grigio scuro con un bordo leggermente più chiaro
    s.fill(rrect(24.0, 24.0, S - 48.0, S - 48.0, 210.0), Rgb::hex(0x3A3A40));
    s.fill(rrect(40.0, 40.0, S - 80.0, S - 80.0, 196.0), charcoal);
    // raggi di sole dietro al disco
    let (cx, cy) = (512.0, 440.0);
    for i in (0..24).step_by(2) {
        let a0 = std::f32::consts::TAU * i as f32 / 24.0;
        let a1 = a0 + std::f32::consts::TAU / 24.0;
        // abbastanza corti da restare dentro il quadrato arrotondato
        let r = 392.0;
        s.fill(polygon(&[(cx, cy), (cx + r * a0.cos(), cy + r * a0.sin()), (cx + r * a1.cos(), cy + r * a1.sin())]), Rgb::hex(0x2C2C31));
    }
    // il disco
    s.fill(circle(cx, cy, 372.0), Rgb::hex(0x0C0C0C));
    for (r, w) in [(340.0, 7.0), (300.0, 6.0), (262.0, 6.0), (224.0, 5.0), (186.0, 5.0)] {
        s.stroke(circle(cx, cy, r), Rgb::hex(0x34343A), w);
    }
    // riflesso
    s.stroke(polyline(&arc(cx, cy, 320.0, 3.55, 4.25)), Rgb::hex(0x6A6A72), 14.0);
    // etichetta centrale del disco
    s.fill(circle(cx, cy, 136.0), Rgb::hex(0xE86A17));
    s.fill(circle(cx, cy, 100.0), Rgb::hex(0xF2B134));
    s.fill(rect(cx - 136.0, cy - 14.0, 272.0, 28.0), Rgb::hex(0xC0392B));
    s.fill(circle(cx, cy, 20.0), Rgb::hex(0x0C0C0C));
    // l'etichetta da juke-box (proporzioni 78 × 25) con ombra
    let (x, y, w, h) = (96.0, 612.0, 832.0, 266.0);
    s.fill(rrect(x + 14.0, y + 18.0, w, h, 18.0), Rgb::hex(0x050505));
    s.fill(rrect(x, y, w, h, 18.0), Rgb::hex(0xFBF6E9));
    let red = Rgb::hex(0xC8102E);
    s.fill(rect(x, y + 16.0, w, 16.0), red);
    s.fill(rect(x, y + h - 32.0, w, 16.0), red);
    // fascia centrale con le punte, come sulle etichette classiche
    let (b0, b1, m) = (y + 98.0, y + 168.0, y + 133.0);
    s.fill(polygon(&[(x + 70.0, b0), (x + w - 70.0, b0), (x + w - 30.0, m), (x + w - 70.0, b1), (x + 70.0, b1), (x + 30.0, m)]), red);
    s.fill(star(x + 84.0, m, 26.0, 11.0, 5), Rgb::hex(0xF2B134));
    s.fill(star(x + w - 84.0, m, 26.0, 11.0, 5), Rgb::hex(0xF2B134));
    // righe di "testo" stilizzate
    let ink = Rgb::hex(0x1F1F1F);
    s.fill(rrect(x + 210.0, y + 50.0, w - 420.0, 26.0, 13.0), ink);
    s.fill(rrect(x + 250.0, y + 190.0, w - 500.0, 26.0, 13.0), ink);
    s.fill(rrect(x + 230.0, m - 12.0, w - 460.0, 24.0, 12.0), Rgb::hex(0xFFFFFF));
    s
}

fn arc(cx: f32, cy: f32, r: f32, a0: f32, a1: f32) -> Vec<(f32, f32)> {
    (0..=24).map(|i| a0 + (a1 - a0) * i as f32 / 24.0).map(|a| (cx + r * a.cos(), cy + r * a.sin())).collect()
}

pub fn render(size: u32) -> tiny_skia::Pixmap {
    raster::rasterize(&scene(), size as f32 / S)
}

/// Pixel RGBA non premoltiplicati, per l'icona della finestra.
pub fn rgba(size: u32) -> (Vec<u8>, u32) {
    let pm = render(size);
    let data = pm
        .pixels()
        .iter()
        .flat_map(|p| {
            let c = p.demultiply();
            [c.red(), c.green(), c.blue(), c.alpha()]
        })
        .collect();
    (data, pm.width())
}

fn png(size: u32) -> Vec<u8> {
    render(size).encode_png().expect("PNG dell'icona")
}

/// File .ico per Windows: più misure, ognuna salvata come PNG.
fn ico() -> Vec<u8> {
    let sizes = [16u32, 24, 32, 48, 64, 128, 256];
    let images: Vec<Vec<u8>> = sizes.iter().map(|&s| png(s)).collect();
    let mut out = Vec::new();
    out.extend_from_slice(&[0, 0, 1, 0]);
    out.extend_from_slice(&(sizes.len() as u16).to_le_bytes());
    let mut offset = 6 + 16 * sizes.len() as u32;
    for (s, data) in sizes.iter().zip(&images) {
        let dim = if *s >= 256 { 0 } else { *s as u8 };
        out.extend_from_slice(&[dim, dim, 0, 0]);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&offset.to_le_bytes());
        offset += data.len() as u32;
    }
    for data in images {
        out.extend_from_slice(&data);
    }
    out
}

/// File .icns per macOS.
fn icns() -> Vec<u8> {
    let entries: [(&[u8; 4], u32); 9] = [
        (b"icp4", 16),
        (b"icp5", 32),
        (b"icp6", 64),
        (b"ic07", 128),
        (b"ic08", 256),
        (b"ic09", 512),
        (b"ic10", 1024),
        (b"ic11", 32),
        (b"ic12", 64),
    ];
    let mut body = Vec::new();
    for (kind, size) in entries {
        let data = png(size);
        body.extend_from_slice(kind);
        body.extend_from_slice(&(data.len() as u32 + 8).to_be_bytes());
        body.extend_from_slice(&data);
    }
    let mut out = b"icns".to_vec();
    out.extend_from_slice(&(body.len() as u32 + 8).to_be_bytes());
    out.extend_from_slice(&body);
    out
}

/// Scrive icon.ico, icon.icns e i PNG (16-1024 px) nella cartella data.
pub fn write_all(dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    std::fs::write(dir.join("icon.ico"), ico())?;
    std::fs::write(dir.join("icon.icns"), icns())?;
    for size in [16u32, 32, 48, 64, 128, 256, 512, 1024] {
        std::fs::write(dir.join(format!("icon-{size}.png")), png(size))?;
    }
    Ok(())
}
