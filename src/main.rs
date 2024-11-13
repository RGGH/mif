use image::{GenericImageView, Rgba};
use minifb::{Window, WindowOptions, Key};

const SQUARE_SIZE: u32 = 50; // Size of the square

// Function to convert image to grayscale (or background)
fn convert_to_mono(image_data: &[u8]) -> Vec<u32> {
    // Load the image from the included byte data
    let img = image::load_from_memory(image_data).expect("Failed to load image");

    let (width, height) = img.dimensions();
    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];

    // Iterate through the image pixels and convert each to grayscale
    for (x, y, pixel) in img.pixels() {
        // Extract the R, G, B components (ignoring the A component for grayscale)
        let Rgba([r, g, b, _]) = pixel;

        // Convert to grayscale using the luminance formula
        let grayscale = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;

        // Pack the grayscale value into a u32 (BGRA format)
        buffer[(y * width + x) as usize] = (grayscale as u32) | ((grayscale as u32) << 8) | ((grayscale as u32) << 16) | 0xFF000000;
    }

    buffer
}

// Function to draw the square on top of the background
fn draw_square(buffer: &mut Vec<u32>, width: u32, height: u32, x: u32, y: u32) {
    for dy in 0..SQUARE_SIZE {
        for dx in 0..SQUARE_SIZE {
            let px = x + dx;
            let py = y + dy;
            if px < width && py < height {
                // Draw the square in red (ARGB format)
                buffer[(py * width + px) as usize] = 0xFFFF0000; // Red color
            }
        }
    }
}

fn main() {
    // Embed the image as bytes into the binary
    let image_data = include_bytes!("background.png"); // The image must be in the same directory

    // Load the image as background
    let background_buffer = convert_to_mono(image_data);
    
    let (width, height) = image::load_from_memory(image_data)
        .expect("Failed to load image")
        .dimensions();
    let width = width as u32;
    let height = height as u32;

    // Create a window to display the game
    let mut window = Window::new(
        "Game Window",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    // Initial position of the square
    let mut square_x = width / 2 - SQUARE_SIZE / 2;
    let mut square_y = height / 2 - SQUARE_SIZE / 2;

    // Game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer = background_buffer.clone(); // Clone the background buffer

        // Handle input to move the square
        if window.is_key_down(Key::Up) {
            if square_y > 0 {
                square_y -= 1;
            }
        }
        if window.is_key_down(Key::Down) {
            if square_y + SQUARE_SIZE < height {
                square_y += 1;
            }
        }
        if window.is_key_down(Key::Left) {
            if square_x > 0 {
                square_x -= 1;
            }
        }
        if window.is_key_down(Key::Right) {
            if square_x + SQUARE_SIZE < width {
                square_x += 1;
            }
        }

        // Draw the square on the background buffer
        draw_square(&mut buffer, width, height, square_x, square_y);

        // Update the window with the new buffer
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("Failed to update window buffer");

        // Optionally, handle mouse or other events
    }
}

