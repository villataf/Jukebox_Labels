# Jukebox-Etiketten

[Italiano](README.md) · [English](README.en.md) · **Deutsch** · [Français](README.fr.md) · [中文](README.zh.md) · [日本語](README.ja.md)

Ein Programm für die Titelstreifen der Platten einer Jukebox, in genau **78 × 25 mm**, mit
Designs, Farben und Schriften im Stil der 70er, 80er und 90er Jahre. Es exportiert ein
druckfertiges Vektor-PDF. Es läuft unter Windows, macOS und Linux, auf Intel- und ARM-Prozessoren.

## So geht's

1. **Interpret** und **Titel** in die Felder schreiben (Seite A und, wenn gewünscht, Seite B).
2. Ein **Design**, die **Farben** und die **Schrift** wählen: die Vorschau ändert sich sofort.
3. Weitere Etiketten mit **➕ Neu** anlegen oder eine Liste mit **Liste importieren…** einfügen.
4. **PDF exportieren…** drücken und die Datei mit 100 % („Originalgröße“) drucken, ohne sie an die Seite anzupassen.

## Was es kann

- Felder für **Interpret**, **Titel Seite A** und **Titel Seite B** (optional); auf Wunsch alles in Großbuchstaben.
- Zwei Aufteilungen: ein Interpret für beide Seiten oder ein Interpret pro Seite
  (Titel A, Interpret A, wiederholter Text in der Mitte, Interpret B, Titel B, wie die Etiketten von damals).
- 24 Designs aus den 70ern, 80ern und 90ern, darunter knallbunte, inspiriert von italienischen
  Jukebox-Etiketten der 80er: Fliegende Platte, Harlekin, Zweifarbig, Sommersegel, Glücksrad,
  Perforiertes Ticket, Jukebox-Feld, Sternenfestival, Sonnenstrahlen, Ska-Karos, Pop-Blitz,
  Windjacke.
- **Quadratisches Foto**: ein quadratisches Feld links mit einem eigenen Bild
  (PNG, JPG, WebP, BMP oder GIF, mittig zugeschnitten).
- 23 Farbpaletten und 24 Retro-Schriften, frei kombinierbar; die Titel können eine andere Schrift
  als die Interpreten haben. Namen auf Chinesisch und Japanisch werden mit Noto Sans SC geschrieben.
- Zu lange Texte werden automatisch kleiner, damit sie auf das Etikett passen.
- Etikettenliste: hinzufügen, duplizieren, löschen, umsortieren, eine eingefügte Liste importieren.
  Formate: `Interpret - Titel A / Titel B`, `Interpret; Titel A; Titel B` und, mit einem Interpreten pro Seite,
  `Interpret A - Titel A | Interpret B - Titel B` oder `Interpret A; Titel A; Titel B; Interpret B`.
- PDF auf A4- oder Letter-Blättern (auf A4 18 Etiketten pro Blatt, 4 mm Abstand, mit Schnittmarken
  an den Ecken) oder ein Etikett pro Seite in 78 × 25 mm.
  Abstand, grauer Rand und Anzahl der Kopien je Etikett sind einstellbar.
- Die Blattvorschau blättert durch die Seiten, wenn es mehr als eine gibt.
- Oberfläche in Dunkelgrau/Schwarz oder im Tagmodus; Sprachen: Italienisch, Englisch, Deutsch,
  Französisch, Chinesisch, Japanisch.
- **Neues Projekt** löscht alle Etiketten auf einmal (nach einer Rückfrage).
- Projekte speichern und öffnen; die Arbeit bleibt auch nach dem Schließen erhalten. Ein Projekt
  speichert den Speicherort des Bildes, nicht das Bild selbst: wird die Datei verschoben, neu wählen.

## Kompilieren

```
cargo build --release
```

Für die anderen Plattformen und Prozessoren (Intel und ARM):

| System | Intel | ARM |
|---|---|---|
| Windows | `x86_64-pc-windows-msvc` | `aarch64-pc-windows-msvc` |
| macOS | `x86_64-apple-darwin` | `aarch64-apple-darwin` |
| Linux | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu` |

`rustup target add <target>`, dann `cargo build --release --target <target>` auf dem passenden
System. Die Datei `.github/workflows/build.yml` baut alle sechs Versionen auf GitHub.
Unter Linux werden die Bibliotheken `libgtk-3`, `libxkbcommon` und `libwayland` benötigt.

## Symbol

Das Symbol (eine Vinylplatte hinter einem Jukebox-Etikett) ist in `src/icon.rs` gezeichnet;
die Dateien liegen in `assets/icon`: `icon.ico` (Windows), `icon.icns` (macOS) und PNGs von 16 bis 1024 px.

- **Windows**: `build.rs` bettet es bei jedem Kompilieren mit Name und Version in die `.exe` ein.
- **macOS**: `cargo install cargo-bundle`, dann erstellt `cargo bundle --release` die `.app` mit Symbol.
- **Linux**: `cargo bundle --release --format deb` erstellt das Paket mit Symbol, oder
  `assets/icon/icon-256.png` als `jukebox-etichette.png` nach `~/.local/share/icons` und
  `assets/linux/jukebox-etichette.desktop` nach `~/.local/share/applications` kopieren.
- Zum Ändern: `src/icon.rs` bearbeiten und das Programm mit `--make-icons assets/icon` starten.

## Befehlszeilenoptionen

- `--screenshot datei.png` öffnet das Fenster, zeichnet es und speichert ein Bild davon.
- `--export-pdf datei.pdf` erstellt ohne Fenster ein PDF mit den Beispiel-Etiketten.
- `--gallery datei.png` speichert ein Bild mit allen Designs.
- `--make-icons ordner` erzeugt die Symboldateien neu.
- `--lang it|en|de|fr|zh|ja`, `--day` und `--image foto.jpg`, um beim Start eine Sprache,
  den Tagmodus und das Foto-Design auszuprobieren.

## Schriften

Die Schriften in `assets/fonts` stammen von Google Fonts: SIL Open Font License (`OFL.txt`) und,
für Permanent Marker, Special Elite und Ultra, Apache 2.0 (`APACHE-LICENSE.txt`).
