//! Trasforma una scena in un'immagine per l'anteprima.

use crate::scene::{Item, Path, Scene, Seg};
use tiny_skia::{
    FillRule, FilterQuality, LineCap, LineJoin, Paint, PathBuilder, Pixmap, PixmapPaint, Stroke, Transform,
};

fn to_skia(path: &Path) -> Option<tiny_skia::Path> {
    let mut pb = PathBuilder::new();
    for s in path {
        match *s {
            Seg::M(x, y) => pb.move_to(x, y),
            Seg::L(x, y) => pb.line_to(x, y),
            Seg::Q(x1, y1, x, y) => pb.quad_to(x1, y1, x, y),
            Seg::C(x1, y1, x2, y2, x, y) => pb.cubic_to(x1, y1, x2, y2, x, y),
            Seg::Z => pb.close(),
        }
    }
    pb.finish()
}

pub fn rasterize(scene: &Scene, px_per_mm: f32) -> Pixmap {
    let w = (scene.w * px_per_mm).round().max(1.0) as u32;
    let h = (scene.h * px_per_mm).round().max(1.0) as u32;
    let mut pm = Pixmap::new(w, h).expect("dimensioni immagine");
    let tf = Transform::from_scale(w as f32 / scene.w, h as f32 / scene.h);
    for item in &scene.items {
        let sh = match item {
            Item::Shape(sh) => sh,
            Item::Image { img, x, y, w, h } => {
                let t = Transform::from_scale(w / img.w as f32, h / img.h as f32).post_translate(*x, *y).post_concat(tf);
                let paint = PixmapPaint { quality: FilterQuality::Bicubic, ..PixmapPaint::default() };
                pm.draw_pixmap(0, 0, img.pixmap.as_ref(), &paint, t, None);
                continue;
            }
        };
        let Some(p) = to_skia(&sh.path) else { continue };
        if let Some((c, width)) = sh.stroke {
            let mut paint = Paint::default();
            paint.set_color_rgba8(c.0, c.1, c.2, 255);
            paint.anti_alias = true;
            let st = Stroke { width, line_cap: LineCap::Round, line_join: LineJoin::Round, ..Stroke::default() };
            pm.stroke_path(&p, &paint, &st, tf, None);
        }
        if let Some(c) = sh.fill {
            let mut paint = Paint::default();
            paint.set_color_rgba8(c.0, c.1, c.2, 255);
            paint.anti_alias = true;
            pm.fill_path(&p, &paint, FillRule::Winding, tf, None);
        }
    }
    pm
}

pub fn to_color_image(pm: &Pixmap) -> eframe::egui::ColorImage {
    eframe::egui::ColorImage::from_rgba_premultiplied([pm.width() as usize, pm.height() as usize], pm.data())
}
