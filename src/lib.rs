use crate::engine::input::process_word_input;
use crate::engine::render::get_upcoming_words;
use crate::engine::render::render_line;
use std::io::Stdout;

mod dictionary;
mod engine;

pub enum GameMode {
    Timer(usize),
    Words(usize),
}

pub enum RenderMode {
    All,
    Upcoming(usize),
}

pub fn run(
    out: &mut Stdout,
    words: Vec<&str>,
    num_words_to_show: usize,
    game_mode: GameMode,
    render_mode: RenderMode,
) -> Result<(), Box<dyn std::error::Error>> {
    for index in 0..words.len() {
        let main_word = words[index];
        let upcoming = get_upcoming_words(index, &words, num_words_to_show);

        render_line(out, main_word, upcoming)?;
        process_word_input(out, main_word)?;
    }
    Ok(())
}
