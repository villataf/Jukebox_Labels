//! Su Windows incorpora l'icona e le informazioni del programma nel file .exe.

fn main() {
    println!("cargo:rerun-if-changed=assets/icon/icon.ico");
    println!("cargo:rerun-if-changed=build.rs");
    #[cfg(windows)]
    windows_resources();
}

#[cfg(windows)]
fn windows_resources() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let icon = "assets/icon/icon.ico";
    if !std::path::Path::new(icon).exists() {
        println!("cargo:warning=manca {icon}: rigeneralo con `--make-icons assets/icon`");
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon(icon)
        .set("ProductName", "Etichette Jukebox")
        .set("FileDescription", "Etichette Jukebox: title strip 78 x 25 mm in PDF")
        .set("OriginalFilename", "jukebox-etichette.exe");
    if let Err(e) = res.compile() {
        // senza gli strumenti delle risorse di Windows il programma funziona lo stesso, solo senza icona
        println!("cargo:warning=icona non incorporata nell'eseguibile: {e}");
    }
}
