use std::io::{Stdout, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};

pub fn process_word_input(
    out: &mut Stdout,
    target: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut written = String::new();
    let target_chars: Vec<char> = target.chars().collect();

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char(c) => {
                    let index = written.len();

                    if c == ' ' && written == target {
                        break;
                    }

                    let color = if index < target_chars.len() && c == target_chars[index] {
                        Color::Green
                    } else {
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
    Ok(())
}
