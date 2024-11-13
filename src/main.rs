use image::{GenericImageView, Rgba};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const SQUARE_SIZE: u32 = 50;
const SQUARE_SIZE_STEP: u32 = 5;
const DROP_SIZE: u32 = 5;
const NUM_DROPS: usize = 6;

struct Raindrop {
    x: u32,
    y: u32,
}

fn convert_to_mono(image_data: &[u8]) -> Vec<u32> {
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

fn draw_square(buffer: &mut Vec<u32>, width: u32, height: u32, x: u32, y: u32, size: u32) {
    for dy in 0..size {
        for dx in 0..size {
            let px = x + dx;
            let py = y + dy;
            if px < width && py < height {
                buffer[(py * width + px) as usize] = 0xFFFF0000;
            }
        }
    }
}

fn draw_raindrop(buffer: &mut Vec<u32>, width: u32, height: u32, drop: &Raindrop) {
    for dy in 0..DROP_SIZE {
        for dx in 0..DROP_SIZE {
            let px = drop.x + dx;
            let py = drop.y + dy;
            if px < width && py < height {
                buffer[(py * width + px) as usize] = 0xFF0000FF; // Blue color
            }
        }
    }
}

fn main() {
    let image_data = include_bytes!("background.png");
    let background_buffer = convert_to_mono(image_data);

    let (width, height) = image::load_from_memory(image_data)
        .expect("Failed to load image")
        .dimensions();
    let width = width as u32;
    let height = height as u32;

    let mut window = Window::new(
        "Game Window",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    let mut square_x = width / 2 - SQUARE_SIZE / 2;
    let mut square_y = height / 2 - SQUARE_SIZE / 2;
    let mut current_square_size = SQUARE_SIZE;

    // each raindrop's initial position randomized
    let mut raindrops: Vec<Raindrop> = (0..NUM_DROPS)
        .map(|_| Raindrop {
            x: rand::thread_rng().gen_range(0..width - DROP_SIZE),
            y: 0,
        })
        .collect();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer = background_buffer.clone();

        // Handle input to move the square
        if window.is_key_down(Key::Up) && square_y > 0 {
            square_y -= 1;
        }
        if window.is_key_down(Key::Down) && square_y + current_square_size < height {
            square_y += 1;
        }
        if window.is_key_down(Key::Left) && square_x > 0 {
            square_x -= 1;
        }
        if window.is_key_down(Key::Right) && square_x + current_square_size < width {
            square_x += 1;
        }

        // Handle space bar to grow the square
        if window.is_key_down(Key::Space) {
            // Save the current square size
            let old_size = current_square_size;
            current_square_size = SQUARE_SIZE + SQUARE_SIZE_STEP;

            // Adjust position to keep the square growing around its center
            square_x -= (current_square_size - old_size) / 2;
            square_y -= (current_square_size - old_size) / 2;
        } else {
            current_square_size = SQUARE_SIZE;
        }

        // Ensure square stays within bounds when it grows
        if square_x + current_square_size > width {
            square_x = width - current_square_size;
        }
        if square_y + current_square_size > height {
            square_y = height - current_square_size;
        }

        // Draw the square on the background buffer
        draw_square(
            &mut buffer,
            width,
            height,
            square_x,
            square_y,
            current_square_size,
        );

        // Update raindrop positions and draw them
        for drop in raindrops.iter_mut() {
            draw_raindrop(&mut buffer, width, height, drop);

            // Move the raindrop downwards
            // Move the raindrop downwards
            drop.y += 1;

            // Reset raindrop to the top if it goes beyond the bottom
            if drop.y >= height {
                drop.y = 0;
                drop.x = rand::thread_rng().gen_range(0..width - DROP_SIZE);
            }
        }

        // Clear the console and print the square coordinates
        print!("\x1B[2J\x1B[1;1H"); // Clears the terminal screen
        println!("Square Position - x: {}, y: {}", square_x, square_y);

        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("Failed to update window buffer");
    }
}
