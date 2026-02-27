use std::{io::stdout, process::exit};

use clap::Parser;
use crossterm::terminal;
use clack::{RenderMode, run};

use crate::{cli::Args, dictionary::load_dictionary};

mod cli;
mod dictionary;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut out = stdout();
    terminal::enable_raw_mode()?;

    let dictionary = load_dictionary(&args.dictionary, args.filter);
    if dictionary.is_empty() {
        terminal::disable_raw_mode()?;
        println!("empty dictionary");
        exit(0);
    }

    let result = run(
        &mut out,
        &dictionary,
        args.word_preview as usize,
        args.mode,
        args.quantity,
        args.auto_advance,
        RenderMode::Upcoming(args.word_preview as usize),
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
