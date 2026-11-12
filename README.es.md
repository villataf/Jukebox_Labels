# Etiquetas Jukebox

[Italiano](README.md) · [English](README.en.md) · **Español** · [Deutsch](README.de.md) · [Français](README.fr.md) · [中文](README.zh.md) · [日本語](README.ja.md)

Versión 0.6.1 · Federico Villata - 2026

Un programa para crear las tiras de título de los discos de una jukebox, con la medida exacta de
**78 × 25 mm**, con diseños, colores y fuentes al estilo de los años 70, 80 y 90. Exporta un PDF
vectorial listo para imprimir. Funciona en Windows, macOS y Linux, con procesadores Intel y ARM.

## Cómo se usa

1. Escribe el **artista** y los **títulos** en los recuadros (cara A y, si quieres, cara B).
2. Elige un **diseño**, los **colores** y la **fuente**: la vista previa se actualiza al momento.
3. Añade más etiquetas con **➕ Nueva**, o pega una lista con **Importar lista…**.
4. Elige el **tamaño del papel** en el menú desplegable del panel **Imprimir PDF** y si usarlo en **vertical** u **horizontal**.
5. Pulsa **Exportar PDF…** e imprime el archivo al 100 % («tamaño real»), sin ajustarlo a la página.

## Qué hace

- Recuadros para **Artista**, **Título cara A** y **Título cara B** (opcional); texto en mayúsculas si quieres.
- Dos composiciones: un artista para las dos caras, o un artista por cara
  (título A, artista A, texto central repetido, artista B, título B, como las etiquetas de la época).
- 24 diseños de los años 70, 80 y 90, entre ellos algunos de colores vivos inspirados en las etiquetas
  italianas de jukebox de los 80: Disco volador, Arlequín, Dos colores, Vela de verano,
  Rueda de la fortuna, Billete perforado, Panel jukebox, Festival de estrellas, Rayos de sol,
  Cuadros ska, Rayo pop, Cortavientos.
- **Foto cuadrada**: un recuadro cuadrado a la izquierda con una imagen que tú eliges
  (PNG, JPG, WebP, BMP o GIF, recortada por el centro).
- 23 paletas de colores y 24 fuentes retro, combinables libremente; los títulos pueden usar una
  fuente distinta de la de los artistas. Los nombres en chino y japonés se escriben con Noto Sans SC.
- Los textos demasiado largos se reducen solos para caber en la etiqueta.
- Lista de etiquetas: añadir, duplicar, eliminar, reordenar, importar una lista pegada.
  Formatos: `Artista - Título A / Título B`, `Artista; Título A; Título B` y, con un artista por cara,
  `Artista A - Título A | Artista B - Título B` o `Artista A; Título A; Título B; Artista B`.
- Tamaños de papel, elegidos en un menú desplegable:

  | Papel | Medidas | Etiquetas por hoja en vertical | en horizontal |
  |---|---|---|---|
  | A3 | 297 × 420 mm | 39 | 36 |
  | A4 | 210 × 297 mm | 18 | 18 |
  | A5 | 148 × 210 mm | 6 | 8 |
  | A6 | 105 × 148 mm | 4 | 3 |
  | Letter | 216 × 279 mm | 18 | 18 |
  | Una etiqueta por página | 78 × 25 mm | 1 | 1 |

  Cada hoja se puede usar en vertical o en horizontal.
  En las hojas, las etiquetas están separadas 4 mm, con marcas de corte en las esquinas.
  El espacio entre etiquetas, el contorno gris y las copias de cada etiqueta se pueden cambiar.
- La vista previa de la hoja pasa de una página a otra cuando hay más de una.
- Interfaz gris oscuro/negro o modo día; idiomas: italiano, inglés, español, alemán, francés,
  chino, japonés.
- **Nuevo proyecto** elimina todas las etiquetas de una vez (tras una confirmación).
- Guarda y abre proyectos; el trabajo también se recuerda al cerrar el programa. El proyecto guarda
  la ubicación de la imagen, no la imagen: si el archivo se mueve, hay que elegirlo de nuevo.

## Compilar

```
cargo build --release
```

Para las demás plataformas y procesadores (Intel y ARM):

| Sistema | Intel | ARM |
|---|---|---|
| Windows | `x86_64-pc-windows-msvc` | `aarch64-pc-windows-msvc` |
| macOS | `x86_64-apple-darwin` | `aarch64-apple-darwin` |
| Linux | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu` |

`rustup target add <target>` y después `cargo build --release --target <target>` en el sistema
correspondiente. El archivo `.github/workflows/build.yml` compila las seis versiones en GitHub.
En Linux se necesitan las bibliotecas `libgtk-3`, `libxkbcommon` y `libwayland`.

La versión que aparece arriba a la izquierda es la indicada en `Cargo.toml`.

## Icono

El icono (un disco de vinilo detrás de una etiqueta de jukebox) está dibujado en `src/icon.rs`;
los archivos están en `assets/icon`: `icon.ico` (Windows), `icon.icns` (macOS) y PNG de 16 a 1024 px.

- **Windows**: `build.rs` lo incorpora al archivo `.exe` en cada compilación, con nombre y versión.
- **macOS**: `cargo install cargo-bundle` y después `cargo bundle --release` crea la app `.app` con el icono.
- **Linux**: `cargo bundle --release --format deb` crea el paquete con el icono, o copia
  `assets/icon/icon-256.png` como `jukebox-etichette.png` en `~/.local/share/icons` y
  `assets/linux/jukebox-etichette.desktop` en `~/.local/share/applications`.
- Para cambiarlo: edita `src/icon.rs` y ejecuta el programa con `--make-icons assets/icon`.

## Opciones de línea de comandos

- `--screenshot archivo.png` abre la ventana, la dibuja y guarda una imagen de ella.
- `--export-pdf archivo.pdf` crea un PDF con las etiquetas de ejemplo, sin ventana.
- `--gallery archivo.png` guarda una imagen con todos los diseños.
- `--make-icons carpeta` vuelve a generar los archivos del icono.
- `--lang it|en|es|de|fr|zh|ja`, `--day` e `--image foto.jpg` para probar al arrancar un idioma,
  el modo día y el diseño con foto.

## Fuentes

Las fuentes de `assets/fonts` proceden de Google Fonts: licencia SIL Open Font License (`OFL.txt`)
y, para Permanent Marker, Special Elite y Ultra, Apache 2.0 (`APACHE-LICENSE.txt`).
