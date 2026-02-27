use std::{
    io::{Stdout, Write},
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{self, Color, Print, SetForegroundColor},
    terminal,
};

use crate::WordResult;

pub mod render;
pub fn process_word_input(
    out: &mut Stdout,
    target: &str,
    time_limit: Option<Duration>,
    start_time: Instant,
) -> Result<Option<WordResult>, Box<dyn std::error::Error>> {
    let mut written = String::new();
    let target_chars: Vec<char> = target.chars().collect();
    let mut incorrect: usize = 0;

    loop {
        if let Some(limit) = time_limit {
            if start_time.elapsed() >= limit {
                break;
            }
        }

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }
                match key.code {
                    KeyCode::Char(c) => {
                        let index = written.len();

                        if c == ' ' && written == target {
                            execute!(out, style::ResetColor)?;
                            return Ok(Some(WordResult::new(target_chars.len(), incorrect))); // word finished successfully
                        }

                        let color = if index < target_chars.len() && c == target_chars[index] {
                            Color::Green
                        } else {
                            incorrect += 1;
                            Color::Red
                        };

                        execute!(out, SetForegroundColor(color), Print(c))?;
                        written.push(c);
                    }
                    KeyCode::Backspace => {
                        if let Some(_last_char) = written.pop() {
                            let index = written.len();
                            let restore_char = target_chars.get(index).cloned().unwrap_or(' ');

                            execute!(
                                out,
                                cursor::MoveLeft(1),
                                SetForegroundColor(Color::White),
                                Print(restore_char),
                                cursor::MoveLeft(1)
                            )?;
                        }
                    }
                    KeyCode::Esc => {
                        terminal::disable_raw_mode()?;
                        std::process::exit(0);
                    }
                    _ => (),
                }
                out.flush()?;
            }
        }
    }

    execute!(out, style::ResetColor)?;
    Ok(None)
}
