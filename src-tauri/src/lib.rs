use crate::config::{get_config, prepare_config, Config};
use crate::images::{general_fn, get_count_files, FilesCount};
use tauri::{AppHandle, Emitter, Manager};

pub mod config;
pub mod images;

#[tauri::command]
fn image_start(
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
    println!("config: {}", config.path_input);
    general_fn(config);

    // let files = fs::read_dir(config.path_input).map_err(|e| e.to_string())?;
    // for file in files {
    //     let file = file.map_err(|e| e.to_string())?;
    //     let path = file.path();
    //     let file_name = path.file_name().unwrap().to_str().unwrap();
    //     println!("{}", file_name);
    // }
    // let base_image: DynamicImage = ImageReader::open("1.png")
    //     .map_err(|e| e.to_string())?
    //     .decode()
    //     .map_err(|e| e.to_string())?;
    // let overlay_image: DynamicImage = ImageReader::open("2.png")
    //     .map_err(|e| e.to_string())?
    //     .decode()
    //     .map_err(|e| e.to_string())?;

    // let x = (base_image.width() - overlay_image.width()) / 2;
    // let y = (base_image.height() - overlay_image.height()) / 2;

    // let result = overlay_images(&base_image, &overlay_image, x, y);

    // result.save("result.png").map_err(|e| e.to_string())?;

    // println!("Изображение успешно сохранено как result.png");
    // Ok(())
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

    let watermark_count: i32 = get_count_files(&config.path_watermark);
    let inputs_count: i32 = get_count_files(&config.path_input);

    let payload = FilesCount {
        watermark: watermark_count,
        inputs: inputs_count,
    };
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
