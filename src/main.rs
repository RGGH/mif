use image::GenericImageView;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::Instant;
use image_utils::convert_to_mono;

mod image_utils;

const SQUARE_SIZE: u32 = 50;
const SQUARE_SIZE_STEP: u32 = 5;
const DROP_SIZE: u32 = 5;
const NUM_DROPS: usize = 6;
const DROP_DELAY: f32 = 0.5; // Delay in seconds for staggered start of each drop
const DROP_SPEED: f32 = 0.001; // Time-based speed (larger values make drops fall slower)

struct Raindrop {
    x: u32,
    y: u32,
    start_time: Instant,  // Time when the drop starts falling
    last_update: Instant, // Last time the drop's position was updated
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
                buffer[(py * width + px) as usize] = 0xFFFFFFFF // white
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
        "Cat ZZZ",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    let mut square_x = width / 2 - SQUARE_SIZE / 2;
    let mut square_y = height / 2 - SQUARE_SIZE / 2;
    let mut current_square_size = SQUARE_SIZE;

    // Initialize raindrops: they start falling from the top with staggered start times
    let mut raindrops: Vec<Raindrop> = (0..NUM_DROPS)
        .map(|i| Raindrop {
            x: rand::thread_rng().gen_range(0..width - DROP_SIZE),
            y: 0,
            start_time: Instant::now() + std::time::Duration::from_secs_f32(i as f32 * DROP_DELAY), // Staggered start times
            last_update: Instant::now(), // Set initial update time
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
            // Only reset the size, not the position
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

        // Update raindrop positions and draw them with staggered start
        for drop in raindrops.iter_mut() {
            if drop.start_time.elapsed().as_secs_f32() > 0.0 {
                // Only move the raindrop if its start time has passed
                if drop.y < height - DROP_SIZE
                    && drop.last_update.elapsed().as_secs_f32() > DROP_SPEED
                {
                    // Update the position of the raindrop at a slower rate (controlled by DROP_SPEED)
                    drop.y += 1;
                    drop.last_update = Instant::now(); // Update the last update time
                }
            }
            // Draw the raindrop
            draw_raindrop(&mut buffer, width, height, drop);
        }

        // Reset raindrops that have stuck at the bottom to the top for the next cycle
        for drop in raindrops.iter_mut() {
            if drop.y == height - DROP_SIZE {
                drop.y = 0; // Reset to top
                drop.x = rand::thread_rng().gen_range(0..width - DROP_SIZE); // Randomize position
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
