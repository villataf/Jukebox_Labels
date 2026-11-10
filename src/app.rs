//! La finestra del programma.

use crate::designs::{self, DESIGNS, PALETTES};
use crate::fonts::{self, FONTS};
use crate::i18n::{self, tr, Lang, T};
use crate::model::{Label, Paper, Project, LABEL_H, LABEL_W};
use crate::scene::Scene;
use crate::{pdf, raster};
use eframe::egui::{
    self, pos2, vec2, Align, Align2, Color32, Context, CornerRadius, FontFamily, FontId, Layout, Rect, RichText,
    Sense, Stroke, TextureHandle, TextureId, TextureOptions, Ui, Vec2,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// ---------------------------------------------------------------- temi: notte (grigio scuro/nero) e giorno

struct Theme {
    bar: Color32,
    panel: Color32,
    card: Color32,
    hover: Color32,
    field: Color32,
    stroke: Color32,
    text: Color32,
    weak: Color32,
    accent: Color32,
    accent2: Color32,
    on_accent: Color32,
    shadow: Color32,
}

const NIGHT: Theme = Theme {
    bar: Color32::from_rgb(0x0E, 0x0E, 0x0E),
    panel: Color32::from_rgb(0x1C, 0x1C, 0x1E),
    card: Color32::from_rgb(0x2A, 0x2A, 0x2D),
    hover: Color32::from_rgb(0x3A, 0x3A, 0x3E),
    field: Color32::from_rgb(0x10, 0x10, 0x12),
    stroke: Color32::from_rgb(0x3C, 0x3C, 0x40),
    text: Color32::from_rgb(0xEE, 0xEE, 0xEE),
    weak: Color32::from_rgb(0x9C, 0x9C, 0xA2),
    accent: Color32::from_rgb(0xE8, 0x6A, 0x17),
    accent2: Color32::from_rgb(0xF2, 0xB1, 0x34),
    on_accent: Color32::from_rgb(0x11, 0x11, 0x11),
    shadow: Color32::from_rgb(0x05, 0x05, 0x05),
};

const DAY: Theme = Theme {
    bar: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    panel: Color32::from_rgb(0xF1, 0xF1, 0xF3),
    card: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    hover: Color32::from_rgb(0xE2, 0xE2, 0xE6),
    field: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    stroke: Color32::from_rgb(0xCC, 0xCC, 0xD2),
    text: Color32::from_rgb(0x1A, 0x1A, 0x1C),
    weak: Color32::from_rgb(0x6A, 0x6A, 0x70),
    accent: Color32::from_rgb(0xE0, 0x60, 0x10),
    accent2: Color32::from_rgb(0xF2, 0xB1, 0x34),
    on_accent: Color32::from_rgb(0x11, 0x11, 0x11),
    shadow: Color32::from_rgb(0xB8, 0xB8, 0xBE),
};

/// Dimensione del nome dell'app in alto a sinistra.
const TITLE_SIZE: f32 = 26.0;
/// Non tradotto: resta così in tutte le lingue.
const AUTHOR: &str = "Federico Villata - 2026";

static IS_DAY: AtomicBool = AtomicBool::new(false);

fn th() -> &'static Theme {
    if IS_DAY.load(Ordering::Relaxed) {
        &DAY
    } else {
        &NIGHT
    }
}

fn retro(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("retro".into()))
}

fn setup_fonts(ctx: &Context) {
    let mut defs = egui::FontDefinitions::default();
    defs.font_data
        .insert("righteous".into(), Arc::new(egui::FontData::from_static(FONTS[fonts::index_of("righteous")].data)));
    // cinese e giapponese: in fondo a ogni famiglia, usato solo per i caratteri che mancano agli altri
    defs.font_data.insert("cjk".into(), Arc::new(egui::FontData::from_static(fonts::CJK)));
    for fam in [FontFamily::Proportional, FontFamily::Monospace] {
        defs.families.entry(fam).or_default().push("cjk".into());
    }
    let mut retro_fam = vec!["righteous".to_string()];
    retro_fam.extend(defs.families[&FontFamily::Proportional].iter().cloned());
    defs.families.insert(FontFamily::Name("retro".into()), retro_fam);
    ctx.set_fonts(defs);
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = vec2(8.0, 7.0);
        s.spacing.button_padding = vec2(10.0, 5.0);
        s.text_styles.insert(egui::TextStyle::Body, FontId::proportional(14.5));
        s.text_styles.insert(egui::TextStyle::Button, FontId::proportional(14.5));
        s.text_styles.insert(egui::TextStyle::Heading, retro(20.0));
    });
}

