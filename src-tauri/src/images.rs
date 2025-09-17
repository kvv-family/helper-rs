use crate::config::Config;
use image::{DynamicImage, ImageReader, Rgba};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Deserialize)]
pub struct Watermak {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilesCount {
    pub watermark: i32,
    pub inputs: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Progress {
    pub result_file: i32,
}

fn blend_channel(base: u8, overlay: u8, alpha: f32) -> u8 {
    (overlay as f32 * alpha + base as f32 * (1.0 - alpha)) as u8
}

fn blend_pixels(base: &Rgba<u8>, overlay: &Rgba<u8>) -> Rgba<u8> {
    let alpha = overlay[3] as f32 / 255.0;

    if alpha == 0.0 {
        return *base;
    }
    if alpha == 1.0 {
        return *overlay;
    }

    let r = blend_channel(base[0], overlay[0], alpha);
    let g = blend_channel(base[1], overlay[1], alpha);
    let b = blend_channel(base[2], overlay[2], alpha);
    let a = base[3].max(overlay[3]);

    Rgba([r, g, b, a])
}

pub fn overlay_images(
    base_image: &DynamicImage,
    overlay_image: &DynamicImage,
    x: u32,
    y: u32,
) -> DynamicImage {
    let mut result = base_image.to_rgba8();

    for (ox, oy, pixel) in overlay_image.to_rgba8().enumerate_pixels() {
        let target_x = x + ox;
        let target_y = y + oy;

        if target_x < base_image.width() && target_y < base_image.height() {
            // Альфа-блендинг
            let base_pixel = result.get_pixel(target_x, target_y);
            let blended = blend_pixels(base_pixel, pixel);
            result.put_pixel(target_x, target_y, blended);
        }
    }

    DynamicImage::ImageRgba8(result)
}

pub fn get_count_files(path: &String) -> i32 {
    let mut files_count: i32 = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        files_count += 1;
                    }
                } else {
                    println!(
                        "Error reading metadata: {}",
                        entry.metadata().unwrap_err().to_string()
                    );
                }
            } else {
                println!("Error reading entry: {}", entry.unwrap_err().to_string());
            }
        }
    } else {
        println!("Error reading directory: {}", path);
    }
    return files_count;
}

pub fn prepare_index(index: i32) -> String {
    format!("{:03}", index)
}

pub fn report_progress(app: &AppHandle, result_file: i32) {
    let payload: Progress = Progress {
        result_file: result_file,
    };
    app.emit("report_progress", &payload).unwrap();
}

fn watermark_fn(config: Config, app: AppHandle) {
    let mut result_file: i32 = 0;
    let watermark_files: fs::ReadDir = match fs::read_dir(&config.path_watermark) {
        Ok(files) => files,
        Err(e) => {
            println!("Error reading watermark files: {}", e);
            return;
        }
    };
    for watermark_file in watermark_files {
        let watermark_file = watermark_file.unwrap();
        let watermark_name: String = watermark_file
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .nth(0)
            .unwrap()
            .to_string();
        // Создание директории с названием водянного знака
        let out_dir: String = config.path_output.clone() + "/" + watermark_name.as_str();
        fs::create_dir_all(&out_dir).unwrap();
        let input_files = match fs::read_dir(&config.path_input) {
            Ok(files) => files,
            Err(e) => {
                println!("Error reading input files: {}", e);
                continue;
            }
        };
        let mut index: i32 = 1;
        for input_file in input_files {
            let input_file = input_file.unwrap();
            let file_input_name: String = input_file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split(".")
                .nth(0)
                .unwrap()
                .to_string();
            let file_input_format: String = input_file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split(".")
                .nth(1)
                .unwrap()
                .to_string();
            let wm_file = ImageReader::open(watermark_file.path())
                .unwrap()
                .decode()
                .unwrap();
            let in_file = ImageReader::open(input_file.path())
                .unwrap()
                .decode()
                .unwrap();
            let x: u32 = (in_file.width() - wm_file.width()) / 2;
            let y: u32 = (in_file.height() - wm_file.height()) / 2;
            let result: DynamicImage = overlay_images(&in_file, &wm_file, x, y);
            let mut outname: String = prepare_index(index) + "." + file_input_format.as_str();
            if config.name_output_file == "origin" {
                outname = file_input_name + "." + file_input_format.as_str();
            }
            result
                .save(out_dir.clone() + "/" + outname.as_str())
                .unwrap();
            index += 1;
            result_file += 1;
            report_progress(&app, result_file);
        }
    }
    app.emit("ready", "ready").unwrap();
}

fn open_image(path: &String) -> DynamicImage {
    let image = match ImageReader::open(path) {
        Ok(image) => image,
        Err(e) => {
            panic!("Error opening image: {}", e);
        }
    };
    let image = match image.decode() {
        Ok(image) => image,
        Err(e) => {
            panic!("Error decoding image: {}", e);
        }
    };
    return image;
}

fn index_fn(config: Config, app: AppHandle) {
    let mut result_file: i32 = 0;
    let input_files = match fs::read_dir(&config.path_input) {
        Ok(files) => files,
        Err(e) => {
            println!("Error reading input files: {}", e);
            return;
        }
    };

    for input_file in input_files {
        let input_file = input_file.unwrap();
        let file_input_name: String = input_file
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .nth(0)
            .unwrap()
            .to_string();
        let file_input_format: String = input_file
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .nth(1)
            .unwrap()
            .to_string();
        let out_dir: String = config.path_output.clone() + "/" + file_input_name.as_str();
        fs::create_dir_all(&out_dir).unwrap();
        let mut index: i32 = 1;
        let watermark_files: fs::ReadDir = match fs::read_dir(&config.path_watermark) {
            Ok(files) => files,
            Err(e) => {
                panic!("Error reading watermark files: {}", e);
            }
        };
        for watermark_file in watermark_files {
            let watermark_file = watermark_file.unwrap();
            let watermark_name: String = watermark_file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split(".")
                .nth(0)
                .unwrap()
                .to_string();
            let mut out_name = prepare_index(index) + "." + file_input_format.as_str();
            if config.name_output_file == "origin" {
                out_name = watermark_name + "." + file_input_format.as_str();
            }
            let wm_file = open_image(&watermark_file.path().to_str().unwrap().to_string());
            let in_file = open_image(&input_file.path().to_str().unwrap().to_string());

            let x: u32 = (in_file.width() - wm_file.width()) / 2;
            let y: u32 = (in_file.height() - wm_file.height()) / 2;
            let result: DynamicImage = overlay_images(&in_file, &wm_file, x, y);
            match result.save(out_dir.clone() + "/" + out_name.as_str()) {
                Ok(_) => (),
                Err(e) => println!("Error saving result: {}", e),
            }

            index += 1;
            result_file += 1;
            report_progress(&app, result_file);
        }
    }
    app.emit("ready", "ready").unwrap();
}

pub fn general_fn(config: Config, app: AppHandle) {
    if config.name_output == "watermark" {
        watermark_fn(config, app);
    } else if config.name_output == "index" {
        index_fn(config, app);
    }
}

pub fn get_count_config(config: &Config) -> FilesCount {
    let watermark_count: i32 = get_count_files(&config.path_watermark);
    let inputs_count: i32 = get_count_files(&config.path_input);

    let payload: FilesCount = FilesCount {
        watermark: watermark_count,
        inputs: inputs_count,
    };
    return payload;
}
