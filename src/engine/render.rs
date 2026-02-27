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
    queue: &Vec<String>
) -> Result<(), std::io::Error> {
    let current = queue.iter().next().unwrap();
    execute!(
        out,
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveToColumn(0),
        SetForegroundColor(Color::White),
        Print(PROMPT),
        Print(current),
    )?;

    for word in queue.iter().skip(1) {
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
