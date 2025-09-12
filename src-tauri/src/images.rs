use crate::config::Config;
use image::{DynamicImage, Rgba};
use serde::{Deserialize, Serialize};
use std::fs;

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

pub fn general_fn(config: Config) {
    let res = fs::read_dir(config.path_input).map_err(|e| e.to_string());
    let _ = match res {
        Ok(files) => files,
        Err(_) => panic!(""),
    };
}
