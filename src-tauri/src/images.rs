use image::{DynamicImage, Rgba};

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
