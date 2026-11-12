# Étiquettes Jukebox

[Italiano](README.md) · [English](README.en.md) · [Español](README.es.md) · [Deutsch](README.de.md) · **Français** · [中文](README.zh.md) · [日本語](README.ja.md)

Version 0.6.1 · Federico Villata - 2026

Un programme pour créer les bandes-titres des disques d'un juke-box, au format exact de
**78 × 25 mm**, avec des designs, des couleurs et des polices dans le style des années 70, 80 et 90.
Il exporte un PDF vectoriel prêt à imprimer. Il fonctionne sous Windows, macOS et Linux, sur
processeurs Intel et ARM.

## Mode d'emploi

1. Saisissez l'**artiste** et les **titres** dans les champs (face A et, si vous voulez, face B).
2. Choisissez un **design**, les **couleurs** et la **police** : l'aperçu se met à jour aussitôt.
3. Ajoutez d'autres étiquettes avec **➕ Nouvelle**, ou collez une liste avec **Importer une liste…**.
4. Choisissez le **format de la feuille** dans le menu déroulant du panneau **Imprimer en PDF**, en **portrait** ou en **paysage**.
5. Cliquez sur **Exporter en PDF…** et imprimez le fichier à 100 % (« taille réelle »), sans l'ajuster à la page.

## Fonctionnalités

- Champs **Artiste**, **Titre face A** et **Titre face B** (facultatif) ; texte en majuscules au choix.
- Deux mises en page : un artiste pour les deux faces, ou un artiste par face
  (titre A, artiste A, texte central répété, artiste B, titre B, comme les étiquettes d'époque).
- 24 designs des années 70, 80 et 90, dont des designs aux couleurs vives inspirés des étiquettes
  italiennes de juke-box des années 80 : Disque volant, Arlequin, Bicolore, Voile d'été,
  Roue de la fortune, Ticket perforé, Panneau juke-box, Festival étoilé, Rayons de soleil,
  Damier ska, Éclair pop, Coupe-vent.
- **Photo carrée** : un carré à gauche avec une image de votre choix
  (PNG, JPG, WebP, BMP ou GIF, recadrée au centre).
- 23 palettes de couleurs et 24 polices rétro, librement combinables ; les titres peuvent avoir
  une autre police que les artistes. Les noms en chinois et en japonais sont écrits avec Noto Sans SC.
- Les textes trop longs rétrécissent d'eux-mêmes pour tenir sur l'étiquette.
- Liste d'étiquettes : ajouter, dupliquer, supprimer, réordonner, importer une liste collée.
  Formats : `Artiste - Titre A / Titre B`, `Artiste; Titre A; Titre B` et, avec un artiste par face,
  `Artiste A - Titre A | Artiste B - Titre B` ou `Artiste A; Titre A; Titre B; Artiste B`.
- Formats de feuille, choisis dans un menu déroulant :

  | Feuille | Dimensions | Étiquettes par feuille, portrait | paysage |
  |---|---|---|---|
  | A3 | 297 × 420 mm | 39 | 36 |
  | A4 | 210 × 297 mm | 18 | 18 |
  | A5 | 148 × 210 mm | 6 | 8 |
  | A6 | 105 × 148 mm | 4 | 3 |
  | Letter | 216 × 279 mm | 18 | 18 |
  | Une étiquette par page | 78 × 25 mm | 1 | 1 |

  Chaque feuille peut être utilisée en portrait ou en paysage.
  Sur les feuilles, les étiquettes sont espacées de 4 mm, avec des traits de coupe aux coins.
  L'espace entre les étiquettes, le contour gris et le nombre de copies sont réglables.
- L'aperçu de la feuille passe d'une page à l'autre quand il y en a plusieurs.
- Interface gris foncé/noir ou mode jour ; langues : italien, anglais, espagnol, allemand,
  français, chinois, japonais.
- **Nouveau projet** supprime toutes les étiquettes d'un coup (après confirmation).
- Enregistrez et ouvrez vos projets ; le travail est aussi conservé à la fermeture. Un projet
  garde l'emplacement de l'image, pas l'image elle-même : si le fichier est déplacé, choisissez-le à nouveau.

## Compiler

```
cargo build --release
```

Pour les autres plateformes et processeurs (Intel et ARM) :

| Système | Intel | ARM |
|---|---|---|
| Windows | `x86_64-pc-windows-msvc` | `aarch64-pc-windows-msvc` |
| macOS | `x86_64-apple-darwin` | `aarch64-apple-darwin` |
| Linux | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu` |

`rustup target add <target>`, puis `cargo build --release --target <target>` sur le système
correspondant. Le fichier `.github/workflows/build.yml` compile les six versions sur GitHub.
Sous Linux, il faut les bibliothèques `libgtk-3`, `libxkbcommon` et `libwayland`.

La version affichée en haut à gauche est celle indiquée dans `Cargo.toml`.

## Icône

L'icône (un disque vinyle derrière une étiquette de juke-box) est dessinée dans `src/icon.rs` ;
les fichiers sont dans `assets/icon` : `icon.ico` (Windows), `icon.icns` (macOS) et des PNG de 16 à 1024 px.

- **Windows** : `build.rs` l'intègre au fichier `.exe` à chaque compilation, avec le nom et la version.
- **macOS** : `cargo install cargo-bundle`, puis `cargo bundle --release` crée l'app `.app` avec l'icône.
- **Linux** : `cargo bundle --release --format deb` crée le paquet avec l'icône, ou copiez
  `assets/icon/icon-256.png` sous le nom `jukebox-etichette.png` dans `~/.local/share/icons` et
  `assets/linux/jukebox-etichette.desktop` dans `~/.local/share/applications`.
- Pour la modifier : éditez `src/icon.rs` et lancez le programme avec `--make-icons assets/icon`.

## Options en ligne de commande

- `--screenshot fichier.png` ouvre la fenêtre, la dessine et en enregistre une image.
- `--export-pdf fichier.pdf` crée un PDF avec les étiquettes d'exemple, sans fenêtre.
- `--gallery fichier.png` enregistre une image avec tous les designs.
- `--make-icons dossier` régénère les fichiers de l'icône.
- `--lang it|en|es|de|fr|zh|ja`, `--day` et `--image photo.jpg` pour essayer au démarrage une langue,
  le mode jour et le design avec photo.

## Polices

Les polices de `assets/fonts` viennent de Google Fonts : licence SIL Open Font License (`OFL.txt`)
et, pour Permanent Marker, Special Elite et Ultra, Apache 2.0 (`APACHE-LICENSE.txt`).
