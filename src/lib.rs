use crate::engine::input::process_word_input;
use crate::engine::render::render_line;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::io::Stdout;
use std::time::Duration;
use std::time::Instant;

mod dictionary;
mod engine;

pub enum GameMode {
    Timer(u64),
    Words(usize),
}

pub enum RenderMode {
    All,
    Upcoming(usize),
}

pub struct WordResult {
    correct: usize,
    incorrect: usize,
}
impl WordResult {
    pub fn new(correct: usize, incorrect: usize) -> Self {
        WordResult { correct, incorrect }
    }
}

#[derive(Clone, Copy)]
pub struct GameResult {
    pub time_took: Duration,
    pub words_completed: usize,
    pub accuracy: f32,
}
impl GameResult {
    pub fn get_wpm(self) -> usize {
        let secs = self.time_took.as_secs() as usize;
        if secs == 0 {
            return 0;
        }

        (self.words_completed * 60) / secs
    }
}

pub fn run(
    out: &mut Stdout,
    words: Vec<&'static str>,
    num_words_to_show: usize,
    game_mode: GameMode,
    _render_mode: RenderMode,
) -> Result<GameResult, Box<dyn std::error::Error>> {
    let mut rng = thread_rng();

    let time_took: Duration;
    let mut words_completed: usize = 0;
    let mut queue: Vec<&str> = vec![];
    let mut get_word = || words.choose(&mut rng).copied().unwrap_or("error");
    let mut correct_chars = 0;
    let mut incorrect_chars = 0;

    for _ in 0..num_words_to_show {
        queue.push(get_word());
    }

    match game_mode {
        GameMode::Words(num_to_finish) => {
            let start = Instant::now();
            while words_completed < num_to_finish {
                let word_cycle_result = run_word_cycle(out, &mut queue, None, start)?;
                if let Some(word_result) = word_cycle_result {
                    words_completed += 1;
                    correct_chars += word_result.correct;
                    incorrect_chars += word_result.incorrect;
                }
                queue.remove(0);
                if words_completed < num_to_finish - 2 {
                    queue.push(get_word());
                }
            }
            time_took = start.elapsed();
        }
        GameMode::Timer(seconds) => {
            let start = Instant::now();
            let limit = Duration::from_secs(seconds);
            while start.elapsed() < limit {
                let word_cycle_result = run_word_cycle(out, &mut queue, Some(limit), start)?;
                if let Some(word_result) = word_cycle_result {
                    words_completed += 1;
                    correct_chars += word_result.correct;
                    incorrect_chars += word_result.incorrect;
                }
                queue.remove(0);
                queue.push(get_word());
            }
            time_took = start.elapsed();
        }
    }

    let total_presses = (correct_chars + incorrect_chars) as f32;

    let accuracy = if total_presses > 0.0 {
        (correct_chars as f32 * 100.0) / total_presses
    } else {
        0.0
    };

    Ok(GameResult {
        time_took,
        words_completed,
        accuracy,
    })
}

fn run_word_cycle(
    out: &mut Stdout,
    queue: &mut Vec<&str>,
    time_limit: Option<Duration>,
    start_time: Instant,
) -> Result<Option<WordResult>, Box<dyn std::error::Error>> {
    render_line(out, queue)?;

    Ok(process_word_input(
        out,
        queue.iter().next().unwrap(),
        time_limit,
        start_time,
    )?)
}
