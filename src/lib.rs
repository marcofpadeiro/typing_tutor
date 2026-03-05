use crate::cli::Args;
use crate::dictionary::WordProvider;
use crate::engine::process_word_input;
use crate::engine::render::render_line;
use clap::ValueEnum;
use rand::thread_rng;
use std::collections::VecDeque;
use std::io::Stdout;
use std::time::Duration;
use std::time::Instant;
use std::usize;

pub mod cli;
pub mod dictionary;
mod engine;

#[derive(ValueEnum, Clone, Debug)]
pub enum GameMode {
    Timer,
    Words,
}

pub enum RenderMode {
    All,
    Upcoming(usize),
}

pub struct WordResult {
    correct_chars: usize,
    incorrect_chars: usize,
}
impl WordResult {
    pub fn new(correct_chars: usize, incorrect_chars: usize) -> Self {
        WordResult {
            correct_chars,
            incorrect_chars,
        }
    }
}

#[derive(Clone, Copy)]
pub struct GameResult {
    pub time_took: Duration,
    pub words_completed: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
}
impl GameResult {
    pub fn new(
        time_took: Duration,
        words_completed: usize,
        correct_chars: usize,
        incorrect_chars: usize,
    ) -> Self {
        GameResult {
            time_took,
            words_completed,
            correct_chars,
            incorrect_chars,
        }
    }
    pub fn wpm(self) -> usize {
        let secs = self.time_took.as_secs() as usize;
        if secs == 0 {
            return 0;
        }

        let words = self.correct_chars / 5;

        (words * 60) / secs
    }
    pub fn accuracy(self) -> f32 {
        let total_presses = (self.correct_chars + self.incorrect_chars) as f32;

        if total_presses > 0.0 {
            (self.correct_chars as f32 * 100.0) / total_presses
        } else {
            0.0
        }
    }
}

pub fn run(
    out: &mut Stdout,
    provider: &WordProvider,
    settings: &Args,
    _render_mode: RenderMode,
) -> Result<GameResult, Box<dyn std::error::Error>> {
    let mut rng = thread_rng();

    let mut words_completed: usize = 0;
    let mut correct_chars = 0;
    let mut incorrect_chars = 0;

    let mut queue: VecDeque<String> = VecDeque::new();

    for _ in 0..=settings.word_preview {
        queue.push_back(provider.get_word(&mut rng));
    }

    let start = Instant::now();
    let time_limit = match settings.mode {
        GameMode::Timer => Some(Duration::from_secs(settings.quantity as u64)),
        GameMode::Words => None,
    };

    while match settings.mode {
        GameMode::Words => words_completed < settings.quantity,
        GameMode::Timer => start.elapsed() < time_limit.unwrap_or_default(),
    } {
        render_line(out, &queue)?;
        let current_word = queue.front().map(|s| s.as_str()).unwrap_or("thisshouldnothappenlmao");
        let word_cycle_result = process_word_input(
            out,
            current_word,
            settings.auto_advance,
            time_limit,
            start,
        )?;

        if let Some(word_result) = word_cycle_result {
            words_completed += 1;
            correct_chars += word_result.correct_chars;
            incorrect_chars += word_result.incorrect_chars;

            queue.pop_front();

            let should_refill = match settings.mode {
                GameMode::Words => words_completed + queue.len() < settings.quantity,
                GameMode::Timer => true,
            };

            if should_refill {
                queue.push_back(provider.get_word(&mut rng));
            }
        }
    }

    Ok(GameResult {
        time_took: start.elapsed(),
        words_completed,
        correct_chars,
        incorrect_chars,
    })
}
