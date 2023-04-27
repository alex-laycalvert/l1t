use clap::Parser;
use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use home::home_dir;
use l1t::level::*;
use l1t::menu::*;
//use l1t::repository::*;
use l1t::userdata::*;
use std::{error::Error, io::stdout, path::PathBuf, thread, time};

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

fn exit(error: Option<&str>) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        stdout(),
        cursor::Show,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All)
    )?;
    if let Some(e) = error {
        eprintln!("Error: {e}");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let repo = Repository::new(
    //    "My Repo".to_string(),
    //    "http://localhost:8000/l1t".to_string(),
    //)
    //.await
    //.ok();

    let args = Args::parse();
    let mut stdout = stdout();
    enable_raw_mode().ok();
    stdout.execute(cursor::Hide).ok();
    if let Some(filename) = &args.file {
        // File has been provided
        loop {
            let mut level = match Level::file(filename.to_path_buf()) {
                Ok(l) => l,
                Err(e) => return exit(Some(e)),
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
                Err(e) => return exit(Some(e)),
            }
        }
        return exit(None);
    }

    let home = match home_dir() {
        Some(h) => h,
        None => return exit(Some("failed to find user's home directory")),
    };
    let home = home.to_str().unwrap_or("");
    let mut user_data = match UserData::read(home.to_string()) {
        Ok(d) => d,
        Err(e) => return exit(Some(&e)),
    };

    loop {
        let selection = Menu::open(MenuType::MainSelection(
            user_data.completed_core_levels.clone(),
        ))
        .unwrap_or(Selection::Play(LevelSource::Core(0)));
        match selection {
            Selection::Play(level_source) => match level_source {
                LevelSource::Core(level) => {
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
                            Err(e) => return exit(Some(e)),
                        };
                        let result = level.play();
                        match result {
                            Ok(result) => {
                                if result.has_won {
                                    thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                                    Menu::open(MenuType::Message("YAY, You Won!".to_string()));
                                    current_level += 1;
                                    if let Err(e) = user_data.complete(level.info) {
                                        return exit(Some(&e));
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
                            Err(e) => return exit(Some(e)),
                        }
                    }
                }
                LevelSource::File(_) => {}
                LevelSource::Url(url) => {
                    let mut level = match Level::url(url) {
                        Ok(l) => l,
                        Err(e) => return exit(Some(e)),
                    };
                    let result = level.play();
                    match result {
                        Ok(result) => {
                            if result.has_won {
                                thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                                Menu::open(MenuType::Message("YAY, You Won!".to_string()));
                                if let Err(e) = user_data.complete(level.info) {
                                    return exit(Some(&e));
                                };
                                break;
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
                        Err(e) => return exit(Some(e)),
                    }
                }
            },
            Selection::Help => {
                Menu::open(MenuType::HelpMenu);
            }
            _ => break,
        }
    }
    exit(None)
}
