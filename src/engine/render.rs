use std::io::Write;

use crate::Stdout;
use crossterm::{
    cursor, execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};

const PROMPT: &str = "Type: ";

pub fn render_line(
    out: &mut Stdout,
    current: &str,
    upcoming: &[&str],
) -> Result<(), std::io::Error> {
    execute!(
        out,
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveToColumn(0),
        SetForegroundColor(Color::White),
        Print(PROMPT),
        Print(current),
    )?;

    for word in upcoming {
        execute!(
            out,
            SetForegroundColor(Color::DarkGrey),
            Print(" "),
            Print(word)
        )?;
    }

    execute!(out, cursor::MoveToColumn(PROMPT.len() as u16))?;
    out.flush()
}

pub fn get_upcoming_words<'a>(
    current_idx: usize,
    words: &'a [&'a str],
    num_words_to_show: usize,
) -> &'a [&'a str] {
    let next_idx = current_idx + 1;

    if next_idx >= words.len() {
        return &[];
    }

    let end_idx = (next_idx + num_words_to_show).min(words.len());
    &words[next_idx..end_idx]
}
