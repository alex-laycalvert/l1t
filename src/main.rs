use clap::Parser;
use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand,
};
use home::home_dir;
use l1t::level::*;
use l1t::menu::*;
use l1t::userdata::*;
use std::{
    io::stdout,
    path::{Path, PathBuf},
};
use std::{thread, time};

const SLEEP_TIME: u64 = 500;

/// A terminal based strategy game about shooting lasers and lighting statues
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The `.l1t` file to load a level from
    #[arg(short, long)]
    file: Option<PathBuf>,
    ///// Repository to download levels from
    //#[arg(short, long)]
    //repo_url: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut stdout = stdout();
    enable_raw_mode().ok();
    stdout.execute(cursor::Hide).ok();

    if let Some(filename) = &args.file {
        // File has been provided
        play_file(filename);
    } else {
        play_core();
    }

    stdout.execute(cursor::Show).ok();
    disable_raw_mode().ok();
    stdout.execute(cursor::MoveTo(0, 0)).ok();
    stdout
        .execute(Clear(crossterm::terminal::ClearType::All))
        .ok();
}

fn play_core() {
    let home = match home_dir() {
        Some(h) => h,
        None => return,
    };
    let mut user_data = match UserData::read(&home) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    loop {
        let selection = Menu::open(MenuType::MainSelection(
            user_data.completed_core_levels.clone(),
        ))
        .unwrap_or(Selection::Play(1));
        match selection {
            Selection::Play(level) => {
                let mut current_level = level;
                loop {
                    if current_level >= Level::NUM_CORE_LEVELS {
                        Menu::open(MenuType::Message(
                            "You've completed all core levels, thanks for playing!".to_string(),
                        ));
                        break;
                    }
                    let mut level = match Level::core(current_level) {
                        Ok(l) => l,
                        Err(e) => {
                            eprintln!("{e}");
                            return;
                        }
                    };
                    let result = level.play();
                    match result {
                        Ok(result) => {
                            if result.has_won {
                                thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                                Menu::open(MenuType::Message("YAY, You Won!".to_string()));
                                current_level += 1;
                                if let Err(e) = user_data.complete_core(current_level) {
                                    eprintln!("{e}");
                                    return;
                                };
                            } else if let Some(r) = result.reason_for_loss {
                                match r {
                                    LevelLossReason::Zapper => {
                                        thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                                        Menu::open(MenuType::Message(
                                            "Uh oh, you lit a zapper!".to_string(),
                                        ));
                                    }
                                    LevelLossReason::Death => {
                                        thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                                        Menu::open(MenuType::Message(
                                            "Uh oh, you got shot by a laser beam!".to_string(),
                                        ));
                                    }
                                    LevelLossReason::Quit => break,
                                }
                            } else {
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            break;
                        }
                    }
                }
            }
            Selection::Help => {
                Menu::open(MenuType::HelpMenu);
            }
            _ => break,
        }
    }
}

fn play_file(filename: &Path) {
    loop {
        let mut level = match Level::file(filename) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };
        let result = level.play();
        match result {
            Ok(result) => {
                if result.has_won {
                    thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                    Menu::open(MenuType::Message("YAY, You Won!".to_string()));
                    break;
                } else if let Some(r) = result.reason_for_loss {
                    match r {
                        LevelLossReason::Zapper => {
                            thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                            Menu::open(MenuType::Message("Uh oh, you lit a zapper!".to_string()));
                        }
                        LevelLossReason::Death => {
                            thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                            Menu::open(MenuType::Message(
                                "Uh oh, you got shot by a laser beam!".to_string(),
                            ));
                        }
                        LevelLossReason::Quit => break,
                    }
                } else {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
