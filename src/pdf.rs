//! Scrive il PDF: le etichette a 78 × 25 mm, in vettoriale (si stampano nitide a ogni risoluzione).

use crate::model::{Paper, Sheet, LABEL_H, LABEL_W};
use crate::images::Img;
use crate::scene::{Item, Scene, Seg};
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::Arc;

const PT: f32 = 72.0 / 25.4;
/// Margine minimo del foglio: lascia spazio agli indicatori di ritaglio esterni.
const MARGIN: f32 = 10.0;

pub struct Layout {
    pub page_w: f32,
    pub page_h: f32,
    pub cols: usize,
    pub rows: usize,
    pub x0: f32,
    pub y0: f32,
    pub gap: f32,
}

impl Layout {
    pub fn per_page(&self) -> usize {
        self.cols * self.rows
    }
}

pub fn layout(sheet: &Sheet) -> Layout {
    let gap = sheet.gap_mm.clamp(0.0, 20.0);
    let (pw, ph) = match sheet.paper {
        Paper::A4 => (210.0, 297.0),
        Paper::Letter => (215.9, 279.4),
        Paper::Single => {
            return Layout { page_w: LABEL_W, page_h: LABEL_H, cols: 1, rows: 1, x0: 0.0, y0: 0.0, gap: 0.0 }
        }
    };
    let cols = (((pw - 2.0 * MARGIN + gap) / (LABEL_W + gap)).floor() as usize).max(1);
    let rows = (((ph - 2.0 * MARGIN + gap) / (LABEL_H + gap)).floor() as usize).max(1);
    let gw = cols as f32 * LABEL_W + (cols - 1) as f32 * gap;
    let gh = rows as f32 * LABEL_H + (rows - 1) as f32 * gap;
    Layout { page_w: pw, page_h: ph, cols, rows, x0: (pw - gw) / 2.0, y0: (ph - gh) / 2.0, gap }
}

/// Indicatori di ritaglio per le prime `count` etichette di una pagina: segmenti
/// (x1, y1, x2, y2) in mm, origine in alto a sinistra. Ogni angolo ha due trattini che
/// partono poco fuori dall'etichetta, così non finiscono mai sopra la grafica.
pub fn crop_marks(lay: &Layout, count: usize) -> Vec<(f32, f32, f32, f32)> {
    const OUTER_OFF: f32 = 1.0;
    const OUTER_LEN: f32 = 4.0;
    const INNER_OFF: f32 = 0.5;
    let inner_len = lay.gap / 2.0 - INNER_OFF - 0.3;
    let mut v = Vec::new();
    for i in 0..count.min(lay.per_page()) {
        let (col, row) = (i % lay.cols, i / lay.cols);
        let x = lay.x0 + col as f32 * (LABEL_W + lay.gap);
        let y = lay.y0 + row as f32 * (LABEL_H + lay.gap);
        let has = |c: isize, r: isize| {
            c >= 0 && r >= 0 && (c as usize) < lay.cols && (r as usize * lay.cols + c as usize) < count
        };
        let (c, r) = (col as isize, row as isize);
        // per ogni lato: c'è un'etichetta vicina? allora il trattino sta nello spazio tra le due
        let tick = |neighbor: bool| -> Option<(f32, f32)> {
            if !neighbor {
                Some((OUTER_OFF, OUTER_LEN))
            } else if inner_len >= 0.6 {
                Some((INNER_OFF, inner_len))
            } else {
                None
            }
        };
        let left = tick(has(c - 1, r));
        let right = tick(has(c + 1, r));
        let up = tick(has(c, r - 1));
        let down = tick(has(c, r + 1));
        for (cx, cy, hx, vy) in [
            (x, y, left.map(|t| (-1.0, t)), up.map(|t| (-1.0, t))),
            (x + LABEL_W, y, right.map(|t| (1.0, t)), up.map(|t| (-1.0, t))),
            (x, y + LABEL_H, left.map(|t| (-1.0, t)), down.map(|t| (1.0, t))),
            (x + LABEL_W, y + LABEL_H, right.map(|t| (1.0, t)), down.map(|t| (1.0, t))),
        ] {
            if let Some((d, (off, len))) = hx {
                v.push((cx + d * off, cy, cx + d * (off + len), cy));
            }
            if let Some((d, (off, len))) = vy {
                v.push((cx, cy + d * off, cx, cy + d * (off + len)));
            }
        }
    }
    v
}

