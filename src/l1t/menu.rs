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
                        (term_cols + message.len() as u16 + 1) / 2,
                        term_rows / 2 - 1
                    ),
                    Print("│".bold()),
                    cursor::MoveTo((term_cols + message.len() as u16 + 1) / 2, term_rows / 2),
                    Print("│".bold()),
                    cursor::MoveTo(
                        (term_cols + message.len() as u16 + 1) / 2,
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