fn apply_theme(ctx: &Context, day: bool) {
    IS_DAY.store(day, Ordering::Relaxed);
    let t = th();
    let mut v = if day { egui::Visuals::light() } else { egui::Visuals::dark() };
    v.panel_fill = t.panel;
    v.window_fill = t.panel;
    v.window_stroke = Stroke::new(1.0_f32, t.stroke);
    v.extreme_bg_color = t.field;
    v.faint_bg_color = t.card;
    v.override_text_color = Some(t.text);
    v.selection.bg_fill = t.accent;
    v.selection.stroke = Stroke::new(1.0_f32, t.text);
    v.hyperlink_color = t.accent;
    for w in [&mut v.widgets.inactive, &mut v.widgets.noninteractive] {
        w.bg_fill = t.card;
        w.weak_bg_fill = t.card;
        w.bg_stroke = Stroke::new(1.0_f32, t.stroke);
    }
    v.widgets.hovered.bg_fill = t.hover;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, t.accent2);
    v.widgets.active.bg_fill = t.accent;
    v.widgets.active.weak_bg_fill = t.accent;
    let pref = if day { egui::ThemePreference::Light } else { egui::ThemePreference::Dark };
    ctx.set_theme(pref);
    ctx.set_visuals_of(egui::Theme::Dark, v.clone());
    ctx.set_visuals_of(egui::Theme::Light, v);
}

// ---------------------------------------------------------------- stato

#[derive(Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    day: bool,
    lang: Lang,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { day: false, lang: Lang::from_system() }
    }
}

const SETTINGS_KEY: &str = "settings";

/// Come avviare la finestra (le opzioni da riga di comando).
#[derive(Default)]
pub struct Start {
    pub shot: Option<PathBuf>,
    pub lang: Option<Lang>,
    pub day: Option<bool>,
    /// Mette questa immagine nella prima etichetta, con la grafica "Foto Quadrata".
    pub image: Option<String>,
}

struct Shot {
    path: PathBuf,
    frames: u32,
    requested: bool,
}

pub struct JukeApp {
    project: Project,
    selected: usize,
    textures: HashMap<String, TextureHandle>,
    suggested_style: bool,
    status: String,
    import_open: bool,
    import_text: String,
    confirm_new: bool,
    sheet_page: usize,
    project_path: Option<PathBuf>,
    settings: Settings,
    shot: Option<Shot>,
}

impl JukeApp {
    pub fn new(cc: &eframe::CreationContext<'_>, start: Start) -> Self {
        let Start { shot, lang, day, image } = start;
        setup_fonts(&cc.egui_ctx);
        let mut project = Project::default();
        let mut settings = Settings::default();
        if shot.is_none() {
            if let Some(st) = cc.storage {
                if let Some(p) = eframe::get_value::<Project>(st, eframe::APP_KEY) {
                    if !p.labels.is_empty() {
                        project = p;
                    }
                }
                if let Some(s) = eframe::get_value::<Settings>(st, SETTINGS_KEY) {
                    settings = s;
                }
            }
        }
        if let Some(l) = lang {
            settings.lang = l;
        }
        if let Some(d) = day {
            settings.day = d;
        }
        if let Some(img) = image {
            let first = &mut project.labels[0];
            first.image = img;
            first.design = "foto".into();
            first.palette = "confetto".into();
            first.font = "archivo".into();
        }
        apply_theme(&cc.egui_ctx, settings.day);
        JukeApp {
            project,
            selected: 0,
            textures: HashMap::new(),
            suggested_style: true,
            status: tr(settings.lang).welcome.into(),
            import_open: false,
            import_text: String::new(),
            confirm_new: false,
            sheet_page: 0,
            project_path: None,
            settings,
            shot: shot.map(|path| Shot { path, frames: 0, requested: false }),
        }
    }

