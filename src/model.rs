use serde::{Deserialize, Serialize};

pub const LABEL_W: f32 = 78.0;
pub const LABEL_H: f32 = 25.0;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Label {
    /// Artista del lato A (o di entrambi i lati).
    pub artist: String,
    pub title_a: String,
    pub title_b: String,
    /// Un artista diverso per ogni lato.
    pub two_artists: bool,
    pub artist_b: String,
    /// Scritta piccola nella fascia centrale (solo con due artisti), ripetuta se c'è spazio.
    pub slogan: String,
    pub design: String,
    pub palette: String,
    /// Carattere degli artisti.
    pub font: String,
    /// Carattere dei titoli: vuoto = uguale a quello degli artisti.
    pub font_titles: String,
    pub uppercase: bool,
    /// Immagine personale (percorso del file) per le grafiche con il riquadro foto.
    pub image: String,
}

impl Default for Label {
    fn default() -> Self {
        Label {
            artist: String::new(),
            title_a: String::new(),
            title_b: String::new(),
            two_artists: false,
            artist_b: String::new(),
            slogan: String::new(),
            design: "classico".into(),
            palette: "crema".into(),
            font: "bebas".into(),
            font_titles: String::new(),
            uppercase: true,
            image: String::new(),
        }
    }
}

impl Label {
    pub fn sample(artist: &str, a: &str, b: &str, design: &str, palette: &str, font: &str) -> Label {
        Label {
            artist: artist.into(),
            title_a: a.into(),
            title_b: b.into(),
            design: design.into(),
            palette: palette.into(),
            font: font.into(),
            ..Label::default()
        }
    }

    pub fn sample_two(a: (&str, &str), b: (&str, &str), slogan: &str, design: &str, palette: &str, font: &str) -> Label {
        Label {
            artist: a.0.into(),
            title_a: a.1.into(),
            two_artists: true,
            artist_b: b.0.into(),
            title_b: b.1.into(),
            slogan: slogan.into(),
            ..Label::sample("", "", "", design, palette, font)
        }
    }

    /// Chiave che cambia ogni volta che cambia l'aspetto dell'etichetta.
    pub fn key(&self) -> String {
        format!(
            "{}\u{1}{}\u{1}{}\u{1}{}\u{1}{}\u{1}{}\u{1}{}\u{1}{}\u{1}{}\u{1}{}\u{1}{}\u{1}{}\u{1}{:?}",
            self.artist,
            self.title_a,
            self.title_b,
            self.two_artists,
            self.artist_b,
            self.slogan,
            self.design,
            self.palette,
            self.font,
            self.font_titles,
            self.uppercase,
            self.image,
            // se il file dell'immagine cambia, l'anteprima si aggiorna
            std::fs::metadata(&self.image).and_then(|m| m.modified()).ok()
        )
    }

    /// "Artista – Titolo", con i segnaposto dati per i campi vuoti.
    pub fn summary(&self, no_artist: &str, no_title: &str) -> String {
        let or = |s: &str, d: &str| if s.trim().is_empty() { d.to_string() } else { s.trim().to_string() };
        let mut s = format!("{} – {}", or(&self.artist, no_artist), or(&self.title_a, no_title));
        if self.two_artists {
            s.push_str(&format!(" / {}", or(&self.artist_b, no_artist)));
        }
        s
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Paper {
    A4,
    Letter,
    /// Una pagina grande come l'etichetta (78 × 25 mm).
    Single,
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Sheet {
    pub paper: Paper,
    /// Indicatori di ritaglio (crocini) agli angoli di ogni etichetta.
    pub crop_marks: bool,
    /// Contorno grigio sottile attorno a ogni etichetta.
    pub cut_lines: bool,
    pub gap_mm: f32,
    pub copies: u32,
}

impl Default for Sheet {
    fn default() -> Self {
        Sheet { paper: Paper::A4, crop_marks: true, cut_lines: false, gap_mm: 4.0, copies: 1 }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Project {
    pub labels: Vec<Label>,
    pub sheet: Sheet,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            labels: vec![
                Label::sample("I Cosmonauti", "Notte al Juke-Box", "Ballo del Sabato", "tramonto", "tramonto", "shrikhand"),
                Label::sample("Lola Neon", "Cuore Elettrico", "Luna di Vinile", "neon", "synth", "audiowide"),
                Label::sample_two(
                    ("Gli Astronauti", "Estate al Mare"),
                    ("Rita Lampo", "Ciao Ciao Amore"),
                    "Festival d'Estate",
                    "vela",
                    "limone",
                    "anton",
                ),
                Label::sample("Groove Machine", "Strada Funky", "Pista da Ballo", "classico", "crema", "bebas"),
                Label::sample_two(
                    ("Mister Disco", "Ballando sotto le Stelle"),
                    ("Le Comete", "Notte Rosa"),
                    "Juke-box d'Oro",
                    "doppio",
                    "verdearancio",
                    "fjalla",
                ),
            ],
            sheet: Sheet::default(),
        }
    }
}
