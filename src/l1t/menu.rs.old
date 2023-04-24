use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::Stdout;

/// A thinkg
pub enum MenuType {
    /// Dialog box with a single message. Any key will close.
    Message(String),

    /// Dialog box that displays the message `String` and will
    /// show a list of the given option `Strings`. Returns the
    /// index of the selected option on enter.
    Selection(String, Vec<String>),

    /// Same as `MenuType::Selection` but only allows selecting
    /// `Yes` or `No`.
    YesNoSelection(String),

    /// Same as `MenuType::Selection` except the first argument
    /// is a `Vec<String>` representing a logo or large message
    /// to display.
    MainSelection(Vec<String>, Vec<String>),
}

/// A thing
pub struct Menu {}

impl Menu {
    pub fn draw(
        stdout: &mut Stdout,
        menu_type: MenuType,
        term_rows: u16,
        term_cols: u16,
    ) -> Option<usize> {
        match menu_type {
            MenuType::MainSelection(logo, options) => {
                if logo.len() == 0 || options.len() == 0 {
                    return None;
                }
                let mut current_selection: usize = 0;
                let start_row = (term_rows - logo.len() as u16 - options.len() as u16 - 5) / 2;
                let end_row = (term_rows + logo.len() as u16 + options.len() as u16 + 3) / 2;
                let start_col = (term_cols - logo[0].len().max(options[0].len()) as u16 - 4) / 2;
                let end_col = (term_cols + logo[0].len().max(options[0].len()) as u16 + 2) / 2;
                for r in start_row..(end_row + 1) {
                    for c in start_col..(end_col + 1) {
                        if r == start_row || r == end_row {
                            execute!(stdout, cursor::MoveTo(c, r), Print("─".bold()),).ok();
                        } else if c == start_col || c == end_col {
                            execute!(stdout, cursor::MoveTo(c, r), Print("│".bold()),).ok();
                        } else {
                            execute!(stdout, cursor::MoveTo(c, r), Print(" "),).ok();
                        }
                    }
                }
                execute!(
                    stdout,
                    cursor::MoveTo(start_col, start_row),
                    Print("┌".bold()),
                    cursor::MoveTo(end_col, start_row),
                    Print("┐".bold()),
                    cursor::MoveTo(start_col, end_row),
                    Print("└".bold()),
                    cursor::MoveTo(end_col, end_row),
                    Print("┘".bold())
                )
                .ok();
                for i in 0..logo.len() {
                    execute!(
                        stdout,
                        cursor::MoveTo(start_col + 2, start_row + 2 + i as u16),
                        Print(logo[i].clone().bold())
                    )
                    .ok();
                }
                for i in 0..options.len() {
                    execute!(
                        stdout,
                        cursor::MoveTo(
                            (term_cols - options[i].len() as u16) / 2,
                            start_row + logo.len() as u16 + 3 + i as u16
                        ),
                        Print(options[i].clone().bold())
                    )
                    .ok();
                }
                match read().unwrap() {
                    Event::Key(event) => match event.code {
                        _ => (),
                    },
                    _ => (),
                }
                return Some(current_selection);
            }
            MenuType::Message(message) => {
                execute!(
                    stdout,
                    cursor::MoveTo(
                        (term_cols - message.len() as u16) / 2 - 2,
                        term_rows / 2 - 2
                    ),
                    Print("┌".bold()),
                    cursor::MoveTo(
                        (term_cols + message.len() as u16) / 2 + 1,
                        term_rows / 2 - 2
                    ),
                    Print("┐".bold()),
                    cursor::MoveTo(
                        (term_cols - message.len() as u16) / 2 - 2,
                        term_rows / 2 + 2
                    ),
                    Print("└".bold()),
                    cursor::MoveTo(
                        (term_cols + message.len() as u16) / 2 + 1,
                        term_rows / 2 + 2
                    ),
                    Print("┘".bold()),
                    cursor::MoveTo(
                        (term_cols - message.len() as u16) / 2 - 2,
                        term_rows / 2 - 1
                    ),
                    Print("│".bold()),
                    cursor::MoveTo((term_cols - message.len() as u16) / 2 - 2, term_rows / 2),
                    Print("│".bold()),
                    cursor::MoveTo(
                        (term_cols - message.len() as u16) / 2 - 2,
                        term_rows / 2 + 1
                    ),
                    Print("│".bold()),
                    cursor::MoveTo(
                        (term_cols + message.len() as u16) / 2 + 1,
                        term_rows / 2 - 1
                    ),
                    Print("│".bold()),
                    cursor::MoveTo((term_cols + message.len() as u16) / 2 + 1, term_rows / 2),
                    Print("│".bold()),
                    cursor::MoveTo(
                        (term_cols + message.len() as u16) / 2 + 1,
                        term_rows / 2 + 1
                    ),
                    Print("│".bold()),
                    cursor::MoveTo((term_cols - message.len() as u16) / 2, term_rows / 2),
                    Print(message.clone().bold()),
                )
                .ok();
                for c in ((term_cols - message.len() as u16) / 2 - 1)
                    ..((term_cols + message.len() as u16) / 2 + 1)
                {
                    execute!(
                        stdout,
                        cursor::MoveTo(c, term_rows / 2 - 2),
                        Print("─".bold()),
                        cursor::MoveTo(c, term_rows / 2 + 2),
                        Print("─".bold()),
                    )
                    .ok();
                }
                match read().unwrap() {
                    Event::Key(event) => match event.code {
                        _ => (),
                    },
                    _ => (),
                }
            }
            _ => (),
        }
        None
    }
}
