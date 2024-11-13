use image::{GenericImageView, Rgba};

pub fn convert_to_mono(image_data: &[u8]) -> Vec<u32> {
    let img = image::load_from_memory(image_data).expect("Failed to load image");
    let (width, height) = img.dimensions();
    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];

    for (x, y, pixel) in img.pixels() {
        let Rgba([r, g, b, _]) = pixel;
        let grayscale = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
        // 1D array (buffer) to represent a 2D image
        buffer[(y * width + x) as usize] = (grayscale as u32)
            | ((grayscale as u32) << 8)
            | ((grayscale as u32) << 16)
            | 0xFF000000;
    }

    buffer
}
