use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyCode},
    execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
        Stylize,
    },
};
use std::io::stdout;

#[derive(Clone, Copy)]
pub enum Selection {
    Play,
    Help,
    Quit,
    Yes,
    No,
}

pub enum MenuType {
    /// Dialog box with a single message. Press `Enter` to close.
    Message(String),

    /// Dialog box that displays the message `String` and will
    /// show a list of the given option `Strings`. Returns the
    /// index of the selected option on enter.
    Selection(String, Vec<String>),

    /// Same as `MenuType::Selection` but only allows selecting
    /// `Yes` or `No`.
    YesNoSelection(String),

    /// Same as `Message` but displays the entire help menu for
    /// the application.
    HelpMenu,

    /// Prints out the `Main Menu` of the application with the logo
    /// and selections for `Play`, `Help`, and `Quit`.
    /// The `Help` option will open up the `HelpMenu` and not return as selection.
    MainSelection,
}

pub struct Menu;

impl Menu {
    pub fn draw(menu_type: MenuType, term_rows: u16, term_cols: u16) -> Option<Selection> {
        match menu_type {
            MenuType::MainSelection => {
                let options: Vec<Selection> =
                    vec![Selection::Play, Selection::Help, Selection::Quit];
                const RED: Color = Color::Rgb { r: 255, g: 0, b: 0 };
                const YELLOW: Color = Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 0,
                };
                let row_padding = 2;
                let col_padding = 3;
                let start_row: u16 =
                    (term_rows - options.len() as u16 * 2 - 9 - row_padding) / 2 - row_padding;
                let start_col: u16 = (term_cols - 23) / 2 - col_padding;
                let end_row: u16 =
                    (term_rows + options.len() as u16 + 10 + row_padding) / 2 + row_padding;
                let end_col: u16 = (term_cols + 23) / 2 + col_padding;
                for r in (start_row - 1)..=end_row {
                    for c in (start_col - 1)..=end_col {
                        if r == start_row - 1 || r == end_row {
                            execute!(stdout(), cursor::MoveTo(c, r), Print("─"),).ok();
                        } else if c == start_col - 1 || c == end_col {
                            execute!(stdout(), cursor::MoveTo(c, r), Print("│"),).ok();
                        } else {
                            execute!(stdout(), cursor::MoveTo(c, r), Print(" "),).ok();
                        }
                    }
                }
                execute!(
                    stdout(),
                    cursor::MoveTo(start_col - 1, start_row - 1),
                    Print("┌"),
                    cursor::MoveTo(end_col, start_row - 1),
                    Print("┐"),
                    cursor::MoveTo(start_col - 1, end_row),
                    Print("└"),
                    cursor::MoveTo(end_col, end_row),
                    Print("┘"),
                    SetAttribute(Attribute::Bold),
                    MoveTo(start_col + col_padding, start_row + row_padding),
                    Print("          /"),
                    SetForegroundColor(RED),
                    Print("-------"),
                    SetBackgroundColor(RED),
                    Print("L"),
                    SetBackgroundColor(Color::Reset),
                    MoveTo(start_col + col_padding, start_row + row_padding + 1),
                    SetForegroundColor(Color::Green),
                    Print(" ___      "),
                    SetForegroundColor(RED),
                    Print("|"),
                    SetForegroundColor(Color::Green),
                    Print("__      _"),
                    MoveTo(start_col + col_padding, start_row + row_padding + 2),
                    Print("|_  |  "),
                    SetForegroundColor(RED),
                    Print("<--"),
                    SetForegroundColor(Color::White),
                    Print("/"),
                    SetForegroundColor(Color::Green),
                    Print("  |    | \\_"),
                    MoveTo(start_col + col_padding, start_row + row_padding + 3),
                    Print("  | |     `| |    | __|"),
                    MoveTo(start_col + col_padding, start_row + row_padding + 4),
                    Print("  | |      | |    | |"),
                    MoveTo(start_col + col_padding, start_row + row_padding + 5),
                    Print("  | |_    _|_|_   | |_ "),
                    MoveTo(start_col + col_padding, start_row + row_padding + 6),
                    SetForegroundColor(RED),
                    Print("--"),
                    SetForegroundColor(Color::White),
                    Print("\\"),
                    SetForegroundColor(Color::Green),
                    Print("___\\  |_____| "),
                    SetForegroundColor(RED),
                    Print("--"),
                    SetForegroundColor(Color::White),
                    Print("\\"),
                    SetForegroundColor(Color::Green),
                    Print("__|"),
                    MoveTo(start_col + col_padding, start_row + row_padding + 7),
                    SetForegroundColor(RED),
                    Print("  |                v"),
                    MoveTo(start_col + col_padding, start_row + row_padding + 8),
                    Print("  v"),
                    MoveTo(start_col + col_padding, start_row + row_padding + 9),
                    Print("  "),
                    SetForegroundColor(YELLOW),
                    SetBackgroundColor(YELLOW),
                    Print("S"),
                    ResetColor,
                )
                .ok();
                let mut current_selection = 0;
                loop {
                    for i in 0..options.len() {
                        let option = match options[i] {
                            Selection::Play => "P L A Y",
                            Selection::Help => "H E L P",
                            Selection::Quit => "Q U I T",
                            _ => "",
                        };
                        execute!(
                            stdout(),
                            SetForegroundColor(if i == current_selection {
                                Color::Black
                            } else {
                                Color::White
                            }),
                            SetBackgroundColor(if i == current_selection {
                                Color::White
                            } else {
                                Color::Reset
                            }),
                            cursor::MoveTo(
                                (term_cols - option.len() as u16) / 2 - 8,
                                start_row + row_padding * 2 + i as u16 * 2 + 10,
                            ),
                            SetAttribute(Attribute::Bold),
                            Print(format!("{:^23}", option)),
                            ResetColor,
                        )
                        .ok();
                    }
                    match read().unwrap() {
                        Event::Key(event) => match event.code {
                            KeyCode::Enter => break,
                            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                                if current_selection == 0 {
                                    current_selection = options.len() - 1;
                                } else {
                                    current_selection -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                                current_selection = (current_selection + 1) % options.len();
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
                return Some(options[current_selection]);
            }
            MenuType::Message(message) => {
                let row_padding = 1;
                let col_padding = 3;
                let start_row: u16 = term_rows / 2 - row_padding - 1;
                let start_col: u16 = (term_cols - message.len() as u16) / 2 - col_padding;
                let end_row: u16 = (term_rows + row_padding) / 2 + row_padding;
                let end_col: u16 = (term_cols + message.len() as u16) / 2 + col_padding;
                for r in start_row..=end_row {
                    for c in start_col..=end_col {
                        if r == start_row || r == end_row {
                            execute!(stdout(), cursor::MoveTo(c, r), Print("─"),).ok();
                        } else if c == start_col || c == end_col {
                            execute!(stdout(), cursor::MoveTo(c, r), Print("│"),).ok();
                        } else {
                            execute!(stdout(), cursor::MoveTo(c, r), Print(" "),).ok();
                        }
                    }
                }
                execute!(
                    stdout(),
                    cursor::MoveTo(start_col, start_row),
                    Print("┌"),
                    cursor::MoveTo(end_col, start_row),
                    Print("┐"),
                    cursor::MoveTo(start_col, end_row),
                    Print("└"),
                    cursor::MoveTo(end_col, end_row),
                    Print("┘"),
                    cursor::MoveTo((term_cols - message.len() as u16) / 2, term_rows / 2),
                    Print(message.clone()),
                )
                .ok();
                loop {
                    match read().unwrap() {
                        Event::Key(event) => match event.code {
                            KeyCode::Enter => break,
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            MenuType::YesNoSelection(message) => {
                let row_padding = 1;
                let col_padding = 3;
                let start_row: u16 = term_rows / 2 - row_padding - 1;
                let start_col: u16 = (term_cols - message.len() as u16) / 2 - col_padding;
                let end_row: u16 = (term_rows + row_padding) / 2 + row_padding + 2;
                let end_col: u16 = (term_cols + message.len() as u16) / 2 + col_padding;
                for r in start_row..=end_row {
                    for c in start_col..=end_col {
                        if r == start_row || r == end_row {
                            execute!(stdout(), cursor::MoveTo(c, r), Print("─"),).ok();
                        } else if c == start_col || c == end_col {
                            execute!(stdout(), cursor::MoveTo(c, r), Print("│"),).ok();
                        } else {
                            execute!(stdout(), cursor::MoveTo(c, r), Print(" "),).ok();
                        }
                    }
                }
                execute!(
                    stdout(),
                    cursor::MoveTo(start_col, start_row),
                    Print("┌"),
                    cursor::MoveTo(end_col, start_row),
                    Print("┐"),
                    cursor::MoveTo(start_col, end_row),
                    Print("└"),
                    cursor::MoveTo(end_col, end_row),
                    Print("┘"),
                    cursor::MoveTo((term_cols - message.len() as u16) / 2, term_rows / 2),
                    Print(message.clone()),
                )
                .ok();
                let mut current_selection = Selection::No;
                loop {
                    execute!(
                        stdout(),
                        MoveTo(term_cols / 2 - 6, end_row - row_padding - 1),
                        SetForegroundColor(if matches!(current_selection, Selection::Yes) {
                            Color::Black
                        } else {
                            Color::White
                        }),
                        SetBackgroundColor(if matches!(current_selection, Selection::Yes) {
                            Color::White
                        } else {
                            Color::Reset
                        }),
                        Print(" YES ".bold()),
                        MoveTo(term_cols / 2 + 1, end_row - row_padding - 1),
                        SetForegroundColor(if matches!(current_selection, Selection::No) {
                            Color::Black
                        } else {
                            Color::White
                        }),
                        SetBackgroundColor(if matches!(current_selection, Selection::No) {
                            Color::White
                        } else {
                            Color::Reset
                        }),
                        Print(" NO ".bold())
                    )
                    .ok();
                    match read().unwrap() {
                        Event::Key(event) => match event.code {
                            KeyCode::Left
                            | KeyCode::Right
                            | KeyCode::Char('a')
                            | KeyCode::Char('d')
                            | KeyCode::Char('h')
                            | KeyCode::Char('l') => {
                                if matches!(current_selection, Selection::No) {
                                    current_selection = Selection::Yes
                                } else {
                                    current_selection = Selection::No
                                }
                            }
                            KeyCode::Char('q') => return Some(Selection::No),
                            KeyCode::Enter => return Some(current_selection),
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        None
    }
}
