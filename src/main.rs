use crate::dictionary::QUEUE_SIZE;
use crate::dictionary::WORDS;
use std::io::stdout;

use crossterm::terminal;
use typing_tutor::{GameMode, RenderMode, run};

mod dictionary;

const TIMER: u64 = 30;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;

    let result = run(
        &mut out,
        WORDS.to_vec(),
        QUEUE_SIZE,
        GameMode::Timer(TIMER),
        RenderMode::Upcoming(QUEUE_SIZE),
    );

    terminal::disable_raw_mode()?;

    match result {
        Ok(game_result) => {
            println!("\n\n--- Results ---");
            println!("Time: {:.1?}", game_result.time_took);
            println!("Words completed: {}", game_result.words_completed);
            println!("Accuracy: {:.1}%", game_result.accuracy);
            println!("WPM: {}", game_result.get_wpm());
        }
        Err(e) => {
            eprintln!("\nError occurred: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
