//! Immagini scelte dall'utente: ritagliate al centro in un quadrato e tenute in memoria.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, OnceLock};

/// Lato massimo dell'immagine tenuta: 600 px su 22 mm sono circa 700 dpi, più che sufficienti.
const MAX_SIDE: u32 = 600;

pub struct Img {
    /// Identifica il file e la sua versione (cambia se il file viene modificato).
    pub id: u64,
    pub w: u32,
    pub h: u32,
    /// Pixel RGB, riga per riga dall'alto: per il PDF.
    pub rgb: Vec<u8>,
    /// La stessa immagine per l'anteprima.
    pub pixmap: tiny_skia::Pixmap,
}

static CACHE: OnceLock<Mutex<HashMap<u64, Option<Arc<Img>>>>> = OnceLock::new();

fn version_id(path: &str) -> Option<u64> {
    let meta = std::fs::metadata(path).ok()?;
    let mut h = std::collections::hash_map::DefaultHasher::new();
    path.hash(&mut h);
    meta.len().hash(&mut h);
    meta.modified().ok().hash(&mut h);
    Some(h.finish())
}

/// Carica (una volta sola) l'immagine al percorso dato. `None` se manca o non si legge.
pub fn load(path: &str) -> Option<Arc<Img>> {
    if path.trim().is_empty() {
        return None;
    }
    let id = version_id(path)?;
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(v) = cache.lock().ok()?.get(&id) {
        return v.clone();
    }
    let img = decode(path, id).map(Arc::new);
    if let Ok(mut c) = cache.lock() {
        c.insert(id, img.clone());
    }
    img
}

fn decode(path: &str, id: u64) -> Option<Img> {
    let src = image::open(path).ok()?.to_rgba8();
    let side = src.width().min(src.height());
    if side == 0 {
        return None;
    }
    let (x0, y0) = ((src.width() - side) / 2, (src.height() - side) / 2);
    let square = image::imageops::crop_imm(&src, x0, y0, side, side).to_image();
    let n = side.min(MAX_SIDE);
    let small = if n == side { square } else { image::imageops::resize(&square, n, n, image::imageops::FilterType::CatmullRom) };
    // le parti trasparenti diventano bianche, come la carta
    let mut rgb = Vec::with_capacity((n * n * 3) as usize);
    let mut rgba = Vec::with_capacity((n * n * 4) as usize);
    for p in small.pixels() {
        let a = p[3] as f32 / 255.0;
        let c = |v: u8| (v as f32 * a + 255.0 * (1.0 - a)).round() as u8;
        let (r, g, b) = (c(p[0]), c(p[1]), c(p[2]));
        rgb.extend_from_slice(&[r, g, b]);
        rgba.extend_from_slice(&[r, g, b, 255]);
    }
    let pixmap = tiny_skia::Pixmap::from_vec(rgba, tiny_skia::IntSize::from_wh(n, n)?)?;
    Some(Img { id, w: n, h: n, rgb, pixmap })
}
