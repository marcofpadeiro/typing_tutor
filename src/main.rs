use std::io::stdout;

use crossterm::terminal;
use typing_tutor::{GameMode, RenderMode, run};

use crate::dictionary::{QUEUE_SIZE, load_dictionary};

mod dictionary;

const TIMER: u64 = 30;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;

    let dictionary = load_dictionary("medium");

    let result = run(
        &mut out,
        &dictionary,
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
            println!("Accuracy: {:.1}%", game_result.accuracy());
            println!("WPM: {}", game_result.wpm());
        }
        Err(e) => {
            eprintln!("\nError occurred: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
