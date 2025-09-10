use crate::config::{get_config, set_config, Config};
use crate::images::overlay_images;
use image::{DynamicImage, ImageReader};
use std::fs;

pub mod config;
pub mod images;

#[tauri::command]
fn image_start(
    path_input: String,
    // watermark_path: String,
    // output_path: String,
    // name_output: String,
    // name_output_file: String,
    // format_output: String,
) -> Result<(), String> {
    let mut config: Config = get_config();
    config.path_input = path_input;
    // config.path_watermark = watermark_path;
    // config.path_output = output_path;
    // config.name_output = name_output;
    // config.name_output_file = name_output_file;
    // config.format_output = format_output;
    config = set_config(config);
    println!("config: {}", config.path_input);

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
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = get_config();

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![image_start])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
