// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::path::Path;
use image::GenericImageView;
use image::imageops::FilterType;
#[tauri::command]
fn open_folder(path: String) {
    let path = Path::new(&path);
    showfile::show_path_in_file_manager(path);
}

#[tauri::command]
fn resize_image(mod_image: String, origin_image: String) -> Result<(), String> {
    let img_mod = image::open(&Path::new(&mod_image)).map_err(|e| format!("Failed to open image A: {}", e))?;
    let img_origin = image::open(&Path::new(&origin_image)).map_err(|e| format!("Failed to open image B: {}", e))?;
    
    let (width_a, height_a) = img_mod.dimensions();
    let (width_b, height_b) = img_origin.dimensions();
    
    if width_a != width_b || height_a != height_b {
        let resized_img_mod = img_mod.resize_exact(width_b, height_b, FilterType::Lanczos3);
        
        resized_img_mod.save(&Path::new(&mod_image))
            .map_err(|e| format!("Failed to save resized image A: {}", e))?;
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![open_folder, resize_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
