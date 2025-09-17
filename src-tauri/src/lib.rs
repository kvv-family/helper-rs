use crate::config::{get_config, prepare_config, Config};
use crate::images::{general_fn, get_count_config, FilesCount};
use tauri::{AppHandle, Emitter, Manager};
use std::thread;

pub mod config;
pub mod images;

#[tauri::command]
fn image_start(
    app: AppHandle,
    path_input: String,
    watermark_path: String,
    output_path: String,
    name_output: String,
    name_output_file: String,
    format_output: String,
) {
    let config = prepare_config(
        path_input,
        watermark_path,
        output_path,
        name_output,
        name_output_file,
        format_output,
    );
    let payload: FilesCount = get_count_config(&config);
    app.emit("files_count", &payload).unwrap();
    thread::spawn(move || {
        general_fn(config, app);
    });
}

#[tauri::command]
fn get_count(
    app: AppHandle,
    path_input: String,
    watermark_path: String,
    output_path: String,
    name_output: String,
    name_output_file: String,
    format_output: String,
) {
    let config: Config = prepare_config(
        path_input,
        watermark_path,
        output_path,
        name_output,
        name_output_file,
        format_output,
    );
    let payload: FilesCount = get_count_config(&config);
    app.emit("files_count", &payload).unwrap();
}

#[tauri::command]
fn handler_config(app: AppHandle) {
    let config = get_config();
    app.emit("handler_config", &config).unwrap();
}

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
