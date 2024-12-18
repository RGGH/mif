use image::GenericImageView;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use rusttype::{Font, Scale};
use std::time::{Duration, Instant};

use image_utils::{convert_to_mono, draw_raindrop, draw_square, is_collision};
use image_utils::{get_background_for_score, load_background_data};
use image_utils::draw_text;

mod image_utils;

const DROP_SIZE: u32 = 6;
const NUM_DROPS: usize = 8;
const DROP_DELAY: f32 = 0.5; // Delay in seconds for staggered start of each drop
const DROP_SPEED: f32 = 200.0; // Speed in pixels per second
const WINNING_SCORE: i32 = 30; // Score required to win
const TARGET_FPS: f32 = 60.0;

struct Raindrop {
    x: u32,
    y: u32,
    start_time: Instant,  // Time when the drop starts falling
    last_update: Instant, // Last time the drop's position was updated
}



fn main() {
    let mut score: i32 = 0;
    let mut square_size: u32 = 30;

    let image_data = include_bytes!("../assets/background.png");
    let original_background = load_background_data(image_data);

    let mouse1_image_data = include_bytes!("../assets/background_mouse_1.png");
    let mouse1_background = load_background_data(mouse1_image_data);

    let mouse2_image_data = include_bytes!("../assets/background_mouse_2.png");
    let mouse2_background = load_background_data(mouse2_image_data);

    let winner_image_data = include_bytes!("../assets/winner.png");
    let winner_background = load_background_data(winner_image_data);

    let mono_background = convert_to_mono(image_data);

    let original_backgroundxs =
        image::load_from_memory(image_data).expect("Failed to load original image");
    let (width, height) = original_backgroundxs.dimensions();
    let width = width as u32;
    let height = height as u32;

    let mut window = Window::new(
        "Cat ZZZ",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    let cat_width = 200;
    let cat_x = (width - cat_width) / 2;
    let cat_y = (height * 4) / 5; // Bottom fifth of the screen

    let mut cursor_x = width / 2 - square_size / 2;
    let mut cursor_y = height / 2 - square_size / 2;

    let mut raindrops: Vec<Raindrop> = (0..NUM_DROPS)
        .map(|i| Raindrop {
            x: rand::thread_rng().gen_range(0..width - DROP_SIZE),
            y: 0,
            start_time: Instant::now() + Duration::from_secs_f32(i as f32 * DROP_DELAY),
            last_update: Instant::now(),
        })
        .collect();

    let frame_duration = Duration::from_secs_f32(1.0 / TARGET_FPS);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();

        if score >= WINNING_SCORE {
            println!("You win! Final Score: {}", score);

            let start_time = Instant::now();
            let buffer = winner_background.clone();
            while start_time.elapsed().as_secs() < 4 {
                // Display winner background
                window
                    .update_with_buffer(&buffer, width as usize, height as usize)
                    .expect("Failed to update window buffer");
            }

            break;
        }

        let mut buffer = get_background_for_score(
            score.try_into().unwrap(),
            &mono_background,
            &mouse1_background,
            &mouse2_background,
            &original_background,
            &winner_background,
        );

        // Cursor movement
        if window.is_key_down(Key::Up) && cursor_y > 0 {
            cursor_y = cursor_y.saturating_sub(5);
        }
        if window.is_key_down(Key::Down) && cursor_y + square_size < height {
            cursor_y = cursor_y.saturating_add(5);
        }
        if window.is_key_down(Key::Left) && cursor_x > 0 {
            cursor_x = cursor_x.saturating_sub(5);
        }
        if window.is_key_down(Key::Right) && cursor_x + square_size < width {
            cursor_x = cursor_x.saturating_add(5);
        }

        let cat_rect = (cat_x, cat_y, cat_width, square_size);

        draw_square(&mut buffer, width, height, cursor_x, cursor_y, square_size);

        // Update raindrops
        for drop in raindrops.iter_mut() {
            if drop.start_time.elapsed().as_secs_f32() > 0.0 {
                let elapsed = drop.last_update.elapsed().as_secs_f32();
                let pixels_to_move = (elapsed * DROP_SPEED).round() as u32;

                if drop.y < height - DROP_SIZE {
                    drop.y += pixels_to_move;
                    drop.last_update = Instant::now();
                } else {
                    drop.y = 0;
                    drop.x = rand::thread_rng().gen_range(0..width - DROP_SIZE);
                }
            }

            let drop_rect = (drop.x, drop.y, DROP_SIZE, DROP_SIZE);
            let cursor_rect = (cursor_x, cursor_y, square_size, square_size);

            // Deduct points if raindrop hits the cat
            if is_collision(drop_rect, cat_rect) {
                score -= 5;
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

        // Render the score on the screen
        draw_text(&mut buffer, width, height, &format!("Score: {}", score), 10, 30);

        if score > 10 {
            square_size = 50;
        } else {
            square_size = 30;
        }

        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("Failed to update window buffer");

        // Cap FPS
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}

