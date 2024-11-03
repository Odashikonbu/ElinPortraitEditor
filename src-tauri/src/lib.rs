// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::path::Path;
#[tauri::command]
fn open_folder(path: String) {
    let path = Path::new(&path);
    showfile::show_path_in_file_manager(path);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![open_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
