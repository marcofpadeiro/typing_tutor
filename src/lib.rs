use crate::engine::process_word_input;
use crate::engine::render::render_line;
use clap::ValueEnum;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::io::Stdout;
use std::time::Duration;
use std::time::Instant;
use std::usize;

mod dictionary;
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

        (self.words_completed * 60) / secs
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
    words: &Vec<String>,
    word_preview: usize,
    mode: GameMode,
    quantity: u64,
    auto_advance: bool,
    _render_mode: RenderMode,
) -> Result<GameResult, Box<dyn std::error::Error>> {
    let mut rng = thread_rng();

    let time_took: Duration;
    let mut words_completed: usize = 0;
    let mut queue: Vec<String> = vec![];
    let mut get_word = || {
        words
            .choose(&mut rng)
            .cloned()
            .unwrap_or(String::from("error"))
            .to_string()
    };
    let mut correct_chars = 0;
    let mut incorrect_chars = 0;

    queue.push(get_word());
    for _ in 0..word_preview {
        queue.push(get_word());
    }

    match mode {
        GameMode::Words => {
            let start = Instant::now();
            while words_completed < quantity as usize {
                let word_cycle_result = run_word_cycle(out, &mut queue, auto_advance, None, start)?;
                if let Some(word_result) = word_cycle_result {
                    words_completed += 1;
                    correct_chars += word_result.correct_chars;
                    incorrect_chars += word_result.incorrect_chars;
                }
                queue.remove(0);
                if words_completed < quantity as usize - 1 {
                    queue.push(get_word());
                }
            }
            time_took = start.elapsed();
        }
        GameMode::Timer => {
            let start = Instant::now();
            let limit = Duration::from_secs(quantity);
            while start.elapsed() < limit {
                let word_cycle_result =
                    run_word_cycle(out, &mut queue, auto_advance, Some(limit), start)?;
                if let Some(word_result) = word_cycle_result {
                    words_completed += 1;
                    correct_chars += word_result.correct_chars;
                    incorrect_chars += word_result.incorrect_chars;
                }
                queue.remove(0);
                queue.push(get_word());
            }
            time_took = start.elapsed();
        }
    }

    Ok(GameResult {
        time_took,
        words_completed,
        correct_chars,
        incorrect_chars,
    })
}

fn run_word_cycle(
    out: &mut Stdout,
    queue: &mut Vec<String>,
    auto_advance: bool,
    time_limit: Option<Duration>,
    start_time: Instant,
) -> Result<Option<WordResult>, Box<dyn std::error::Error>> {
    render_line(out, queue)?;

    Ok(process_word_input(
        out,
        queue.iter().next().unwrap(),
        auto_advance,
        time_limit,
        start_time,
    )?)
}