    fn t(&self) -> &'static T {
        tr(self.settings.lang)
    }

    fn texture(&mut self, ctx: &Context, key: String, px_per_mm: f32, make: impl FnOnce() -> Scene) -> TextureId {
        let key = format!("{key}@{px_per_mm:.2}");
        if let Some(t) = self.textures.get(&key) {
            return t.id();
        }
        if self.textures.len() > 900 {
            self.textures.clear();
        }
        let img = raster::to_color_image(&raster::rasterize(&make(), px_per_mm));
        let t = ctx.load_texture(&key, img, TextureOptions::LINEAR);
        let id = t.id();
        self.textures.insert(key, t);
        id
    }

    fn label_texture(&mut self, ctx: &Context, l: &Label, px_per_mm: f32) -> TextureId {
        let l2 = l.clone();
        self.texture(ctx, format!("L{}", l.key()), px_per_mm, move || designs::render_label(&l2))
    }

    fn current(&self) -> &Label {
        &self.project.labels[self.selected]
    }

    // ------------------------------------------------------------ azioni

    fn blank_like_current(&self) -> Label {
        let mut l = self.current().clone();
        l.artist.clear();
        l.title_a.clear();
        l.title_b.clear();
        l.artist_b.clear();
        l
    }

    fn add_label(&mut self) {
        let l = self.blank_like_current();
        self.project.labels.insert(self.selected + 1, l);
        self.selected += 1;
        self.status = self.t().label_added.into();
    }

    /// Nuovo progetto: tutte le etichette vanno via, ne resta una vuota con lo stile attuale.
    fn new_project(&mut self) {
        let mut l = self.blank_like_current();
        l.slogan.clear();
        l.image.clear();
        self.project.labels = vec![l];
        self.selected = 0;
        self.sheet_page = 0;
        self.project_path = None;
        self.status = self.t().new_done.into();
    }

    fn export_pdf(&mut self) {
        let t = self.t();
        let Some(path) = rfd::FileDialog::new().add_filter("PDF", &["pdf"]).set_file_name("jukebox-labels.pdf").save_file()
        else {
            return;
        };
        let scenes: Vec<Scene> = self.project.labels.iter().map(designs::render_label).collect();
        let bytes = pdf::build_pdf(&scenes, &self.project.sheet);
        self.status = match std::fs::write(&path, bytes) {
            Ok(()) => t.pdf_saved.replace("{p}", &path.display().to_string()),
            Err(e) => t.pdf_error.replace("{e}", &e.to_string()),
        };
    }

    fn save_project(&mut self) {
        let t = self.t();
        let mut dlg = rfd::FileDialog::new().add_filter(t.project_filter, &["json"]);
        dlg = match &self.project_path {
            Some(p) => dlg.set_file_name(p.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default()),
            None => dlg.set_file_name("jukebox-labels.json"),
        };
        let Some(path) = dlg.save_file() else { return };
        let res = serde_json::to_string_pretty(&self.project)
            .map_err(|e| e.to_string())
            .and_then(|s| std::fs::write(&path, s).map_err(|e| e.to_string()));
        self.status = match res {
            Ok(()) => t.project_saved.replace("{p}", &path.display().to_string()),
            Err(e) => t.save_error.replace("{e}", &e),
        };
        self.project_path = Some(path);
    }

    fn open_project(&mut self) {
        let t = self.t();
        let Some(path) = rfd::FileDialog::new().add_filter(t.project_filter, &["json"]).pick_file() else {
            return;
        };
        let res = std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|s| serde_json::from_str::<Project>(&s).map_err(|e| e.to_string()));
        match res {
            Ok(p) if !p.labels.is_empty() => {
                self.project = p;
                self.selected = 0;
                self.sheet_page = 0;
                self.status = t.opened.replace("{p}", &path.display().to_string());
                self.project_path = Some(path);
            }
            Ok(_) => self.status = t.no_labels_in_file.into(),
            Err(e) => self.status = t.open_error.replace("{e}", &e),
        }
    }

    fn choose_image(&mut self) {
        let t = self.t();
        let Some(path) = rfd::FileDialog::new()
            .add_filter(t.images_filter, &["png", "jpg", "jpeg", "webp", "bmp", "gif"])
            .pick_file()
        else {
            return;
        };
        let l = &mut self.project.labels[self.selected];
        l.image = path.display().to_string();
        // l'immagine si vede solo nella grafica con il riquadro: passa a quella
        if l.design != "foto" {
            l.design = "foto".into();
        }
    }

    fn import_list(&mut self) {
        let style = self.current().clone();
        let mut added = 0;
        for line in self.import_text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let (artist, a, b, artist_b) = parse_line(line);
            let mut l = style.clone();
            l.artist = artist;
            l.title_a = a;
            l.title_b = b;
            l.two_artists = !artist_b.is_empty();
            l.artist_b = artist_b;
            self.project.labels.push(l);
            added += 1;
        }
        if added > 0 {
            self.selected = self.project.labels.len() - added;
            self.import_text.clear();
            self.import_open = false;
        }
        self.status = self.t().imported.replace("{n}", &added.to_string());
    }

    // ------------------------------------------------------------ pannelli

    fn top_bar(&mut self, ctx: &Context) {
        let t = self.t();
        let frame = egui::Frame::new().fill(th().bar).inner_margin(10.0).stroke(Stroke::new(1.0_f32, th().stroke));
        egui::TopBottomPanel::top("top").exact_height(58.0).frame(frame).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                record_icon(ui);
                ui.label(RichText::new(t.app_title).font(retro(TITLE_SIZE)).color(th().accent2));
                // versione e autore: un terzo del nome, uguali in tutte le lingue
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 1.0;
                    ui.add_space(10.0);
                    let small = FontId::proportional(TITLE_SIZE / 3.0);
                    ui.label(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).font(small.clone()).color(th().weak));
                    ui.label(RichText::new(AUTHOR).font(small).color(th().weak));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    // un pulsante con menu, fatto come gli altri pulsanti della riga, così resta allineato
                    let mut lang = self.settings.lang;
                    ui.menu_button(format!("{}  ▼", lang.native_name()), |ui| {
                        for l in Lang::ALL {
                            if ui.selectable_label(l == lang, l.native_name()).clicked() {
                                lang = l;
                                ui.close_menu();
                            }
                        }
                    });
                    if lang != self.settings.lang {
                        self.settings.lang = lang;
                        self.status = tr(lang).welcome.into();
                    }
                    ui.label(RichText::new(t.language).color(th().weak));
                    ui.add_space(8.0);
                    let toggle = if self.settings.day { t.night } else { t.day };
                    if ui.button(toggle).clicked() {
                        self.settings.day = !self.settings.day;
                        apply_theme(ctx, self.settings.day);
                    }
                    ui.add_space(8.0);
                    if ui.button(t.save_project).clicked() {
                        self.save_project();
                    }
                    if ui.button(t.open_project).clicked() {
                        self.open_project();
                    }
                    if ui.button(t.new_project).clicked() {
                        self.confirm_new = true;
                    }
                });
            });
        });
    }

    fn list_panel(&mut self, ctx: &Context) {
        let t = self.t();
        egui::SidePanel::left("list").exact_width(272.0).resizable(false).show(ctx, |ui| {
            ui.add_space(6.0);
            ui.heading(t.labels.replace("{n}", &self.project.labels.len().to_string()));
            ui.horizontal_wrapped(|ui| {
                if ui.button(t.add).clicked() {
                    self.add_label();
                }
                if ui.button(t.duplicate).clicked() {
                    let l = self.current().clone();
                    self.project.labels.insert(self.selected + 1, l);
                    self.selected += 1;
                }
                let can_delete = self.project.labels.len() > 1;
                if ui.add_enabled(can_delete, egui::Button::new(t.delete)).clicked() {
                    self.project.labels.remove(self.selected);
                    self.selected = self.selected.min(self.project.labels.len() - 1);
                }
                if ui.add_enabled(self.selected > 0, egui::Button::new("⬆")).on_hover_text(t.move_up).clicked() {
                    self.project.labels.swap(self.selected, self.selected - 1);
                    self.selected -= 1;
                }
                let last = self.project.labels.len() - 1;
                if ui.add_enabled(self.selected < last, egui::Button::new("⬇")).on_hover_text(t.move_down).clicked() {
                    self.project.labels.swap(self.selected, self.selected + 1);
                    self.selected += 1;
                }
                if ui.button(t.import_list).clicked() {
                    self.import_open = true;
                }
            });
            ui.separator();
            let ppp = ctx.pixels_per_point();
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                let thumb = vec2(234.0, 75.0);
                for i in 0..self.project.labels.len() {
                    let l = self.project.labels[i].clone();
                    let id = self.label_texture(ctx, &l, thumb.x / LABEL_W * ppp);
                    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), thumb.y + 30.0), Sense::click());
                    let sel = i == self.selected;
                    let fill = if sel { th().accent } else if resp.hovered() { th().hover } else { th().card };
                    let p = ui.painter();
                    p.rect_filled(rect, CornerRadius::same(6), fill);
                    let img = Rect::from_min_size(rect.min + vec2(8.0, 6.0), thumb);
                    p.image(id, img, uv(), Color32::WHITE);
                    let text = format!("{}. {}", i + 1, l.summary(t.no_artist, t.no_title));
                    p.text(
                        pos2(rect.min.x + 10.0, img.max.y + 4.0),
                        Align2::LEFT_TOP,
                        elide(&text, 34),
                        FontId::proportional(13.0),
                        if sel { th().on_accent } else { th().text },
                    );
                    if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                        self.selected = i;
                    }
                    ui.add_space(2.0);
                }
            });
        });
    }

    fn print_panel(&mut self, ctx: &Context) {
        let t = self.t();
        egui::SidePanel::right("print").exact_width(266.0).resizable(false).show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.add_space(6.0);
                ui.heading(t.print);
                ui.label(RichText::new(t.paper).strong());
                let sheet = &mut self.project.sheet;
                for (p, name) in [(Paper::A4, t.paper_a4), (Paper::Letter, t.paper_letter), (Paper::Single, t.paper_single)] {
                    ui.radio_value(&mut sheet.paper, p, name);
                }
                ui.add_space(4.0);
                let multi = sheet.paper != Paper::Single;
                ui.add_enabled_ui(multi, |ui| {
                    ui.checkbox(&mut sheet.crop_marks, t.crop_marks);
                    ui.checkbox(&mut sheet.cut_lines, t.cut_lines);
                    ui.horizontal(|ui| {
                        ui.label(t.gap);
                        ui.add(egui::DragValue::new(&mut sheet.gap_mm).range(0.0..=10.0).speed(0.1).suffix(" mm"));
                    });
                    if sheet.crop_marks && sheet.gap_mm < 2.5 {
                        ui.label(RichText::new(t.gap_hint).color(th().weak).small());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(t.copies);
                    ui.add(egui::DragValue::new(&mut sheet.copies).range(1..=50));
                });
                let lay = pdf::layout(sheet);
                let total = self.project.labels.len() * sheet.copies.max(1) as usize;
                let pages = total.div_ceil(lay.per_page().max(1)).max(1);
                ui.add_space(6.0);
                egui::Frame::new().fill(th().card).corner_radius(CornerRadius::same(6)).inner_margin(10.0).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    if multi {
                        ui.label(
                            t.per_sheet
                                .replace("{c}", &lay.cols.to_string())
                                .replace("{r}", &lay.rows.to_string())
                                .replace("{n}", &lay.per_page().to_string()),
                        );
                    }
                    ui.label(t.totals.replace("{n}", &total.to_string()).replace("{p}", &pages.to_string()));
                    ui.label(RichText::new(t.print_hint).color(th().weak).small());
                });
                ui.add_space(8.0);
                if ui.add_sized([ui.available_width(), 40.0], big_button(t.export_pdf)).clicked() {
                    self.export_pdf();
                }
                ui.add_space(14.0);
                ui.separator();
                ui.heading(t.sheet);
                self.sheet_page = self.sheet_page.min(pages - 1);
                if pages > 1 {
                    ui.horizontal(|ui| {
                        if ui.add_enabled(self.sheet_page > 0, egui::Button::new("⬅")).on_hover_text(t.prev_page).clicked() {
                            self.sheet_page -= 1;
                        }
                        let label = t.page_of.replace("{i}", &(self.sheet_page + 1).to_string()).replace("{n}", &pages.to_string());
                        ui.label(RichText::new(label).strong());
                        if ui.add_enabled(self.sheet_page + 1 < pages, egui::Button::new("➡")).on_hover_text(t.next_page).clicked() {
                            self.sheet_page += 1;
                        }
                    });
                }
                self.sheet_preview(ui);
            });
        });
    }

    fn sheet_preview(&mut self, ui: &mut Ui) {
        let lay = pdf::layout(&self.project.sheet);
        let avail = ui.available_width();
        let k = (avail / lay.page_w).min(320.0 / lay.page_h);
        let size = vec2(lay.page_w * k, lay.page_h * k);
        let (rect, _) = ui.allocate_exact_size(vec2(avail, size.y + 4.0), Sense::hover());
        let page = Rect::from_min_size(pos2(rect.center().x - size.x / 2.0, rect.min.y), size);
        ui.painter().rect_filled(page.translate(vec2(3.0, 3.0)), CornerRadius::ZERO, th().shadow);
        ui.painter().rect_filled(page, CornerRadius::ZERO, Color32::WHITE);
        let ppp = ui.ctx().pixels_per_point();
        let ctx = ui.ctx().clone();
        let copies = self.project.sheet.copies.clamp(1, 50) as usize;
        let per = lay.per_page().max(1);
        let labels: Vec<Label> = self
            .project
            .labels
            .iter()
            .flat_map(|l| std::iter::repeat(l.clone()).take(copies))
            .skip(self.sheet_page * per)
            .take(per)
            .collect();
        for (i, l) in labels.iter().enumerate() {
            let (c, r) = (i % lay.cols, i / lay.cols);
            let x = lay.x0 + c as f32 * (LABEL_W + lay.gap);
            let y = lay.y0 + r as f32 * (LABEL_H + lay.gap);
            let lr = Rect::from_min_size(page.min + vec2(x * k, y * k), vec2(LABEL_W * k, LABEL_H * k));
            let id = self.label_texture(&ctx, l, (k * ppp * 1.5).max(1.0).round());
            ui.painter().image(id, lr, uv(), Color32::WHITE);
        }
        if self.project.sheet.crop_marks && self.project.sheet.paper != Paper::Single {
            let stroke = Stroke::new(1.0_f32, Color32::from_gray(40));
            for (x1, y1, x2, y2) in pdf::crop_marks(&lay, labels.len()) {
                ui.painter().line_segment([page.min + vec2(x1 * k, y1 * k), page.min + vec2(x2 * k, y2 * k)], stroke);
            }
        }
    }

    fn editor(&mut self, ctx: &Context) {
        let t = self.t();
        let mut scroll = egui::ScrollArea::vertical().auto_shrink([false, false]);
        if self.shot.is_some() {
            // la foto di controllo parte sempre dall'alto
            scroll = scroll.vertical_scroll_offset(0.0);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            scroll.show(ui, |ui| {
                ui.add_space(6.0);
                ui.heading(format!("{}  ·  {}", t.label_n.replace("{n}", &(self.selected + 1).to_string()), t.preview));
                ui.add_space(4.0);
                let ppp = ctx.pixels_per_point();
                let w = (ui.available_width() - 40.0).clamp(300.0, 720.0);
                let size = vec2(w, w * LABEL_H / LABEL_W);
                let l = self.current().clone();
                let id = self.label_texture(ctx, &l, (w / LABEL_W * ppp).round());
                let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), size.y + 16.0), Sense::hover());
                let img = Rect::from_min_size(pos2(rect.center().x - size.x / 2.0, rect.min.y + 4.0), size);
                ui.painter().rect_filled(img.translate(vec2(5.0, 6.0)), CornerRadius::same(3), th().shadow);
                ui.painter().image(id, img, uv(), Color32::WHITE);
                ui.painter().text(pos2(img.max.x, img.max.y + 3.0), Align2::RIGHT_TOP, "78 × 25 mm", FontId::proportional(11.0), th().weak);
                ui.add_space(6.0);

                self.text_fields(ui);
                ui.add_space(6.0);
                self.image_row(ui);
                ui.add_space(8.0);
                self.style_pickers(ui, ctx);
            });
        });
    }

    fn text_fields(&mut self, ui: &mut Ui) {
        let t = self.t();
        section(ui, t.texts);
        let l = &mut self.project.labels[self.selected];
        egui::Grid::new("fields").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
            let field = |ui: &mut Ui, label: &str, value: &mut String, hint: &str| {
                ui.label(RichText::new(label).strong());
                ui.add(
                    egui::TextEdit::singleline(value)
                        .hint_text(hint)
                        .desired_width(ui.available_width().min(520.0))
                        .font(FontId::proportional(17.0))
                        .margin(vec2(8.0, 6.0)),
                );
                ui.end_row();
            };
            ui.label(RichText::new(t.artists).strong());
            ui.horizontal_wrapped(|ui| {
                ui.radio_value(&mut l.two_artists, false, t.one_artist);
                ui.radio_value(&mut l.two_artists, true, t.two_artists);
            });
            ui.end_row();
            if l.two_artists {
                field(ui, t.title_a, &mut l.title_a, t.title_a_hint);
                field(ui, t.artist_a, &mut l.artist, t.artist_a_hint);
                field(ui, t.slogan, &mut l.slogan, t.slogan_hint);
                field(ui, t.artist_b, &mut l.artist_b, t.artist_b_hint);
                field(ui, t.title_b, &mut l.title_b, t.title_b_hint);
            } else {
                field(ui, t.artist, &mut l.artist, t.artist_hint);
                field(ui, t.title_a, &mut l.title_a, t.title_a_hint);
                field(ui, t.title_b, &mut l.title_b, t.title_b_hint);
            }
            ui.label("");
            ui.checkbox(&mut l.uppercase, t.uppercase);
            ui.end_row();
        });
    }

    fn image_row(&mut self, ui: &mut Ui) {
        let t = self.t();
        section(ui, t.image);
        ui.horizontal_wrapped(|ui| {
            if ui.button(t.choose_image).clicked() {
                self.choose_image();
            }
            let has = !self.current().image.is_empty();
            if ui.add_enabled(has, egui::Button::new(t.remove_image)).clicked() {
                self.project.labels[self.selected].image.clear();
            }
            let name = if has {
                std::path::Path::new(&self.current().image)
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_default()
            } else {
                t.no_image.to_string()
            };
            ui.label(RichText::new(name).color(th().weak));
        });
        ui.label(RichText::new(t.image_hint).color(th().weak).small());
    }

    fn style_pickers(&mut self, ui: &mut Ui, ctx: &Context) {
        let t = self.t();
        let lang = self.settings.lang;
        let ppp = ctx.pixels_per_point();
        let thumb = vec2(156.0, 50.0);
        let k = (thumb.x / LABEL_W * ppp * 1.25).round();

        section(ui, t.design);
        ui.checkbox(&mut self.suggested_style, t.use_suggested);
        let cur = self.current().clone();
        ui.horizontal_wrapped(|ui| {
            for d in DESIGNS {
                let mut l = cur.clone();
                l.design = d.id.into();
                if self.suggested_style {
                    l.palette = d.palette.into();
                    l.font = d.font.into();
                }
                let id = self.label_texture(ctx, &l, k);
                let name = i18n::design_name(lang, d.id, d.name);
                if card(ui, id, thumb, cur.design == d.id, name, i18n::era(lang, d.era)).clicked() {
                    self.project.labels[self.selected] = l;
                }
            }
        });

        ui.add_space(6.0);
        section(ui, t.colors);
        let cur = self.current().clone();
        ui.horizontal_wrapped(|ui| {
            for p in PALETTES {
                let mut l = cur.clone();
                l.palette = p.id.into();
                let id = self.label_texture(ctx, &l, k);
                let name = i18n::palette_name(lang, p.id, p.name);
                if card(ui, id, thumb, cur.palette == p.id, name, i18n::era(lang, p.era)).clicked() {
                    self.project.labels[self.selected].palette = p.id.into();
                }
            }
        });

        ui.add_space(6.0);
        section(ui, t.font);
        let cur = self.current().clone();
        ui.horizontal(|ui| {
            ui.label(t.font_titles);
            let titles = &mut self.project.labels[self.selected].font_titles;
            let shown = if titles.is_empty() { t.same_font.to_string() } else { FONTS[fonts::index_of(titles)].name.to_string() };
            egui::ComboBox::from_id_salt("font_titles").selected_text(shown).width(240.0).show_ui(ui, |ui| {
                ui.selectable_value(titles, String::new(), t.same_font);
                for f in FONTS {
                    ui.selectable_value(titles, f.id.to_string(), format!("{}  ·  {}", f.name, i18n::era(lang, f.era)));
                }
            });
        });
        ui.label(RichText::new(t.font_cards_hint).color(th().weak).small());
        let pal_i = designs::palette_index(&cur.palette);
        let fsize = vec2(156.0, 33.0);
        ui.horizontal_wrapped(|ui| {
            for (i, f) in FONTS.iter().enumerate() {
                let id = self.texture(ctx, format!("F{}{}", f.id, cur.palette), (fsize.x / 52.0 * ppp * 1.25).round(), || {
                    designs::render_font_sample(i, &PALETTES[pal_i])
                });
                if card(ui, id, fsize, cur.font == f.id, f.name, i18n::era(lang, f.era)).clicked() {
                    self.project.labels[self.selected].font = f.id.into();
                }
            }
        });

        ui.add_space(10.0);
        if ui.button(t.apply_all).clicked() {
            let cur = self.current().clone();
            for l in &mut self.project.labels {
                l.design = cur.design.clone();
                l.palette = cur.palette.clone();
                l.font = cur.font.clone();
                l.font_titles = cur.font_titles.clone();
                l.uppercase = cur.uppercase;
                l.image = cur.image.clone();
            }
            self.status = t.style_applied.into();
        }
        ui.add_space(20.0);
    }

    fn import_window(&mut self, ctx: &Context) {
        let t = self.t();
        let mut open = self.import_open;
        let mut do_import = false;
        egui::Window::new(t.import_title).open(&mut open).default_width(540.0).collapsible(false).show(ctx, |ui| {
            let ex = |ui: &mut Ui, s: &str| {
                ui.label(RichText::new(s).monospace().color(th().accent));
            };
            ui.label(t.import_one_per_line);
            ex(ui, "Artista - Titolo A / Titolo B");
            ex(ui, "Artista; Titolo A; Titolo B");
            ui.label(t.import_two);
            ex(ui, "Artista A - Titolo A | Artista B - Titolo B");
            ex(ui, "Artista A; Titolo A; Titolo B; Artista B");
            ui.label(RichText::new(t.import_style).color(th().weak));
            ui.add(egui::TextEdit::multiline(&mut self.import_text).desired_rows(10).desired_width(f32::INFINITY));
            if ui.add(big_button(t.import_add)).clicked() {
                do_import = true;
            }
        });
        self.import_open = open && !do_import;
        if do_import {
            self.import_list();
        }
    }

    fn confirm_new_window(&mut self, ctx: &Context) {
        if !self.confirm_new {
            return;
        }
        let t = self.t();
        let mut choice = None;
        egui::Window::new(t.new_project)
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.label(RichText::new(t.new_question).size(16.0));
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.add(big_button(t.new_confirm)).clicked() {
                        choice = Some(true);
                    }
                    if ui.button(t.cancel).clicked() {
                        choice = Some(false);
                    }
                });
            });
        if let Some(yes) = choice {
            self.confirm_new = false;
            if yes {
                self.new_project();
            }
        }
    }

    fn handle_screenshot(&mut self, ctx: &Context) {
        let Some(shot) = &mut self.shot else { return };
        shot.frames += 1;
        ctx.request_repaint();
        if shot.frames >= 120 && !shot.requested {
            shot.requested = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
        }
        let image = ctx.input(|i| {
            i.raw.events.iter().find_map(|e| match e {
                egui::Event::Screenshot { image, .. } => Some(image.clone()),
                _ => None,
            })
        });
        if let Some(img) = image {
            let [w, h] = img.size;
            let data: Vec<u8> = img.pixels.iter().flat_map(|c| [c.r(), c.g(), c.b(), 255]).collect();
            if let Some(pm) = tiny_skia::IntSize::from_wh(w as u32, h as u32).and_then(|s| tiny_skia::Pixmap::from_vec(data, s)) {
                if let Some(dir) = shot.path.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                if let Err(e) = pm.save_png(&shot.path) {
                    eprintln!("Impossibile salvare lo screenshot: {e}");
                }
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        } else if shot.frames > 900 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl eframe::App for JukeApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if self.project.labels.is_empty() {
            self.project.labels.push(Label::default());
        }
        self.selected = self.selected.min(self.project.labels.len() - 1);

        self.top_bar(ctx);
        let status_frame = egui::Frame::new().fill(th().bar).inner_margin(6.0);
        egui::TopBottomPanel::bottom("status").frame(status_frame).show(ctx, |ui| {
            ui.label(RichText::new(&self.status).color(th().weak));
        });
        self.list_panel(ctx);
        self.print_panel(ctx);
        self.editor(ctx);
        self.import_window(ctx);
        self.confirm_new_window(ctx);
        self.handle_screenshot(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if self.shot.is_none() {
            eframe::set_value(storage, eframe::APP_KEY, &self.project);
            eframe::set_value(storage, SETTINGS_KEY, &self.settings);
        }
    }
}

// ---------------------------------------------------------------- piccoli aiuti

fn uv() -> Rect {
    Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0))
}

