use crate::config::get_config;
use crate::images::commands::{get_count, handler_config, image_start};
use tauri::Manager;

pub mod config;
pub mod images;
pub mod database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = get_config();

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            image_start,
            get_count,
            handler_config
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Debug)
                        .build(),
                )?;
            }
            app.manage(config);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
