use clap::Parser;
use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use home::home_dir;
use l1t::level::*;
use l1t::menu::*;
//use l1t::repository::*;
use l1t::userdata::*;
use std::{
    error::Error,
    io::stdout,
    path::{Path, PathBuf},
    thread, time,
};

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

enum PlayStatus<'a> {
    WonLevel,
    Quit,
    LostLevel,
    Error(&'a str),
}

fn setup() -> crossterm::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)
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
    setup().ok();
    if let Some(filename) = &args.file {
        return play_file(filename);
    }

    let home = match home_dir() {
        Some(h) => h,
        None => return exit(Some("failed to find user's home directory")),
    };
    let home = home.to_str().unwrap_or("");
    let user_data = match UserData::read(home.to_string()) {
        Ok(d) => d,
        Err(e) => return exit(Some(&e)),
    };

    play(user_data).await
}

async fn play(mut user_data: UserData) -> Result<(), Box<dyn Error>> {
    loop {
        let selection = Menu::open(MenuType::MainSelection(&user_data.completed_core_levels))
            .unwrap_or(Selection::Play(LevelSource::Core(0)));
        match selection {
            Selection::Play(level_source) => match level_source {
                LevelSource::Core(level) => {
                    let mut current_level = level;
                    loop {
                        if current_level >= Level::NUM_CORE_LEVELS {
                            Menu::open(MenuType::Message(
                                "You've completed all core levels, thanks for playing!",
                            ));
                            break;
                        }
                        let mut level = match Level::core(current_level) {
                            Ok(l) => l,
                            Err(e) => return exit(Some(e)),
                        };
                        let result = level.play();
                        match handle_level_result(result) {
                            PlayStatus::WonLevel => {
                                if let Err(e) = user_data.complete(level.info) {
                                    return exit(Some(&e));
                                };
                                current_level += 1;
                            }
                            PlayStatus::LostLevel => continue,
                            PlayStatus::Quit => break,
                            PlayStatus::Error(e) => return exit(Some(e)),
                        }
                    }
                }
                LevelSource::File(_) => {}
                LevelSource::Url(_url) => {}
            },
            Selection::Repository => loop {
                if let Some(Selection::Item(i)) =
                    Menu::open(MenuType::RepositorySelection(&user_data.repositories))
                {
                    if let Err(e) = user_data.repositories[i].download_listing().await {
                        Menu::open(MenuType::Message(&e.to_string()));
                        continue;
                    };
                    loop {
                        if let Some(selection) = Menu::open(MenuType::RepositoryLevelSelection(
                            user_data.repositories[i].clone(),
                            &user_data.completed_levels,
                        )) {
                            match selection {
                                Selection::Item(j) => {
                                    let level_info = &user_data.repositories[i].levels[j];
                                    loop {
                                        let mut level = match Level::url(level_info.clone()).await {
                                            Ok(l) => l,
                                            Err(e) => return exit(Some(e)),
                                        };
                                        let result = level.play();
                                        match handle_level_result(result) {
                                            PlayStatus::WonLevel => {
                                                if let Err(e) = user_data.complete(level.info) {
                                                    return exit(Some(&e));
                                                };
                                                break;
                                            }
                                            PlayStatus::LostLevel => continue,
                                            PlayStatus::Quit => break,
                                            PlayStatus::Error(e) => return exit(Some(e)),
                                        }
                                    }
                                }
                                Selection::Quit => break,
                                _ => continue,
                            }
                        }
                    }
                } else {
                    break;
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

fn play_file(filename: &Path) -> Result<(), Box<dyn Error>> {
    loop {
        let mut level = match Level::file(filename.to_path_buf()) {
            Ok(l) => l,
            Err(e) => return exit(Some(e)),
        };
        let result = level.play();
        match handle_level_result(result) {
            PlayStatus::WonLevel | PlayStatus::Quit => break,
            PlayStatus::LostLevel => continue,
            PlayStatus::Error(e) => return exit(Some(e)),
        }
    }
    exit(None)
}

fn handle_level_result(result: Result<LevelResult, &str>) -> PlayStatus {
    match result {
        Ok(result) => {
            if result.has_won {
                thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                Menu::open(MenuType::Message("YAY, You Won!"));
                PlayStatus::WonLevel
            } else if let Some(r) = result.reason_for_loss {
                match r {
                    LevelLossReason::Zapper => {
                        thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                        Menu::open(MenuType::Message("Uh oh, you lit a zapper!"));
                        PlayStatus::LostLevel
                    }
                    LevelLossReason::Death => {
                        thread::sleep(time::Duration::from_millis(SLEEP_TIME));
                        Menu::open(MenuType::Message("Uh oh, you got shot by a laser beam!"));
                        PlayStatus::LostLevel
                    }
                    LevelLossReason::Quit => PlayStatus::Quit,
                }
            } else {
                PlayStatus::Quit
            }
        }
        Err(e) => PlayStatus::Error(e),
    }
}