fn elide(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(max - 1).collect::<String>())
    }
}

/// Accorcia il testo con "…" finché non sta nella larghezza data (vale anche per cinese e giapponese).
fn fit(ui: &Ui, s: &str, size: f32, max_w: f32) -> String {
    let width = |t: &str| {
        ui.fonts(|f| f.layout_no_wrap(t.to_string(), FontId::proportional(size), Color32::WHITE).size().x)
    };
    if width(s) <= max_w {
        return s.to_string();
    }
    let chars: Vec<char> = s.chars().collect();
    for n in (1..chars.len()).rev() {
        let t = format!("{}…", chars[..n].iter().collect::<String>().trim_end());
        if width(&t) <= max_w {
            return t;
        }
    }
    "…".into()
}

fn big_button(text: &str) -> egui::Button<'static> {
    egui::Button::new(RichText::new(text.to_string()).font(retro(17.0)).color(th().on_accent)).fill(th().accent2)
}

fn section(ui: &mut Ui, title: &str) {
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(6.0, 22.0), Sense::hover());
        ui.painter().rect_filled(r, CornerRadius::same(2), th().accent);
        ui.label(RichText::new(title).font(retro(19.0)).color(if IS_DAY.load(Ordering::Relaxed) { th().accent } else { th().accent2 }));
    });
}

