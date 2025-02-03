mod commands;
mod settings;

use commands::connection::{
    clear_disaster_info, get_settings, get_widgets, post_disaster_info, post_settings, scroll,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            #[cfg(mobile)]
            app.handle()
                .plugin(tauri_plugin_barcode_scanner::init())
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            post_settings,
            post_disaster_info,
            clear_disaster_info,
            scroll,
            get_widgets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
