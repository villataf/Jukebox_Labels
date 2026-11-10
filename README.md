# Etichette Jukebox

**Italiano** · [English](README.en.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [中文](README.zh.md) · [日本語](README.ja.md)

Programma per creare le etichette (title strip) dei dischi di un juke-box, alla misura esatta
di **78 × 25 mm**, con grafiche, colori e caratteri in stile anni 70, 80 e 90. Esporta un PDF
vettoriale pronto da stampare. Funziona su Windows, macOS e Linux, con processori Intel e ARM.

## Come si usa

1. Scrivi **artista** e **titoli** nei box (lato A e, se vuoi, lato B).
2. Scegli una **grafica**, i **colori** e il **carattere**: l'anteprima si aggiorna subito.
3. Aggiungi altre etichette con **➕ Nuova**, oppure incolla un elenco con **Importa elenco…**.
4. Premi **Esporta PDF…** e stampa il file al 100% ("dimensioni effettive"), senza adattarlo alla pagina.

## Cosa fa

- Box per **Artista**, **Titolo lato A** e **Titolo lato B** (facoltativo); testo in maiuscolo a scelta.
- Due impaginazioni: un artista per entrambi i lati, oppure un artista per ogni lato
  (titolo A, artista A, scritta centrale ripetuta, artista B, titolo B, come le etichette d'epoca).
- 24 grafiche anni 70, 80 e 90, tra cui quelle a colori accesi ispirate alle etichette italiane
  degli anni 80: Disco Volante, Arlecchino, Doppio Colore, Vela d'Estate, Ruota della Fortuna,
  Biglietto Perforato, Pannello Juke-box, Festival Stelle, Raggi di Sole, Scacchi Ska, Lampo Pop,
  Giacca a Vento.
- **Foto Quadrata**: un riquadro quadrato a sinistra con un'immagine scelta da te
  (PNG, JPG, WebP, BMP o GIF, ritagliata al centro).
- 23 palette di colori e 24 caratteri retrò, combinabili liberamente; i titoli possono avere
  un carattere diverso da quello degli artisti. I nomi in cinese e giapponese vengono scritti
  con Noto Sans SC.
- I testi troppo lunghi si rimpiccioliscono da soli per stare nell'etichetta.
- Elenco di etichette: aggiungi, duplica, elimina, riordina, importa un elenco incollato.
  Formati: `Artista - Titolo A / Titolo B`, `Artista; Titolo A; Titolo B` e, con un artista per lato,
  `Artista A - Titolo A | Artista B - Titolo B` oppure `Artista A; Titolo A; Titolo B; Artista B`.
- PDF su foglio A4 o Letter (sull'A4 18 etichette per foglio, distanziate di 4 mm, con gli
  indicatori di ritaglio agli angoli) oppure una etichetta per pagina a 78 × 25 mm.
  Lo spazio tra le etichette, il contorno grigio e le copie di ogni etichetta si possono cambiare.
- L'anteprima del foglio scorre tra le pagine quando ce n'è più di una.
- Interfaccia grigio scuro/nero oppure modalità giorno; lingue: italiano, inglese, tedesco,
  francese, cinese, giapponese.
- **Nuovo progetto** elimina in un colpo tutte le etichette (dopo una conferma).
- Salva e apri i progetti; il lavoro viene anche ricordato alla chiusura. Il progetto salva il
  percorso dell'immagine, non l'immagine: se il file viene spostato va scelto di nuovo.

## Compilare

```
cargo build --release
```

Per le altre piattaforme e processori (Intel e ARM):

| Sistema | Intel | ARM |
|---|---|---|
| Windows | `x86_64-pc-windows-msvc` | `aarch64-pc-windows-msvc` |
| macOS | `x86_64-apple-darwin` | `aarch64-apple-darwin` |
| Linux | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu` |

`rustup target add <target>` e poi `cargo build --release --target <target>` sul sistema
corrispondente. Il file `.github/workflows/build.yml` compila tutte e sei le versioni su GitHub.
Su Linux servono le librerie `libgtk-3`, `libxkbcommon` e `libwayland`.

## Icona

L'icona (un disco in vinile dietro un'etichetta da juke-box) è disegnata in `src/icon.rs`;
i file sono in `assets/icon`: `icon.ico` (Windows), `icon.icns` (macOS) e PNG da 16 a 1024 px.

- **Windows**: `build.rs` la incorpora nel file `.exe` a ogni compilazione, con nome e versione.
- **macOS**: `cargo install cargo-bundle` e poi `cargo bundle --release` crea l'app `.app` con l'icona.
- **Linux**: `cargo bundle --release --format deb` crea il pacchetto con l'icona, oppure copia
  `assets/icon/icon-256.png` come `jukebox-etichette.png` in `~/.local/share/icons` e
  `assets/linux/jukebox-etichette.desktop` in `~/.local/share/applications`.
- Per cambiarla: modifica `src/icon.rs` e lancia il programma con `--make-icons assets/icon`.

## Opzioni da riga di comando

- `--screenshot file.png` apre la finestra, la disegna e ne salva l'immagine.
- `--export-pdf file.pdf` crea un PDF con le etichette di esempio, senza finestra.
- `--gallery file.png` salva un'immagine con tutte le grafiche.
- `--make-icons cartella` rigenera i file dell'icona.
- `--lang it|en|de|fr|zh|ja`, `--day` e `--image foto.jpg` per provare lingua, modalità giorno
  e grafica con la foto all'avvio.

## Caratteri

I caratteri in `assets/fonts` vengono da Google Fonts: licenza SIL Open Font License
(`OFL.txt`) e, per Permanent Marker, Special Elite e Ultra, Apache 2.0 (`APACHE-LICENSE.txt`).
