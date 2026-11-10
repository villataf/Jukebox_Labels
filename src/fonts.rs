//! Caratteri retrò inclusi nel programma (Google Fonts, licenze OFL / Apache 2.0).

use std::sync::OnceLock;
use ttf_parser::Face;

pub struct FontInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub era: &'static str,
    pub data: &'static [u8],
}

macro_rules! font {
    ($id:expr, $name:expr, $era:expr, $file:expr) => {
        FontInfo {
            id: $id,
            name: $name,
            era: $era,
            data: include_bytes!(concat!("../assets/fonts/", $file)),
        }
    };
}

pub static FONTS: &[FontInfo] = &[
    font!("bebas", "Bebas Neue", "Classico", "BebasNeue-Regular.ttf"),
    font!("shrikhand", "Shrikhand", "Anni 70", "Shrikhand-Regular.ttf"),
    font!("righteous", "Righteous", "Anni 70", "Righteous-Regular.ttf"),
    font!("pacifico", "Pacifico", "Anni 70", "Pacifico-Regular.ttf"),
    font!("monoton", "Monoton", "Anni 80", "Monoton-Regular.ttf"),
    font!("audiowide", "Audiowide", "Anni 80", "Audiowide-Regular.ttf"),
    font!("pressstart", "Press Start 2P", "Anni 80", "PressStart2P-Regular.ttf"),
    font!("bungee", "Bungee", "Anni 80", "Bungee-Regular.ttf"),
    font!("vt323", "VT323", "Anni 90", "VT323-Regular.ttf"),
    font!("bangers", "Bangers", "Anni 90", "Bangers-Regular.ttf"),
    font!("marker", "Permanent Marker", "Anni 90", "PermanentMarker-Regular.ttf"),
    font!("elite", "Special Elite", "Anni 90", "SpecialElite-Regular.ttf"),
    font!("anton", "Anton", "Anni 80", "Anton-Regular.ttf"),
    font!("fjalla", "Fjalla One", "Anni 80", "FjallaOne-Regular.ttf"),
    font!("archivo", "Archivo Black", "Anni 80", "ArchivoBlack-Regular.ttf"),
    font!("passion", "Passion One", "Anni 80", "PassionOne-Regular.ttf"),
    font!("squada", "Squada One", "Anni 80", "SquadaOne-Regular.ttf"),
    font!("russo", "Russo One", "Anni 80", "RussoOne-Regular.ttf"),
    font!("rubikmono", "Rubik Mono One", "Anni 80", "RubikMonoOne-Regular.ttf"),
    font!("alfaslab", "Alfa Slab One", "Anni 70", "AlfaSlabOne-Regular.ttf"),
    font!("ultra", "Ultra", "Anni 70", "Ultra-Regular.ttf"),
    font!("titan", "Titan One", "Anni 70", "TitanOne-Regular.ttf"),
    font!("lobster", "Lobster", "Anni 70", "Lobster-Regular.ttf"),
    font!("bowlby", "Bowlby One SC", "Anni 70", "BowlbyOneSC-Regular.ttf"),
];

static FACES: OnceLock<Vec<Face<'static>>> = OnceLock::new();

/// Carattere per cinese e giapponese (Noto Sans SC, OFL): per l'interfaccia e per i nomi
/// scritti in questi alfabeti sulle etichette.
pub static CJK: &[u8] = include_bytes!("../assets/fonts/NotoSansSC-VF.ttf");

/// Tutti i caratteri: quelli di FONTS, poi quello cinese/giapponese.
pub fn faces() -> &'static [Face<'static>] {
    FACES.get_or_init(|| {
        let mut v: Vec<Face<'static>> = FONTS
            .iter()
            .map(|f| Face::parse(f.data, 0).expect("carattere incluso non valido"))
            .collect();
        let mut cjk = Face::parse(CJK, 0).expect("carattere CJK non valido");
        // carattere variabile: le etichette lo usano in grassetto
        cjk.set_variation(ttf_parser::Tag::from_bytes(b"wght"), 700.0);
        v.push(cjk);
        v
    })
}

pub fn index_of(id: &str) -> usize {
    FONTS.iter().position(|f| f.id == id).unwrap_or(0)
}

/// Font usati quando un carattere non contiene una lettera (accenti rari, cinese, giapponese).
pub fn fallbacks() -> [usize; 3] {
    [2, 0, FONTS.len()]
}
