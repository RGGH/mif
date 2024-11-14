use rodio::{Decoder, OutputStream, Sink};
use std::{io::Cursor, sync::Arc, thread};

pub fn play_embedded_sound() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Arc::new(Sink::try_new(&stream_handle)?);

    // Clone the sink so we can move it into the thread
    let sink_clone = Arc::clone(&sink);
    
    thread::spawn(move || {
        let audio_data = include_bytes!("../assets/purring.mp3");
        let cursor = Cursor::new(audio_data);
        
        if let Ok(source) = Decoder::new(cursor) {
            sink_clone.append(source);
            sink_clone.sleep_until_end();
        } else {
            eprintln!("Failed to decode the audio data");
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_play_embedded_sound_in_thread() {
        // Attempt to initialize and play sound in a separate thread
        let play_result = play_embedded_sound();

        // Check that initializing playback did not return an error
        assert!(play_result.is_ok(), "Failed to initialize sound playback");

        // Wait for a short duration to allow any errors in playback to occur
        thread::sleep(Duration::from_millis(100));
    }
}

