use image::{GenericImageView, Rgba};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::Instant;

use image_utils::{convert_to_mono, draw_raindrop, draw_square, is_collision};

mod image_utils;

const SQUARE_SIZE: u32 = 40;
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

fn main() {
    let mut score = 0;

    let image_data = include_bytes!("../assets/background.png");

    let original_background = load_background_data(image_data);

    let mouse1_image_data = include_bytes!("../assets/background_mouse_1.png");
    let mouse1_background = load_background_data(mouse1_image_data);

    let mouse2_image_data = include_bytes!("../assets/background_mouse_2.png");
    let mouse2_background = load_background_data(mouse2_image_data);
    
    let mono_background = convert_to_mono(image_data);

    let original_backgroundxs = image::load_from_memory(image_data).expect("Failed to load original image");
    let (width, height) = original_backgroundxs.dimensions();
    let width = width as u32;
    let height = height as u32;

    let mut window = Window::new("Cat ZZZ", width as usize, height as usize, WindowOptions::default())
        .expect("Unable to create window");

    let cat_width = 200;
    let cat_x = (width - cat_width) / 2;
    let cat_y = (height * 4) / 5; // Bottom fifth of the screen

    let mut cursor_x = width / 2 - SQUARE_SIZE / 2;
    let mut cursor_y = height / 2 - SQUARE_SIZE / 2;

    let mut raindrops: Vec<Raindrop> = (0..NUM_DROPS)
        .map(|i| Raindrop {
            x: rand::thread_rng().gen_range(0..width - DROP_SIZE),
            y: 0,
            start_time: Instant::now() + std::time::Duration::from_secs_f32(i as f32 * DROP_DELAY),
            last_update: Instant::now(),
        })
        .collect();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer = get_background_for_score(score, &mono_background, &mouse1_background, &mouse2_background,&original_background);

        // Cursor movement
        if window.is_key_down(Key::Up) && cursor_y > 0 {
            cursor_y -= 1;
        }
        if window.is_key_down(Key::Down) && cursor_y + SQUARE_SIZE < height {
            cursor_y += 1;
        }
        if window.is_key_down(Key::Left) && cursor_x > 0 {
            cursor_x -= 1;
        }
        if window.is_key_down(Key::Right) && cursor_x + SQUARE_SIZE < width {
            cursor_x += 1;
        }

        // Invisible cat boundary for collision detection only
        let cat_rect = (cat_x, cat_y, cat_width, SQUARE_SIZE);

        // Draw cursor
        draw_square(&mut buffer, width, height, cursor_x, cursor_y, SQUARE_SIZE);

        // Update raindrops
        for drop in raindrops.iter_mut() {
            if drop.start_time.elapsed().as_secs_f32() > 0.0 {
                if drop.y < height - DROP_SIZE && drop.last_update.elapsed().as_secs_f32() > DROP_SPEED {
                    drop.y += 1;
                    drop.last_update = Instant::now();
                } else if drop.y >= height - DROP_SIZE {
                    // Reset raindrop when it reaches the bottom
                    drop.y = 0;
                    drop.x = rand::thread_rng().gen_range(0..width - DROP_SIZE);
                }
            }
            let drop_rect = (drop.x, drop.y, DROP_SIZE, DROP_SIZE);
            let cursor_rect = (cursor_x, cursor_y, SQUARE_SIZE, SQUARE_SIZE);

            // Deduct points if raindrop hits the cat
            if is_collision(drop_rect, cat_rect) {
                score = score.saturating_sub(5);
                drop.y = 0;
                drop.x = rand::thread_rng().gen_range(0..width - DROP_SIZE);
            }

            // Increase score if raindrop hits the cursor
            if is_collision(drop_rect, cursor_rect) {
                score += 1;
                drop.y = 0;
                drop.x = rand::thread_rng().gen_range(0..width - DROP_SIZE);
            }

            draw_raindrop(&mut buffer, width, height, drop);
        }

        window.update_with_buffer(&buffer, width as usize, height as usize)
              .expect("Failed to update window buffer");

        print!("\x1B[2J\x1B[1;1H");
        println!("Score: {}", score);
    }
}

fn get_background_for_score<'a>(
    score: i32,
    mono_background: &'a [u32],
    mouse1_background: &'a [u32],
    mouse2_background: &'a [u32],
    original_background: &'a [u32],
) -> Vec<u32> {
    match score {
        s if s < -30 => mono_background.to_vec(),
        s if s <= -10 && s >= -20 => mouse1_background.to_vec(),
        s if s <= -21 && s >= -30 => mouse2_background.to_vec(),
        _ => original_background.to_vec(),
    }
}

fn load_background_data(image_data: &[u8]) -> Vec<u32> {
    let background = image::load_from_memory(image_data).expect("Failed to load image");
    let (width, height) = background.dimensions();
    let mut buffer = vec![0; (width * height) as usize];
    for (x, y, pixel) in background.pixels() {
        let Rgba([r, g, b, a]) = pixel;
        buffer[(y * width + x) as usize] = (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
    }
    buffer
}

