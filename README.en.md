# Jukebox Labels

[Italiano](README.md) · **English** · [Deutsch](README.de.md) · [Français](README.fr.md) · [中文](README.zh.md) · [日本語](README.ja.md)

A program for making the title strips of a jukebox's records, at the exact size of
**78 × 25 mm**, with designs, colours and fonts in the style of the 70s, 80s and 90s. It exports a
vector PDF ready to print. It runs on Windows, macOS and Linux, on Intel and ARM processors.

## How to use it

1. Type the **artist** and the **titles** in the boxes (side A and, if you like, side B).
2. Choose a **design**, the **colours** and the **font**: the preview updates straight away.
3. Add more labels with **➕ New**, or paste a list with **Import list…**.
4. Press **Export PDF…** and print the file at 100% ("actual size"), without fitting it to the page.

## What it does

- Boxes for **Artist**, **Side A title** and **Side B title** (optional); text in capitals if you want.
- Two layouts: one artist for both sides, or one artist per side
  (title A, artist A, repeated centre text, artist B, title B, like the labels of the time).
- 24 designs from the 70s, 80s and 90s, including bright ones inspired by Italian jukebox labels
  of the 80s: Spinning Record, Harlequin, Two-Tone, Summer Sail, Wheel of Fortune,
  Perforated Ticket, Jukebox Panel, Star Festival, Sunburst, Ska Checkers, Pop Lightning,
  Windbreaker.
- **Square Photo**: a square box on the left with a picture of your choice
  (PNG, JPG, WebP, BMP or GIF, cropped to the centre).
- 23 colour palettes and 24 retro fonts, freely combined; titles can use a different font
  from the artists. Names in Chinese and Japanese are written with Noto Sans SC.
- Text that is too long shrinks by itself to fit on the label.
- List of labels: add, duplicate, delete, reorder, import a pasted list.
  Formats: `Artist - Title A / Title B`, `Artist; Title A; Title B` and, with one artist per side,
  `Artist A - Title A | Artist B - Title B` or `Artist A; Title A; Title B; Artist B`.
- PDF on A4 or Letter sheets (on A4, 18 labels per sheet, 4 mm apart, with crop marks at the
  corners) or one label per page at 78 × 25 mm.
  The space between labels, the grey outline and the copies of each label can be changed.
- The sheet preview pages through the sheets when there is more than one.
- Dark grey/black interface or day mode; languages: Italian, English, German, French,
  Chinese, Japanese.
- **New project** deletes all labels in one go (after asking).
- Save and open projects; your work is also remembered when you close the program. A project
  stores the picture's location, not the picture itself: if the file is moved, choose it again.

## Building

```
cargo build --release
```

For the other platforms and processors (Intel and ARM):

| System | Intel | ARM |
|---|---|---|
| Windows | `x86_64-pc-windows-msvc` | `aarch64-pc-windows-msvc` |
| macOS | `x86_64-apple-darwin` | `aarch64-apple-darwin` |
| Linux | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu` |

`rustup target add <target>`, then `cargo build --release --target <target>` on the matching
system. The file `.github/workflows/build.yml` builds all six versions on GitHub.
Linux needs the `libgtk-3`, `libxkbcommon` and `libwayland` libraries.

## Icon

The icon (a vinyl record behind a jukebox label) is drawn in `src/icon.rs`;
the files are in `assets/icon`: `icon.ico` (Windows), `icon.icns` (macOS) and PNGs from 16 to 1024 px.

- **Windows**: `build.rs` embeds it in the `.exe` file on every build, with name and version.
- **macOS**: `cargo install cargo-bundle`, then `cargo bundle --release` creates the `.app` with the icon.
- **Linux**: `cargo bundle --release --format deb` creates the package with the icon, or copy
  `assets/icon/icon-256.png` as `jukebox-etichette.png` into `~/.local/share/icons` and
  `assets/linux/jukebox-etichette.desktop` into `~/.local/share/applications`.
- To change it: edit `src/icon.rs` and run the program with `--make-icons assets/icon`.

## Command-line options

- `--screenshot file.png` opens the window, draws it and saves a picture of it.
- `--export-pdf file.pdf` creates a PDF with the sample labels, without a window.
- `--gallery file.png` saves a picture with all the designs.
- `--make-icons folder` regenerates the icon files.
- `--lang it|en|de|fr|zh|ja`, `--day` and `--image photo.jpg` to try a language, day mode
  and the photo design at start-up.

## Fonts

The fonts in `assets/fonts` come from Google Fonts: SIL Open Font License (`OFL.txt`) and,
for Permanent Marker, Special Elite and Ultra, Apache 2.0 (`APACHE-LICENSE.txt`).