/// Un piccolo disco in vinile accanto al titolo.
fn record_icon(ui: &mut Ui) {
    let (r, _) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::hover());
    let c = r.center();
    let p = ui.painter();
    p.circle_filled(c, 17.0, Color32::from_rgb(0x14, 0x14, 0x14));
    for rr in [14.5f32, 12.0, 9.5] {
        p.circle_stroke(c, rr, Stroke::new(1.0_f32, Color32::from_rgb(0x3A, 0x3A, 0x3A)));
    }
    p.circle_filled(c, 6.5, th().accent);
    p.circle_filled(c, 1.8, Color32::from_rgb(0x14, 0x14, 0x14));
}

fn card(ui: &mut Ui, tex: TextureId, size: Vec2, selected: bool, name: &str, sub: &str) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(size + vec2(12.0, 44.0), Sense::click());
    let fill = if selected { th().accent } else if resp.hovered() { th().hover } else { th().card };
    let p = ui.painter();
    p.rect_filled(rect, CornerRadius::same(7), fill);
    if !selected {
        p.rect_stroke(rect, CornerRadius::same(7), Stroke::new(1.0_f32, th().stroke), egui::StrokeKind::Inside);
    }
    let img = Rect::from_min_size(rect.min + vec2(6.0, 6.0), size);
    p.image(tex, img, uv(), Color32::WHITE);
    let (main, weak) = if selected { (th().on_accent, th().on_accent) } else { (th().text, th().weak) };
    let max_w = rect.width() - 8.0;
    p.text(pos2(rect.center().x, img.max.y + 4.0), Align2::CENTER_TOP, fit(ui, name, 13.0, max_w), FontId::proportional(13.0), main);
    p.text(pos2(rect.center().x, img.max.y + 20.0), Align2::CENTER_TOP, fit(ui, sub, 11.0, max_w), FontId::proportional(11.0), weak);
    resp.on_hover_text(name).on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Restituisce (artista, titolo A, titolo B, artista B); artista B vuoto = un solo artista.
