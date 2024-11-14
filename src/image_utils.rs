use crate::Raindrop;
use crate::DROP_SIZE;

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

pub fn draw_square(buffer: &mut Vec<u32>, width: u32, height: u32, x: u32, y: u32, size: u32) {
    for dy in 0..size {
        for dx in 0..size {
            let px = x + dx;
            let py = y + dy;
            if px < width && py < height {
                buffer[(py * width + px) as usize] = 0xFFFF0000; // red
            }
        }
    }
}

pub fn draw_raindrop(buffer: &mut Vec<u32>, width: u32, height: u32, raindrop: &Raindrop) {
    // Color for the raindrop (white)
    let white = 0xFFFFFFFF;

    // Drawing the raindrop at its position
    for y in 0..DROP_SIZE {
        for x in 0..DROP_SIZE {
            let px = raindrop.x + x;
            let py = raindrop.y + y;
            if px < width && py < height {
                let index = (py * width + px) as usize;
                buffer[index] = white; // Set pixel color to white
            }
        }
    }
}


// Helper function to check collision between two rectangles
pub fn is_collision(rect1: (u32, u32, u32, u32), rect2: (u32, u32, u32, u32)) -> bool {
    let (x1, y1, w1, h1) = rect1;
    let (x2, y2, w2, h2) = rect2;

    x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
}
