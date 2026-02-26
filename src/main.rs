use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use std::io::{Write, stdout};

const WORDS: [&str; 8] = [
    "tent",
    "station",
    "annotation",
    "eos",
    "stern",
    "ratio",
    "sat",
    "sitio",
];
const NUM_WORDS_TO_SHOW: usize = 3;
const NEEDS_BACKSPACE: bool = true;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;

    for i in 0..WORDS.len() {
        let main_word = WORDS[i];

        let end_idx = (i + NUM_WORDS_TO_SHOW).min(WORDS.len());
        let next_words = &WORDS[i + 1..end_idx];

        execute!(
            out,
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            SetForegroundColor(Color::White),
            Print("Type: "),
            Print(main_word),
        )?;

        for word in next_words {
            execute!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(" "),
                Print(word)
            )?;
        }

        execute!(out, cursor::MoveToColumn(6))?;
        out.flush()?;

        let mut index = 0;
        let chars: Vec<char> = main_word.chars().collect();
        let word_size = chars.len();
        let mut word_written = String::new();

        'word_loop: loop {
            if word_written == main_word && !NEEDS_BACKSPACE {
                break 'word_loop;
            }

            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Release {
                    continue;
                }

                match key_event.code {
                    KeyCode::Char(c) => {
                        if c == ' ' {
                            if word_written == main_word {
                                break 'word_loop;
                            }
                            execute!(out, SetForegroundColor(Color::Red), Print(c))?;
                            word_written.push(c);
                            index += 1;
                        } else if index < word_size && c == chars[index] {
                            execute!(out, SetForegroundColor(Color::Green), Print(c))?;
                            word_written.push(c);
                            index += 1;
                        } else {
                            execute!(out, SetForegroundColor(Color::Red), Print(c))?;
                            word_written.push(c);
                            index += 1;
                        }
                    }
                    KeyCode::Backspace => {
                        if index > 0 {
                            index -= 1;
                            word_written.pop();

                            let restore_char = if index < word_size { chars[index] } else { ' ' };

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
                        return Ok(());
                    }
                    _ => (),
                }
                out.flush()?;
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
