#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod designs;
mod fonts;
mod i18n;
mod icon;
mod images;
mod model;
mod pdf;
mod raster;
mod scene;

use eframe::egui;
use std::path::PathBuf;

/// La stessa icona del file del programma, per la finestra e la barra delle applicazioni.
fn window_icon() -> egui::IconData {
    let (rgba, size) = icon::rgba(256);
    egui::IconData { rgba, width: size, height: size }
}

/// Salva in un'unica immagine tutte le grafiche con i loro colori e caratteri abbinati.
fn gallery(path: &std::path::Path) {
    let k = 8.0;
    let (lw, lh) = ((model::LABEL_W * k) as u32, (model::LABEL_H * k) as u32);
    // ogni grafica due volte: con un artista e con un artista per lato
    let cols = 2;
    let rows = designs::DESIGNS.len() as u32;
    let mut out = tiny_skia::Pixmap::new(cols as u32 * (lw + 20) + 20, rows * (lh + 20) + 20).unwrap();
    out.fill(tiny_skia::Color::from_rgba8(60, 45, 35, 255));
    for (i, d) in designs::DESIGNS.iter().flat_map(|d| [d, d]).enumerate() {
        let l = if i % 2 == 0 {
            model::Label::sample("I Cosmonauti", "Notte al Juke-Box", "Ballo del Sabato", d.id, d.palette, d.font)
        } else {
            model::Label::sample_two(
                ("I Cosmonauti", "Notte al Juke-Box"),
                ("Rita Lampo", "Ciao Ciao Amore"),
                "Festival d'Estate",
                d.id,
                d.palette,
                d.font,
            )
        };
        let pm = raster::rasterize(&designs::render_label(&l), k);
        let (x, y) = ((i % cols) as i32 * (lw as i32 + 20) + 20, (i / cols) as i32 * (lh as i32 + 20) + 20);
        out.draw_pixmap(x, y, pm.as_ref(), &tiny_skia::PixmapPaint::default(), tiny_skia::Transform::identity(), None);
    }
    let _ = out.save_png(path);
}

fn main() -> eframe::Result<()> {
    let mut args = std::env::args().skip(1);
    let mut shot: Option<PathBuf> = None;
    let mut export: Option<PathBuf> = None;
    let mut start = app::Start::default();
    let mut test_image: Option<String> = None;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--screenshot" => shot = args.next().map(PathBuf::from),
            "--export-pdf" => export = args.next().map(PathBuf::from),
            // per provare: lingua e modalità giorno all'avvio, immagine per la grafica con la foto
            "--lang" => {
                start.lang = args.next().and_then(|c| i18n::Lang::ALL.into_iter().find(|l| l.code() == c));
            }
            "--day" => start.day = Some(true),
            "--image" => test_image = args.next(),
            // rigenera i file dell'icona (assets/icon) dal disegno in icon.rs
            "--make-icons" => {
                let dir = args.next().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("assets/icon"));
                if let Err(e) = icon::write_all(&dir) {
                    eprintln!("Impossibile scrivere le icone in {}: {e}", dir.display());
                    std::process::exit(1);
                }
                return Ok(());
            }
            "--gallery" => {
                if let Some(path) = args.next() {
                    gallery(&PathBuf::from(path));
                }
                return Ok(());
            }
            _ => {}
        }
    }

    // Esportazione senza finestra: crea un PDF con le etichette di esempio.
    if let Some(path) = export {
        let mut project = model::Project::default();
        // abbastanza etichette per riempire un foglio e vedere gli indicatori di ritaglio
        let base = project.labels.clone();
        while project.labels.len() < 20 {
            project.labels.extend(base.iter().cloned());
        }
        if let Some(img) = &test_image {
            let mut l = model::Label::sample("Lola Neon", "Cuore Elettrico", "Luna di Vinile", "foto", "confetto", "archivo");
            l.image = img.clone();
            project.labels.insert(0, l);
        }
        let scenes: Vec<_> = project.labels.iter().map(designs::render_label).collect();
        if let Err(e) = std::fs::write(&path, pdf::build_pdf(&scenes, &project.sheet)) {
            eprintln!("Impossibile scrivere {}: {e}", path.display());
            std::process::exit(1);
        }
        return Ok(());
    }

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Etichette Jukebox")
        .with_inner_size([1320.0, 860.0])
        .with_min_inner_size([980.0, 640.0])
        .with_icon(window_icon());
    if shot.is_some() {
        viewport = viewport.with_active(false);
        #[cfg(target_os = "windows")]
        {
            viewport = viewport.with_window_level(egui::WindowLevel::AlwaysOnBottom);
        }
    }

    #[allow(unused_mut)]
    let mut options = eframe::NativeOptions { viewport, ..Default::default() };
    if shot.is_some() {
        options.persist_window = false;
        #[cfg(target_os = "macos")]
        {
            options.event_loop_builder = Some(Box::new(|builder| {
                use winit::platform::macos::EventLoopBuilderExtMacOS;
                builder.with_activate_ignoring_other_apps(false);
            }));
        }
    }

    eframe::run_native(
        "Etichette Jukebox",
        options,
        Box::new(move |cc| {
            start.shot = shot;
            if let Some(img) = test_image {
                start.image = Some(img);
            }
            Ok(Box::new(app::JukeApp::new(cc, start)))
        }),
    )
}
