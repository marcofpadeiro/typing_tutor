use crate::dictionary::NUM_UPCOMING_WORDS_TO_SHOW;
use crate::dictionary::WORDS;
use std::io::stdout;

use crossterm::terminal;
use typing_tutor::{GameMode, RenderMode, run};

mod dictionary;

const TIMER: usize = 30;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;

    if let Err(e) = run(
        &mut out,
        WORDS.to_vec(),
        NUM_UPCOMING_WORDS_TO_SHOW,
        GameMode::Timer(TIMER),
        RenderMode::Upcoming(NUM_UPCOMING_WORDS_TO_SHOW),
    ) {
        terminal::disable_raw_mode()?;
        return Err(e);
    }

    terminal::disable_raw_mode()?;
    println!("\r\nDone!");
    Ok(())
}
