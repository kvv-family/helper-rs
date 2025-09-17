use crate::config::{get_config, prepare_config, Config};
use crate::images::lib::{general_fn, get_count_config};
use crate::images::structs::FilesCount;
use std::thread;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn image_start(
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
pub fn get_count(
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
pub fn handler_config(app: AppHandle) {
    let config = get_config();
    app.emit("handler_config", &config).unwrap();
}