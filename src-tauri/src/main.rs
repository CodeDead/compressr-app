// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open(site: &str) -> Result<(), String> {
    match open::that(site) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
