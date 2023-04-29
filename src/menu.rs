use crate::{
    controls::Control,
    level::{Level, LevelSource},
    repository::Repository,
    userdata::CompletedRepoLevel,
};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
        StyledContent, Stylize,
    },
    terminal::{size, Clear, ClearType},
};
use std::io::stdout;

#[derive(Clone)]
pub enum Selection {
    Play(LevelSource),
    Repository,
    Help,
    Quit,
    Yes,
    No,
    Item(usize),
}

pub enum MenuType<'a> {
    /// Dialog box with a single message. Press `Enter` or `q` to close.
    Message(&'a str),

    /// Dialog box that displays the message `String` and will
    /// show a list of the given option `Strings`. Returns the
    /// index of the selected option on enter.
    Selection(&'a str, Vec<String>),

    /// Same as `MenuType::Selection` but only allows selecting
    /// `Yes` or `No`.
    YesNoSelection(&'a str),

    /// Same as `Message` but displays the entire help menu for
    /// the application in a `ScrollableMenu`.
    HelpMenu,

    /// Same as `Message` but displays more content and can be scrolled in.
    /// The `Vec<Vec<StyledContent<&'static str>>>` represents the list
    /// of `crossterm` styled lines where each inner `Vec` represents
    /// the list of chunks to print.
    ScrollableMenu(Vec<Vec<StyledContent<&'a str>>>),

    /// Draws the `Main Menu` of the application with the logo
    /// and selections for `Play`, `Help`, and `Quit`. Must
    /// provide a `Vec<usize>` representing the core levels the
    /// player has completed and a `Vec<Repository>` which is
    /// the list of available repositories the user has.
    ///
    /// Selecting `Play` will open the `CoreLevelSelection` and
    /// will return a `Selection::Play(l)` where `l` is the selected
    /// level. Selecting `Repository` or `Online` from the menu will
    /// return a `Selection::Play(l)` where `l` is the selected repository
    /// level.
    MainSelection(&'a Vec<usize>),

    /// Draws the `Core Level` selection menu for the player
    /// to choose one of the built-in levels. Must be provided
    /// a `Vec<usize>` representing the core levels the player
    /// has completed.
    CoreLevelSelection(&'a Vec<usize>),

    /// Draws the `Repository` selection menu to allow
    /// the player to select which repo they want to play
    /// a level from. After selecting a repo, the `RepositoryLevelSelection`
    /// is opened to select the url of the level.
    RepositorySelection(&'a Vec<Repository>),

    RepositoryLevelSelection(Repository, &'a Vec<CompletedRepoLevel>),
}

const RED: Color = Color::Rgb { r: 255, g: 0, b: 0 };
const YELLOW: Color = Color::Rgb {
    r: 255,
    g: 255,
    b: 0,
};

pub struct Menu;

impl Menu {
    fn draw_borders(
        start_row: u16,
        end_row: u16,
        start_col: u16,
        end_col: u16,
    ) -> crossterm::Result<()> {
        for r in start_row..=end_row {
            for c in start_col..=end_col {
                if r == start_row || r == end_row {
                    execute!(stdout(), MoveTo(c, r), Print("─"),)?;
                } else if c == start_col || c == end_col {
                    execute!(stdout(), MoveTo(c, r), Print("│"),)?;
                } else {
                    execute!(stdout(), MoveTo(c, r), Print(" "),)?;
                }
            }
        }
        execute!(
            stdout(),
            MoveTo(start_col, start_row),
            Print("┌"),
            MoveTo(end_col, start_row),
            Print("┐"),
            MoveTo(start_col, end_row),
            Print("└"),
            MoveTo(end_col, end_row),
            Print("┘"),
        )
    }

    pub fn open(menu_type: MenuType) -> Option<Selection> {
        let row_padding = 1;
        let col_padding = 2;
        match menu_type {
            MenuType::MainSelection(completed_levels) => {
                let row_padding = 2;
                let col_padding = 3;
                let options: [Selection; 4] = [
                    Selection::Play(LevelSource::Core(0)),
                    Selection::Repository,
                    Selection::Help,
                    Selection::Quit,
                ];
                let mut current_selection = 0;
                loop {
                    let (term_cols, term_rows) = size().unwrap_or((0, 0));
                    let start_row: u16 =
                        (term_rows - options.len() as u16 * 2 - 10 - row_padding) / 2 - row_padding;
                    let mut start_col: u16 = (term_cols - 23) / 2 - col_padding;
                    let end_row: u16 =
                        (term_rows + options.len() as u16 + 10 + row_padding) / 2 + row_padding;
                    let end_col: u16 = (term_cols + 23) / 2 + col_padding;
                    if (end_col - start_col) % 2 != 0 {
                        start_col -= 1;
                    }
                    execute!(stdout(), Clear(ClearType::All)).ok();
                    Menu::draw_borders(start_row, end_row, start_col, end_col).ok();
                    execute!(
                        stdout(),
                        SetAttribute(Attribute::Bold),
                        MoveTo(start_col + col_padding, start_row + row_padding + 1),
                        Print("          /"),
                        SetForegroundColor(RED),
                        Print("-------"),
                        SetBackgroundColor(RED),
                        Print("L"),
                        SetBackgroundColor(Color::Reset),
                        MoveTo(start_col + col_padding, start_row + row_padding + 2),
                        SetForegroundColor(Color::Green),
                        Print(" ___      "),
                        SetForegroundColor(RED),
                        Print("|"),
                        SetForegroundColor(Color::Green),
                        Print("__      _"),
                        MoveTo(start_col + col_padding, start_row + row_padding + 3),
                        Print("|_  |  "),
                        SetForegroundColor(RED),
                        Print("<--"),
                        SetForegroundColor(Color::White),
                        Print("/"),
                        SetForegroundColor(Color::Green),
                        Print("  |    | \\_"),
                        MoveTo(start_col + col_padding, start_row + row_padding + 4),
                        Print("  | |     `| |    | __|"),
                        MoveTo(start_col + col_padding, start_row + row_padding + 5),
                        Print("  | |      | |    | |"),
                        MoveTo(start_col + col_padding, start_row + row_padding + 6),
                        Print("  | |_    _|_|_   | |_ "),
                        MoveTo(start_col + col_padding, start_row + row_padding + 7),
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
                        MoveTo(start_col + col_padding, start_row + row_padding + 8),
                        SetForegroundColor(RED),
                        Print("  |                v"),
                        MoveTo(start_col + col_padding, start_row + row_padding + 9),
                        Print("  v"),
                        MoveTo(start_col + col_padding, start_row + row_padding + 10),
                        Print("  "),
                        SetForegroundColor(YELLOW),
                        SetBackgroundColor(YELLOW),
                        Print("S"),
                        ResetColor,
                    )
                    .ok();
                    for (i, _) in options.iter().enumerate() {
                        let option = match options[i] {
                            Selection::Play(_) => "P L A Y",
                            Selection::Repository => "O N L I N E",
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
                            MoveTo(
                                (term_cols - 23) / 2,
                                start_row + row_padding * 2 + i as u16 * 2 + 10,
                            ),
                            SetAttribute(Attribute::Bold),
                            Print(format!("{:^23}", option)),
                            ResetColor,
                        )
                        .ok();
                    }
                    match Control::read_input() {
                        Control::Select => match options[current_selection] {
                            Selection::Play(_) => {
                                if let Some(Selection::Item(i)) =
                                    Menu::open(MenuType::CoreLevelSelection(completed_levels))
                                {
                                    return Some(Selection::Play(LevelSource::Core(i)));
                                }
                            }
                            _ => break,
                        },
                        Control::Up => {
                            if current_selection == 0 {
                                current_selection = options.len() - 1;
                            } else {
                                current_selection -= 1;
                            }
                        }
                        Control::Down => {
                            current_selection = (current_selection + 1) % options.len();
                        }
                        Control::Quit => return Some(Selection::Quit),
                        _ => (),
                    }
                }
                return Some(options[current_selection].clone());
            }
            MenuType::Message(message) => loop {
                let (term_cols, term_rows) = size().unwrap_or((0, 0));
                let start_row: u16 = term_rows / 2 - row_padding - 1;
                let start_col: u16 =
                    (term_cols - (term_cols - 4).min(message.len() as u16)) / 2 - col_padding;
                let end_row: u16 = (term_rows + row_padding) / 2 + row_padding;
                let end_col: u16 = (term_cols + message.len() as u16) / 2 + col_padding;
                Menu::draw_borders(start_row, end_row, start_col, end_col).ok();
                execute!(
                    stdout(),
                    MoveTo(start_col + 2, term_rows / 2),
                    Print(message),
                )
                .ok();
                if let Control::Select = Control::read_input() {
                    break;
                }
            },
            MenuType::YesNoSelection(message) => {
                let mut current_selection = Selection::No;
                loop {
                    let (term_cols, term_rows) = size().unwrap_or((0, 0));
                    let start_row: u16 = term_rows / 2 - row_padding - 1;
                    let start_col: u16 = (term_cols - message.len() as u16) / 2 - col_padding;
                    let end_row: u16 = (term_rows + row_padding) / 2 + row_padding + 2;
                    let end_col: u16 = (term_cols + message.len() as u16) / 2 + col_padding;
                    Menu::draw_borders(start_row, end_row, start_col, end_col).ok();
                    execute!(
                        stdout(),
                        MoveTo((term_cols - message.len() as u16) / 2, term_rows / 2),
                        Print(message),
                    )
                    .ok();
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
                    match Control::read_input() {
                        Control::Left | Control::Right => {
                            if matches!(current_selection, Selection::No) {
                                current_selection = Selection::Yes
                            } else {
                                current_selection = Selection::No
                            }
                        }
                        Control::Select => return Some(current_selection),
                        Control::Quit => return Some(Selection::No),
                        _ => (),
                    }
                }
            }
            MenuType::HelpMenu => {
                return Menu::open(MenuType::ScrollableMenu(vec![
                    vec![],
                    vec![
                        "In ".stylize(),
                        "l1t".bold().green(),
                        ", your goal is to use the available lasers ".stylize(),
                    ],
                    vec!["to light up all of the statues in the level.".stylize()],
                    vec![],
                    vec!["CONTROLS".bold().underlined()],
                    vec![],
                    vec![" W - ".bold(), "Move Up".stylize()],
                    vec![" S - ".bold(), "Move Down".stylize()],
                    vec![" A - ".bold(), "Move Left".stylize()],
                    vec![" D - ".bold(), "Move Right".stylize()],
                    vec![
                        " Space - ".bold(),
                        "Toggle surrounding blocks (if able)".stylize(),
                    ],
                    vec![" Shift-H - ".bold(), "Show this help menu".stylize()],
                    vec![" Q - ".bold(), "Quit".stylize()],
                    vec![],
                    vec!["Arrow keys can also be used to move around the ".stylize()],
                    vec!["level".stylize()],
                    vec![],
                    vec![
                        "X".green().on_green(),
                        " ".stylize(),
                        "PLAYER".bold().underlined(),
                    ],
                    vec![],
                    vec!["Hey, that's you!".stylize()],
                    vec![],
                    vec![
                        "L".with(RED).on(RED),
                        " ".stylize(),
                        "LASERS".bold().underlined(),
                    ],
                    vec![],
                    vec!["Lasers shoot laser beams in their set direction".stylize()],
                    vec![
                        "(".stylize(),
                        "UP, DOWN, LEFT, RIGHT".bold(),
                        "). Laser beams are the key".stylize(),
                    ],
                    vec!["to winning the game and can affect various ".stylize()],
                    vec!["blocks.".stylize()],
                    vec![],
                    vec!["Lasers cannot change directions but they can".stylize()],
                    vec!["be toggled on and off.".stylize()],
                    vec![],
                    vec![
                        "If a laser hits you, you'll ".stylize(),
                        "die".with(RED).bold(),
                        " and have to ".stylize(),
                    ],
                    vec!["restart the level.".stylize()],
                    vec![],
                    vec!["If a laser is hit by a laser beam, it will".stylize()],
                    vec!["turn off and must be toggled on by the player.".stylize()],
                    vec![],
                    vec![
                        "S".with(YELLOW).on(YELLOW),
                        " ".stylize(),
                        "STATUES".bold().underlined(),
                    ],
                    vec![],
                    vec!["All statues in a level must be lit up by a ".stylize()],
                    vec![
                        "laser beam to ".stylize(),
                        "win".with(YELLOW).bold(),
                        " the level.".stylize(),
                    ],
                    vec![],
                    vec!["Statues can not be moved or manually toggled.".stylize()],
                    vec![],
                    vec![
                        "R".bold().black().on(YELLOW),
                        " ".stylize(),
                        "REVERSE STATUES".bold().underlined(),
                    ],
                    vec![],
                    vec![
                        "Same as statues except they must ".stylize(),
                        "NOT".bold().italic(),
                        " be lit up ".stylize(),
                    ],
                    vec![
                        "to ".stylize(),
                        "win".with(YELLOW).bold(),
                        " the level.".stylize(),
                    ],
                    vec![],
                    vec!["/ ".bold(), "MIRRORS".bold().underlined()],
                    vec![],
                    vec!["Mirrors reflect laser beams in different".stylize()],
                    vec!["directions.".stylize()],
                    vec![],
                    vec!["             ".stylize(), "L".with(RED).on(RED)],
                    vec!["             |".bold().with(RED)],
                    vec![
                        "L".with(RED).on(RED),
                        "----".bold().with(RED),
                        "\\".bold(),
                        "    <--".bold().with(RED),
                        "/".bold(),
                    ],
                    vec!["     |".with(RED).bold()],
                    vec!["     V".with(RED).bold()],
                    vec![],
                    vec!["Mirrors cannot be moved but their direction can ".stylize()],
                    vec!["be toggled by the player.".stylize()],
                    vec![],
                    vec![
                        "/".black().on_white().bold(),
                        " ".stylize(),
                        "MOVEABLE MIRRORS".bold().underlined(),
                    ],
                    vec![],
                    vec!["Moveable Mirrors are the same as mirrors except ".stylize()],
                    vec![
                        "they ".stylize(),
                        "CAN ".bold().italic(),
                        "be moved.".stylize(),
                    ],
                    vec![],
                    vec![
                        "Z".bold().yellow().on_black(),
                        " ".stylize(),
                        "ZAPPERS".bold().underlined(),
                    ],
                    vec![],
                    vec!["If any Zappers are lit by a laser beam, you".stylize()],
                    vec![
                        "will immediately ".stylize(),
                        "lose".with(RED).bold(),
                        " the level.".stylize(),
                    ],
                    vec![],
                    vec![
                        "I".bold().white().on_white(),
                        " ".stylize(),
                        "B".bold().grey().on_grey(),
                        " ".stylize(),
                        "s".bold().black().on_red(),
                        " ".stylize(),
                        "OTHER BLOCKS".bold().underlined(),
                    ],
                    vec![],
                    vec![
                        "I".bold().white().on_white(),
                        " Walls - ".bold(),
                        "Cannot be moved by player, will block".stylize(),
                    ],
                    vec!["          laser beams.".stylize()],
                    vec![],
                    vec![
                        "B".bold().grey().on_grey(),
                        " Blocks - ".bold(),
                        "Can be moved around and will block".stylize(),
                    ],
                    vec!["           laser beams.".stylize()],
                    vec![],
                    vec![
                        "T".bold().magenta().on_magenta(),
                        " Toggle Blocks - ".bold(),
                        "Cannot be moved. Switches and".stylize(),
                    ],
                    vec!["                  buttons can toggle these on".stylize()],
                    vec!["                  and off.".stylize()],
                    vec![],
                    vec![
                        "s".bold().black().on_red(),
                        " Switches - ".bold(),
                        "When toggled, will turn toggle".stylize(),
                    ],
                    vec!["             blocks on/off.".stylize()],
                    vec![],
                    vec![
                        "b".bold().black().on_red(),
                        " Buttons - ".bold(),
                        "When pressed, will turn toggle".stylize(),
                    ],
                    vec!["            blocks on/off. Player must be".stylize()],
                    vec!["            next to button to press.".stylize()],
                    vec![],
                    vec!["REPOSITORIES".bold().underlined()],
                    vec![],
                    vec!["Repositories allow you to play levels hosted ".stylize()],
                    vec!["online. To add a repository, add a line in ".stylize()],
                    vec!["your `$HOME/.l1t/repositories.l1t_conf` file:".stylize()],
                    vec![],
                    vec!["My Repo Name = http://myrepourl.com".stylize()],
                    vec![],
                    vec!["The left side of the `=` is the repository ".stylize()],
                    vec!["name and the right is the URL of the repo.".stylize()],
                    vec![],
                    vec!["To host your own repo, checkout the repo ".stylize()],
                    vec!["documentation at:".stylize()],
                    vec![],
                    vec!["https://github.com/alex-laycalvert/l1t/".stylize()],
                ]))
            }
            MenuType::ScrollableMenu(content) => {
                let mut start_index: usize = 0;
                let scroll_message = "  USE ARROW KEYS OR W, S TO SCROLL  ";
                let fast_scroll_message = "  USE g AND G to GOTO TOP AND BOTTOM  ";
                loop {
                    let (term_cols, term_rows) = size().unwrap_or((0, 0));
                    let lines: usize = (term_rows - row_padding * 2) as usize - 6;
                    let start_row = (term_rows - lines as u16) / 2 - row_padding;
                    let end_row = (term_rows + lines as u16) / 2 + row_padding;
                    let start_col = (term_cols - 50) / 2 - col_padding;
                    let end_col = (term_cols + 50) / 2 + col_padding;
                    execute!(
                        stdout(),
                        Clear(ClearType::All),
                        MoveTo((term_cols - scroll_message.len() as u16) / 2, start_row - 1),
                        Print(scroll_message.on_white().black().bold()),
                        MoveTo(
                            (term_cols - fast_scroll_message.len() as u16) / 2,
                            end_row + 1
                        ),
                        Print(fast_scroll_message.on_white().black().bold())
                    )
                    .ok();
                    Menu::draw_borders(start_row, end_row, start_col, end_col).ok();
                    for (i, line) in content
                        .iter()
                        .enumerate()
                        .take((start_index + lines).min(content.len()))
                        .skip(start_index)
                    {
                        execute!(
                            stdout(),
                            MoveTo(
                                start_col + col_padding + 1,
                                start_row + row_padding + (i - start_index) as u16
                            )
                        )
                        .ok();
                        for piece in line.iter() {
                            execute!(stdout(), Print(piece)).ok();
                        }
                    }
                    match Control::read_input() {
                        Control::Up => {
                            if start_index == 0 {
                                continue;
                            }
                            start_index -= 1;
                        }
                        Control::Down => {
                            if start_index + lines >= content.len() {
                                continue;
                            }
                            start_index += 1;
                        }
                        Control::GotoTop => start_index = 0,
                        Control::GotoBottom => start_index = content.len() - lines,
                        Control::Select | Control::Quit => break,
                        _ => (),
                    }
                }
            }
            MenuType::CoreLevelSelection(completed_levels) => {
                let num_levels = Level::NUM_CORE_LEVELS as f64;
                let levels_per_row = num_levels.sqrt() as u16;
                let num_rows = (num_levels / levels_per_row as f64).ceil() as u16;
                let highest_available_level = match completed_levels.iter().max() {
                    Some(n) => *n.min(&(Level::NUM_CORE_LEVELS - 1)) + 1,
                    None => 0,
                };
                let mut current_selection = highest_available_level;
                let message = "  SELECT A LEVEL  ";
                loop {
                    let (term_cols, term_rows) = size().unwrap_or((0, 0));
                    let start_row: u16 = (term_rows - num_rows * 2) / 2;
                    let mut start_col: u16 = (term_cols / 2) - levels_per_row * 2;
                    let end_row: u16 = (term_rows + num_rows * 2) / 2;
                    let end_col: u16 = (term_cols / 2) + levels_per_row * 2;
                    if (end_col - start_col + 1) % 2 != 0 {
                        start_col -= 1;
                    }
                    execute!(
                        stdout(),
                        Clear(ClearType::All),
                        MoveTo((term_cols - message.len() as u16) / 2, start_row - 1),
                        Print(message.on_white().black().bold())
                    )
                    .ok();
                    Menu::draw_borders(start_row, end_row, start_col, end_col).ok();
                    for i in 0..Level::NUM_CORE_LEVELS {
                        let is_available = i <= highest_available_level;
                        let fg_color = if is_available && current_selection != i {
                            Color::White
                        } else {
                            Color::Black
                        };
                        execute!(
                            stdout(),
                            MoveTo(
                                (i as u16 % levels_per_row) * 2
                                    + start_col
                                    + col_padding
                                    + (i as u16 % levels_per_row) * 2,
                                start_row + 1 + (i as u16 / levels_per_row) * 2,
                            ),
                            SetForegroundColor(fg_color),
                            SetBackgroundColor(if current_selection == i {
                                Color::White
                            } else {
                                Color::Reset
                            }),
                            Print(format!("{:0>2}", (i + 1).to_string()).bold()),
                        )
                        .ok();
                    }
                    match Control::read_input() {
                        Control::Up => {
                            if current_selection == 0 {
                                current_selection = highest_available_level;
                            } else if current_selection >= levels_per_row.into() {
                                current_selection -= levels_per_row as usize;
                            } else {
                                current_selection = 0;
                            }
                        }
                        Control::Down => {
                            if current_selection == highest_available_level {
                                current_selection = 0;
                            } else if current_selection
                                < (Level::NUM_CORE_LEVELS as i16 - levels_per_row as i16) as usize
                                && current_selection + (levels_per_row as usize)
                                    <= highest_available_level
                            {
                                current_selection += levels_per_row as usize;
                            } else {
                                current_selection = highest_available_level;
                            }
                        }
                        Control::Left => {
                            if current_selection == 0 {
                                current_selection = highest_available_level;
                            } else {
                                current_selection -= 1;
                            }
                        }
                        Control::Right => {
                            if current_selection >= highest_available_level {
                                current_selection = 0;
                            } else {
                                current_selection += 1;
                            }
                        }
                        Control::Quit => return None,
                        Control::Select => return Some(Selection::Item(current_selection)),
                        _ => (),
                    }
                }
            }
            MenuType::RepositorySelection(repositories) => {
                let message = " SELECT A REPO ";
                let mut current_selection = 0;
                loop {
                    let (term_cols, term_rows) = size().unwrap_or((0, 0));
                    let num_cols = (term_cols - 4).min(200) as usize;
                    let repo_name_len = num_cols / 2 - 2;
                    let repo_url_len = num_cols - 4 - repo_name_len;
                    let start_row: u16 = (term_rows - repositories.len() as u16) / 2;
                    let start_col: u16 = (term_cols - num_cols as u16) / 2;
                    let end_row: u16 = (term_rows + repositories.len() as u16) / 2 + 1;
                    let end_col: u16 = (term_cols + num_cols as u16) / 2;
                    execute!(
                        stdout(),
                        Clear(ClearType::All),
                        MoveTo((term_cols - message.len() as u16) / 2, start_row - 1),
                        Print(message.on_white().black().bold())
                    )
                    .ok();
                    Menu::draw_borders(start_row, end_row, start_col, end_col).ok();
                    for (i, repo) in repositories.iter().enumerate() {
                        execute!(
                            stdout(),
                            SetBackgroundColor(if i == current_selection {
                                Color::White
                            } else {
                                Color::Reset
                            }),
                            SetForegroundColor(if i == current_selection {
                                Color::Black
                            } else {
                                Color::White
                            }),
                            MoveTo(start_col + 1, start_row + i as u16 + 1),
                            Print(
                                format!(
                                    " {: <repo_name_len$} {: <repo_url_len$} ",
                                    &repo.name[0..repo.name.len().min(repo_name_len)],
                                    &repo.url[0..repo.url.len().min(repo_url_len)],
                                )
                                .bold()
                            ),
                            ResetColor
                        )
                        .ok();
                    }
                    match Control::read_input() {
                        Control::Up => {
                            if current_selection == 0 {
                                current_selection = repositories.len() - 1;
                            } else {
                                current_selection -= 1;
                            }
                        }
                        Control::Down => {
                            if current_selection == repositories.len() - 1 {
                                current_selection = 0;
                            } else {
                                current_selection += 1;
                            }
                        }
                        Control::Select => return Some(Selection::Item(current_selection)),
                        Control::None => continue,
                        _ => break,
                    }
                }
            }
            MenuType::RepositoryLevelSelection(repository, completed_levels) => {
                let message = " SELECT A REPO ";
                let mut current_selection = 0;
                loop {
                    let (term_cols, term_rows) = size().unwrap_or((0, 0));
                    let num_cols = (term_cols - 4).min(200) as usize;
                    let level_name_len = num_cols / 5 - 2;
                    let level_author_len = level_name_len;
                    let level_desc_len = num_cols - level_name_len - level_author_len - 6;
                    let start_row: u16 = (term_rows - repository.levels.len() as u16) / 2;
                    let start_col: u16 = (term_cols - num_cols as u16) / 2;
                    let end_row: u16 = (term_rows + repository.levels.len() as u16) / 2 + 1;
                    let end_col: u16 = (term_cols + num_cols as u16) / 2;
                    execute!(
                        stdout(),
                        Clear(ClearType::All),
                        MoveTo((term_cols - message.len() as u16) / 2, start_row - 1),
                        Print(message.on_white().black().bold())
                    )
                    .ok();
                    Menu::draw_borders(start_row, end_row, start_col, end_col).ok();
                    for (i, level) in repository.levels.iter().enumerate() {
                        if let LevelSource::Url(url) = &level.source {
                            let completed = completed_levels.iter().any(|l| {
                                l.url == *url || (l.name == level.name && l.author == level.author)
                            });
                            execute!(
                                stdout(),
                                SetBackgroundColor(if i == current_selection {
                                    Color::White
                                } else {
                                    Color::Reset
                                }),
                                SetForegroundColor(if i == current_selection {
                                    Color::Black
                                } else {
                                    Color::White
                                }),
                                MoveTo(start_col + 1, start_row + i as u16 + 1),
                                Print(
                                    format!(
                                        " {} {: <level_name_len$} {: <level_author_len$} {: <level_desc_len$}",
                                        if completed { "\u{2713}" } else { " " },
                                        &level.name[0..level.name.len().min(level_name_len)],
                                        &level.author[0..level.author.len().min(level_author_len)],
                                        &level.description[0..level.description.len().min(level_desc_len)])
                                    .bold()
                                ),
                                ResetColor
                            )
                            .ok();
                        }
                    }
                    match Control::read_input() {
                        Control::Up => {
                            if current_selection == 0 {
                                current_selection = repository.levels.len() - 1;
                            } else {
                                current_selection -= 1;
                            }
                        }
                        Control::Down => {
                            if current_selection == repository.levels.len() - 1 {
                                current_selection = 0;
                            } else {
                                current_selection += 1;
                            }
                        }
                        Control::Select => return Some(Selection::Item(current_selection)),
                        Control::Quit => return Some(Selection::Quit),
                        Control::None => continue,
                        _ => break,
                    }
                }
            }
            _ => (),
        }
        None
    }
}