fn draw_crop_marks(out: &mut String, lay: &Layout, count: usize) {
    let marks = crop_marks(lay, count);
    if marks.is_empty() {
        return;
    }
    let _ = writeln!(out, "q\n{} 0 0 {} 0 {} cm\n0 0 0 RG 0.15 w 0 J", n(PT), n(-PT), n(lay.page_h * PT));
    for (x1, y1, x2, y2) in marks {
        let _ = writeln!(out, "{} {} m {} {} l S", n(x1), n(y1), n(x2), n(y2));
    }
    out.push_str("Q\n");
}

fn n(v: f32) -> String {
    let s = format!("{:.3}", v);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" || s.is_empty() { "0".into() } else { s.to_string() }
}

fn path_ops(out: &mut String, path: &[Seg]) {
    let (mut cx, mut cy) = (0.0f32, 0.0f32);
    for s in path {
        match *s {
            Seg::M(x, y) => {
                let _ = writeln!(out, "{} {} m", n(x), n(y));
                (cx, cy) = (x, y);
            }
            Seg::L(x, y) => {
                let _ = writeln!(out, "{} {} l", n(x), n(y));
                (cx, cy) = (x, y);
            }
            Seg::Q(x1, y1, x, y) => {
                // le curve quadratiche dei font diventano cubiche
                let c1 = (cx + 2.0 / 3.0 * (x1 - cx), cy + 2.0 / 3.0 * (y1 - cy));
                let c2 = (x + 2.0 / 3.0 * (x1 - x), y + 2.0 / 3.0 * (y1 - y));
                let _ = writeln!(out, "{} {} {} {} {} {} c", n(c1.0), n(c1.1), n(c2.0), n(c2.1), n(x), n(y));
                (cx, cy) = (x, y);
            }
            Seg::C(x1, y1, x2, y2, x, y) => {
                let _ = writeln!(out, "{} {} {} {} {} {} c", n(x1), n(y1), n(x2), n(y2), n(x), n(y));
                (cx, cy) = (x, y);
            }
            Seg::Z => out.push_str("h\n"),
        }
    }
}

fn draw_scene(out: &mut String, scene: &Scene, ox: f32, oy: f32, page_h: f32, images: &HashMap<u64, usize>) {
    // da millimetri (origine in alto) a punti PDF (origine in basso)
    let _ = writeln!(out, "q\n{} 0 0 {} {} {} cm", n(PT), n(-PT), n(ox * PT), n((page_h - oy) * PT));
    let _ = writeln!(out, "0 0 {} {} re W n", n(scene.w), n(scene.h));
    out.push_str("1 j 1 J\n");
    for item in &scene.items {
        let sh = match item {
            Item::Shape(sh) => sh,
            Item::Image { img, x, y, w, h } => {
                if let Some(k) = images.get(&img.id) {
                    // il quadrato unitario dell'immagine va nel rettangolo, con la prima riga in alto
                    let _ = writeln!(out, "q {} 0 0 {} {} {} cm /Im{} Do Q", n(*w), n(-*h), n(*x), n(y + h), k);
                }
                continue;
            }
        };
        if let Some((c, w)) = sh.stroke {
            let (r, g, b) = c.unit();
            let _ = writeln!(out, "{} {} {} RG {} w", n(r), n(g), n(b), n(w));
            path_ops(out, &sh.path);
            out.push_str("S\n");
        }
        if let Some(c) = sh.fill {
            let (r, g, b) = c.unit();
            let _ = writeln!(out, "{} {} {} rg", n(r), n(g), n(b));
            path_ops(out, &sh.path);
            out.push_str("f\n");
        }
    }
    out.push_str("Q\n");
}

fn cut_line(out: &mut String, ox: f32, oy: f32, page_h: f32) {
    let _ = writeln!(
        out,
        "q 0.6 0.6 0.6 RG 0.25 w {} {} {} {} re S Q",
        n(ox * PT),
        n((page_h - oy - LABEL_H) * PT),
        n(LABEL_W * PT),
        n(LABEL_H * PT)
    );
}