fn parse_line(line: &str) -> (String, String, String, String) {
    if let Some((side_a, side_b)) = line.split_once('|') {
        let side = |s: &str| match s.split_once(" - ") {
            Some((artist, title)) => (artist.trim().to_string(), title.trim().to_string()),
            None => (String::new(), s.trim().to_string()),
        };
        let ((aa, ta), (ab, tb)) = (side(side_a), side(side_b));
        return (aa, ta, tb, ab);
    }
    let parts: Vec<String> = if line.contains(';') || line.contains('\t') {
        line.split([';', '\t']).map(|s| s.trim().to_string()).collect()
    } else if let Some((artist, rest)) = line.split_once(" - ") {
        let mut v = vec![artist.trim().to_string()];
        v.extend(rest.split(" / ").map(|s| s.trim().to_string()));
        v
    } else {
        vec![String::new(), line.to_string()]
    };
    let get = |i: usize| parts.get(i).cloned().unwrap_or_default();
    (get(0), get(1), get(2), get(3))
}

#[cfg(test)]
mod tests {
    use super::parse_line;

    #[test]
    fn parses_lines() {
        let t = |a: &str, b: &str, c: &str, d: &str| (a.to_string(), b.to_string(), c.to_string(), d.to_string());
        assert_eq!(parse_line("Lola - Uno / Due"), t("Lola", "Uno", "Due", ""));
        assert_eq!(parse_line("Lola; Uno"), t("Lola", "Uno", "", ""));
        assert_eq!(parse_line("Solo titolo"), t("", "Solo titolo", "", ""));
        assert_eq!(parse_line("Lola - Uno | Rita - Due"), t("Lola", "Uno", "Due", "Rita"));
        assert_eq!(parse_line("Lola; Uno; Due; Rita"), t("Lola", "Uno", "Due", "Rita"));
    }
}
