use crate::Raindrop;
use crate::Scale;
use crate::Font;
use crate::DROP_SIZE;

use image::{GenericImageView, Rgba};
// Render text using rusttype
pub fn draw_text(buffer: &mut Vec<u32>, width: u32, height: u32, text: &str, x: u32, y: u32) {
    let font_data = include_bytes!("../assets/FiraSans-SemiBold.otf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Failed to load font");

    let scale = Scale::uniform(31.0); // Font size
    let start = rusttype::point(x as f32, y as f32);
    let layout = font.layout(text, scale, start);

    for glyph in layout {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, v| {
                let px = bounding_box.min.x + gx as i32;
                let py = bounding_box.min.y + gy as i32;

                if px >= 0 && py >= 0 && px < width as i32 && py < height as i32 {
                    let idx = (py as u32 * width + px as u32) as usize;
                    let alpha = (v * 255.0) as u32;
                    buffer[idx] = (255 << 24) | (alpha << 16) | (alpha << 8) | alpha; // White text
                }
            });
        }
    }
}


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

pub fn get_background_for_score<'a>(
    score: i32,
    mono_background: &'a [u32],
    mouse1_background: &'a [u32],
    mouse2_background: &'a [u32],
    original_background: &'a [u32],
    winner_background: &'a [u32],
) -> Vec<u32> {
    match score {
        s if s > 29 => winner_background.to_vec(),
        s if s < -30 => mono_background.to_vec(),
        s if s <= -10 && s >= -20 => mouse1_background.to_vec(),
        s if s <= -21 && s >= -30 => mouse2_background.to_vec(),
        _ => original_background.to_vec(),
    }
}

pub fn load_background_data(image_data: &[u8]) -> Vec<u32> {
    let background = image::load_from_memory(image_data).expect("Failed to load image");
    let (width, height) = background.dimensions();
    let mut buffer = vec![0; (width * height) as usize];
    for (x, y, pixel) in background.pixels() {
        let Rgba([r, g, b, a]) = pixel;
        buffer[(y * width + x) as usize] =
            (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_draw_raindrop() {
        let width = 10;
        let height = 10;
        let mut buffer = vec![0xFFFFFFFF; (width * height) as usize];

        // Initialize Raindrop with current Instant for `last_update` and `start_time`
        let raindrop = Raindrop {
            x: 3,
            y: 3,
            last_update: Instant::now(),
            start_time: Instant::now(),
        };

        draw_raindrop(&mut buffer, width, height, &raindrop);

        // Check if the raindrop was drawn at the correct position
        for y in 3..(3 + DROP_SIZE) {
            for x in 3..(3 + DROP_SIZE) {
                assert_eq!(buffer[(y * width + x) as usize], 0xFFFFFFFF);
            }
        }
    }

    #[test]
    fn test_draw_square() {
        let width = 10;
        let height = 10;
        let mut buffer = vec![0xFFFFFFFF; (width * height) as usize];

        // Draw a 2x2 red square at (3, 3)
        draw_square(&mut buffer, width, height, 3, 3, 2);

        // Check if the square was drawn at the correct position
        for y in 3..5 {
            for x in 3..5 {
                assert_eq!(buffer[(y * width + x) as usize], 0xFFFF0000);
            }
        }
    }

    #[test]
    fn test_is_collision() {
        // Define two rectangles
        let rect1 = (0, 0, 2, 2);
        let rect2 = (1, 1, 2, 2);

        // Check if they collide
        assert!(is_collision(rect1, rect2));

        // Define a non-colliding rectangle
        let rect3 = (5, 5, 2, 2);
        assert!(!is_collision(rect1, rect3));
    }
}
