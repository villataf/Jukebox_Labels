# 点唱机标签

[Italiano](README.md) · [English](README.en.md) · [Español](README.es.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · **中文** · [日本語](README.ja.md)

版本 0.6.1 · Federico Villata - 2026

一个为点唱机唱片制作标签（歌名条）的程序，尺寸精确为 **78 × 25 毫米**，提供 70、80、90 年代风格的
设计、颜色和字体。可导出可直接打印的矢量 PDF。支持 Windows、macOS 和 Linux，适用于 Intel 和 ARM 处理器。

## 使用方法

1. 在输入框中填写**歌手**和**歌名**（A 面，以及可选的 B 面）。
2. 选择**设计**、**颜色**和**字体**：预览会立即更新。
3. 用 **➕ 新建** 添加更多标签，或用 **导入列表…** 粘贴一份列表。
4. 在 **打印 PDF** 面板的下拉菜单中选择**纸张尺寸**，以及**纵向**或**横向**。
5. 点击 **导出 PDF…**，并按 100%（“实际大小”）打印，不要缩放到页面。

## 功能

- **歌手**、**A 面歌名** 和 **B 面歌名**（可选）输入框；可选择全部使用大写字母。
- 两种排版：两面同一位歌手，或每面一位歌手
  （A 面歌名、A 面歌手、重复的中间文字、B 面歌手、B 面歌名，与当年的标签一样）。
- 24 种 70、80、90 年代风格的设计，其中包括受 80 年代意大利点唱机标签启发的鲜艳设计：
  飞转唱片、小丑菱格、双色、夏日帆船、幸运轮盘、打孔票券、点唱机面板、星光音乐节、
  太阳光芒、斯卡棋盘格、波普闪电、风衣色块。
- **方形照片**：左侧一个方框，放入你自己选择的图片
  （PNG、JPG、WebP、BMP 或 GIF，自动居中裁切）。
- 23 套配色和 24 种复古字体，可自由组合；歌名可以使用与歌手不同的字体。
  中文和日文名称使用 Noto Sans SC 显示。
- 过长的文字会自动缩小以放进标签。
- 标签列表：添加、复制、删除、调整顺序、导入粘贴的列表。
  格式：`歌手 - 歌名A / 歌名B`、`歌手; 歌名A; 歌名B`，每面一位歌手时用
  `歌手A - 歌名A | 歌手B - 歌名B` 或 `歌手A; 歌名A; 歌名B; 歌手B`。
- 纸张尺寸，在下拉菜单中选择：

  | 纸张 | 尺寸 | 每张纸的标签数（纵向） | 横向 |
  |---|---|---|---|
  | A3 | 297 × 420 毫米 | 39 | 36 |
  | A4 | 210 × 297 毫米 | 18 | 18 |
  | A5 | 148 × 210 毫米 | 6 | 8 |
  | A6 | 105 × 148 毫米 | 4 | 3 |
  | Letter | 216 × 279 毫米 | 18 | 18 |
  | 每页一张标签 | 78 × 25 毫米 | 1 | 1 |

  每种纸张都可以纵向或横向使用。
  在纸张上，标签之间间隔 4 毫米，四角带裁切标记。标签间距、灰色细边框和每张标签的份数都可以调整。
- 当有多页时，纸张预览可以翻页。
- 深灰/黑色界面或白天模式；语言：意大利语、英语、西班牙语、德语、法语、中文、日语。
- **新建项目** 会一次删除所有标签（会先确认）。
- 可保存和打开项目；关闭程序时也会记住当前工作。项目保存的是图片的位置，而不是图片本身：
  如果图片文件被移动，需要重新选择。

## 编译

```
cargo build --release
```

其他平台和处理器（Intel 和 ARM）：

| 系统 | Intel | ARM |
|---|---|---|
| Windows | `x86_64-pc-windows-msvc` | `aarch64-pc-windows-msvc` |
| macOS | `x86_64-apple-darwin` | `aarch64-apple-darwin` |
| Linux | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu` |

先运行 `rustup target add <target>`，再在对应的系统上运行 `cargo build --release --target <target>`。
文件 `.github/workflows/build.yml` 会在 GitHub 上编译全部六个版本。
Linux 需要 `libgtk-3`、`libxkbcommon` 和 `libwayland` 库。

左上角显示的版本号来自 `Cargo.toml`。

## 图标

图标（点唱机标签后面的一张黑胶唱片）在 `src/icon.rs` 中绘制；
文件位于 `assets/icon`：`icon.ico`（Windows）、`icon.icns`（macOS）以及 16 到 1024 像素的 PNG。

- **Windows**：每次编译时，`build.rs` 会把图标以及名称和版本嵌入 `.exe` 文件。
- **macOS**：先运行 `cargo install cargo-bundle`，再用 `cargo bundle --release` 生成带图标的 `.app`。
- **Linux**：`cargo bundle --release --format deb` 生成带图标的软件包；或者把
  `assets/icon/icon-256.png` 以 `jukebox-etichette.png` 为名复制到 `~/.local/share/icons`，并把
  `assets/linux/jukebox-etichette.desktop` 复制到 `~/.local/share/applications`。
- 修改图标：编辑 `src/icon.rs`，然后用 `--make-icons assets/icon` 运行程序。

## 命令行选项

- `--screenshot 文件.png` 打开窗口、绘制并保存窗口截图。
- `--export-pdf 文件.pdf` 不打开窗口，直接生成包含示例标签的 PDF。
- `--gallery 文件.png` 保存一张包含所有设计的图片。
- `--make-icons 文件夹` 重新生成图标文件。
- `--lang it|en|es|de|fr|zh|ja`、`--day` 和 `--image 照片.jpg` 可在启动时试用语言、白天模式和照片设计。

## 字体

`assets/fonts` 中的字体来自 Google Fonts：SIL Open Font License（`OFL.txt`）；
Permanent Marker、Special Elite 和 Ultra 为 Apache 2.0（`APACHE-LICENSE.txt`）。
