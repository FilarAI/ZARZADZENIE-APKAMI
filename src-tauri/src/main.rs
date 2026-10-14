use std::process::Command;

// Aplikacje lokalne: id z ui/index.html (data-id) → ścieżka względem katalogu domowego.
const LOKALNE: &[(&str, &str)] = &[("dokumenty", "Desktop/Dokumenty FILAR.app")];

/// Otwiera adres https w domyślnej przeglądarce albo aplikację lokalną z listy wyżej.
#[tauri::command]
fn otworz(cel: String) -> Result<(), String> {
    let co = if cel.starts_with("https://") {
        cel
    } else {
        let (_, sciezka) = LOKALNE
            .iter()
            .find(|(id, _)| *id == cel)
            .ok_or("Nieznana aplikacja")?;
        let dom = std::env::var("HOME").map_err(|e| e.to_string())?;
        format!("{dom}/{sciezka}")
    };
    let ok = Command::new("open")
        .arg(&co)
        .status()
        .map_err(|e| e.to_string())?
        .success();
    if ok {
        Ok(())
    } else {
        Err(format!("Nie udało się otworzyć: {co}"))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![otworz])
        .run(tauri::generate_context!())
        .expect("nie udało się uruchomić aplikacji");
}