/// Crea il PDF con tutte le etichette (ripetute `copies` volte).
pub fn build_pdf(scenes: &[Scene], sheet: &Sheet) -> Vec<u8> {
    let lay = layout(sheet);
    let copies = sheet.copies.clamp(1, 50) as usize;
    let all: Vec<&Scene> = scenes.iter().flat_map(|s| std::iter::repeat(s).take(copies)).collect();

    // immagini dell'utente: ognuna una volta sola nel file
    let mut images: Vec<Arc<Img>> = Vec::new();
    let mut image_index: HashMap<u64, usize> = HashMap::new();
    for scene in scenes {
        for item in &scene.items {
            if let Item::Image { img, .. } = item {
                image_index.entry(img.id).or_insert_with(|| {
                    images.push(img.clone());
                    images.len()
                });
            }
        }
    }

    let per = lay.per_page();
    let mut pages: Vec<Vec<u8>> = Vec::new();
    for chunk in all.chunks(per.max(1)) {
        let mut c = String::new();
        for (i, scene) in chunk.iter().enumerate() {
            let (col, row) = (i % lay.cols, i / lay.cols);
            let ox = lay.x0 + col as f32 * (LABEL_W + lay.gap);
            let oy = lay.y0 + row as f32 * (LABEL_H + lay.gap);
            draw_scene(&mut c, scene, ox, oy, lay.page_h, &image_index);
            if sheet.cut_lines && sheet.paper != Paper::Single {
                cut_line(&mut c, ox, oy, lay.page_h);
            }
        }
        if sheet.crop_marks && sheet.paper != Paper::Single {
            draw_crop_marks(&mut c, &lay, chunk.len());
        }
        pages.push(miniz_oxide::deflate::compress_to_vec_zlib(c.as_bytes(), 6));
    }
    if pages.is_empty() {
        pages.push(miniz_oxide::deflate::compress_to_vec_zlib(b"", 6));
    }

    // numeri degli oggetti: 1 catalogo, 2 pagine, poi le immagini, poi pagina e contenuto di ogni pagina
    let first_image = 3;
    let first_page = first_image + images.len();
    let page_id = |i: usize| first_page + i * 2;
    let n_pages = pages.len();

    let mut pdf: Vec<u8> = Vec::new();
    let mut offsets = Vec::new();
    pdf.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");
    let mut obj = |pdf: &mut Vec<u8>, body: &[u8]| {
        offsets.push(pdf.len());
        pdf.extend_from_slice(format!("{} 0 obj\n", offsets.len()).as_bytes());
        pdf.extend_from_slice(body);
        pdf.extend_from_slice(b"\nendobj\n");
    };
    let stream = |dict: String, data: &[u8]| {
        let mut body = format!("<< {dict} /Length {} /Filter /FlateDecode >>\nstream\n", data.len()).into_bytes();
        body.extend_from_slice(data);
        body.extend_from_slice(b"\nendstream");
        body
    };
    obj(&mut pdf, b"<< /Type /Catalog /Pages 2 0 R >>");
    let kids: Vec<String> = (0..n_pages).map(|i| format!("{} 0 R", page_id(i))).collect();
    obj(&mut pdf, format!("<< /Type /Pages /Kids [{}] /Count {} >>", kids.join(" "), n_pages).as_bytes());
    for img in &images {
        let data = miniz_oxide::deflate::compress_to_vec_zlib(&img.rgb, 6);
        let dict = format!(
            "/Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8",
            img.w, img.h
        );
        obj(&mut pdf, &stream(dict, &data));
    }
    let xobjects: String = (1..=images.len()).map(|k| format!("/Im{k} {} 0 R ", first_image + k - 1)).collect();
    for (i, data) in pages.iter().enumerate() {
        obj(
            &mut pdf,
            format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /XObject << {}>> >> /Contents {} 0 R >>",
                n(lay.page_w * PT),
                n(lay.page_h * PT),
                xobjects,
                page_id(i) + 1
            )
            .as_bytes(),
        );
        obj(&mut pdf, &stream(String::new(), data));
    }
    obj(&mut pdf, b"<< /Title (Jukebox) /Producer (Jukebox Etichette) >>");
    let info_id = offsets.len();
    let xref = pdf.len();
    let mut tail = format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1);
    for o in &offsets {
        let _ = write!(tail, "{:010} 00000 n \n", o);
    }
    let _ = write!(
        tail,
        "trailer\n<< /Size {} /Root 1 0 R /Info {} 0 R >>\nstartxref\n{}\n%%EOF\n",
        offsets.len() + 1,
        info_id,
        xref
    );
    pdf.extend_from_slice(tail.as_bytes());
    pdf
}

