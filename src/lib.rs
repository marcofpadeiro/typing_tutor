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

    for _ in 0..num_words_to_show {
        queue.push(get_word());
    }

    match game_mode {
        GameMode::Words(num_to_finish) => {
            let start = Instant::now();
            while words_completed < num_to_finish {
                if run_word_cycle(out, &mut queue, None, start)? {
                    words_completed += 1;
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
                if run_word_cycle(out, &mut queue, Some(limit), start)? {
                    words_completed += 1;
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
        accuracy: 0.0,
    })
}

fn run_word_cycle(
    out: &mut Stdout,
    queue: &mut Vec<&str>,
    time_limit: Option<Duration>,
    start_time: Instant,
) -> Result<bool, Box<dyn std::error::Error>> {
    render_line(out, queue)?;

    Ok(process_word_input(
        out,
        queue.iter().next().unwrap(),
        time_limit,
        start_time,
    )?)
}
