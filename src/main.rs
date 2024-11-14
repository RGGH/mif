use image::GenericImageView;
use image::Rgba;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::Instant;

use image_utils::{convert_to_mono, draw_raindrop, draw_square, is_collision};
use sound_utils::play_embedded_sound;

mod image_utils;
mod sound_utils;

const SQUARE_SIZE: u32 = 40;
const SQUARE_SIZE_STEP: u32 = 5;
const DROP_SIZE: u32 = 5;
const NUM_DROPS: usize = 6;
const DROP_DELAY: f32 = 0.5; // Delay in seconds for staggered start of each drop
const DROP_SPEED: f32 = 0.002; // Time-based speed (larger values make drops fall slower)

struct Raindrop {
    x: u32,
    y: u32,
    start_time: Instant,  // Time when the drop starts falling
    last_update: Instant, // Last time the drop's position was updated
}

// Enum for different background types
enum Background<'a> {
    Mono(&'a [u32]),    // Use slice instead of Vec for background data
    Original(&'a [u32]), // Use slice instead of Vec for background data
    Mouse1(&'a [u32]),   // Use slice for mouse1 background
    Mouse2(&'a [u32]),   // Use slice for mouse2 background
    
}

fn main() {
    let _ = play_embedded_sound();
    let mut score = 0;
    let mut _current_background = Background::Mono(&[]); // Start with mono background

    // Load both the mono and original backgrounds
    let image_data = include_bytes!("../assets/background.png");
    let original_background =
        image::load_from_memory(image_data).expect("Failed to load original image");
    let mono_background = convert_to_mono(image_data);

    // Load mouse1 background (e.g., when 1st drop lands on cat)
    let mouse1_image_data = include_bytes!("../assets/background_mouse_1.png");
    let mouse1_background = image::load_from_memory(mouse1_image_data).expect("Failed to load mouse1 image");
    
    // Convert mouse1 background to u32 pixel values
    let mouse1_background = {
        let (width, height) = mouse1_background.dimensions();
        let mut buffer = vec![0; (width * height) as usize];
        for (x, y, pixel) in mouse1_background.pixels() {
            let Rgba([r, g, b, a]) = pixel;
            buffer[(y * width + x) as usize] =
                (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
        }
        buffer
    };
    // Load mouse2 background (e.g., when 2nd drop lands on cat)
    let mouse2_image_data = include_bytes!("../assets/background_mouse_2.png");
    let mouse2_background = image::load_from_memory(mouse2_image_data).expect("Failed to load mouse1 image");
    
    // Convert mouse2 background to u32 pixel values
    let mouse2_background = {
        let (width, height) = mouse2_background.dimensions();
        let mut buffer = vec![0; (width * height) as usize];
        for (x, y, pixel) in mouse2_background.pixels() {
            let Rgba([r, g, b, a]) = pixel;
            buffer[(y * width + x) as usize] =
                (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
        }
        buffer
    };

    let (width, height) = original_background.dimensions();
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

    let mut raindrops: Vec<Raindrop> = (0..NUM_DROPS)
        .map(|i| Raindrop {
            x: rand::thread_rng().gen_range(0..width - DROP_SIZE),
            y: 0,
            start_time: Instant::now() + std::time::Duration::from_secs_f32(i as f32 * DROP_DELAY),
            last_update: Instant::now(),
        })
        .collect();

    // Generate the color background
    let color_background = {
        let mut buffer = vec![0; (width * height) as usize];
        for (x, y, pixel) in original_background.pixels() {
            let Rgba([r, g, b, a]) = pixel;
            buffer[(y * width + x) as usize] =
                (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
        }
        buffer
    };

    // Store backgrounds in enum variants
    let mono_background_ref = Background::Mono(&mono_background);
    let original_background_ref = Background::Original(&color_background);
    let mouse1_background_ref = Background::Mouse1(&mouse1_background);
    let mouse2_background_ref = Background::Mouse2(&mouse2_background);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Directly use the background based on score, avoiding cloning
        let mut buffer = match score {
            s if s < 10 => match mono_background_ref {
                Background::Mono(data) => data.to_vec(),
                _ => vec![], // Default if something unexpected happens
            },
            s if s >= 10 && s < 15 => match original_background_ref {
                Background::Original(data) => data.to_vec(),
                _ => vec![], // Default if something unexpected happens
            },
            s if s >= 15 && s < 20 => match mouse1_background_ref {
                Background::Mouse1(data) => data.to_vec(),
                _ => vec![], // Default if something unexpected happens
            },
            s if s >= 20 && s < 250 => match mouse2_background_ref {
                Background::Mouse2(data) => data.to_vec(),
                _ => vec![], // Default if something unexpected happens
            },
            _ => match mouse1_background_ref {
                Background::Mouse1(data) => data.to_vec(),
                _ => vec![], // Default if something unexpected happens
            },
        };

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
            let old_size = current_square_size;
            current_square_size = SQUARE_SIZE + SQUARE_SIZE_STEP;
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

        // Draw square on the background
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
                if drop.y < height - DROP_SIZE
                    && drop.last_update.elapsed().as_secs_f32() > DROP_SPEED
                {
                    drop.y += 1;
                    drop.last_update = Instant::now();
                }
            }
            let drop_rect = (drop.x, drop.y, DROP_SIZE, DROP_SIZE);
            let square_rect = (square_x, square_y, current_square_size, current_square_size);
            if is_collision(drop_rect, square_rect) {
                score += 1;
                drop.y = 0;
                drop.x = rand::thread_rng().gen_range(0..width - DROP_SIZE);
            }
            // Draw raindrop on the background
            draw_raindrop(&mut buffer, width, height, drop);
        }

        for drop in raindrops.iter_mut() {
            if drop.y == height - DROP_SIZE {
                drop.y = 0;
                drop.x = rand::thread_rng().gen_range(0..width - DROP_SIZE);
            }
        }

        print!("\x1B[2J\x1B[1;1H");
        println!("Square Position - x: {}, y: {}", square_x, square_y);
        println!("Score: {}", score);

        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("Failed to update window buffer");
    }
}

