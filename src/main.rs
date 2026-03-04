use std::io::stdout;

use clack::{RenderMode, run};
use clap::Parser;
use crossterm::terminal;

use clack::cli::Args;
use clack::dictionary::{WordProvider, load_dictionary};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut out = stdout();
    terminal::enable_raw_mode()?;

    let result = run(
        &mut out,
        &setup_provider(&args),
        &args,
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

pub fn setup_provider(args: &Args) -> WordProvider {
    if let Some(ref keys) = args.practice {
        WordProvider::Practice(keys.chars().collect())
    } else {
        let words = load_dictionary(&args.dictionary, args.min_word_size, &args.filter);
        if words.is_empty() {
            eprintln!("error: no words matched your filter settings or dictionary was empty");
            std::process::exit(1);
        }
        WordProvider::Dictionary(words)
    }
}
