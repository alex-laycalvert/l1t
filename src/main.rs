use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand,
};
use l1t::level::*;
use l1t::menu::*;
use std::io::stdout;
use std::{thread, time};

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().ok();
    stdout.execute(cursor::Hide).ok();
    let selection = Menu::draw(MenuType::MainSelection).unwrap_or(Selection::Play);
    match selection {
        Selection::Play => {
            let mut current_level = 1;
            loop {
                let filename = String::from(match current_level {
                    1 => "levels/1.l1t",
                    2 => "levels/2.l1t",
                    3 => "levels/3.l1t",
                    4 => "levels/4.l1t",
                    _ => "levels/0.l1t",
                });
                let mut level = match Level::new(filename) {
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
                            thread::sleep(time::Duration::from_millis(300));
                            Menu::draw(MenuType::Message("YAY, You Won!".to_string()));
                            current_level += 1;
                        } else if let Some(r) = result.reason_for_loss {
                            match r {
                                LevelLossReason::Zapper => {
                                    thread::sleep(time::Duration::from_millis(250));
                                    Menu::draw(MenuType::Message(
                                        "Uh oh, you lit a zapper!".to_string(),
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
        _ => (),
    }
    stdout.execute(cursor::Show).ok();
    disable_raw_mode().ok();
    stdout.execute(cursor::MoveTo(0, 0)).ok();
    stdout
        .execute(Clear(crossterm::terminal::ClearType::All))
        .ok();
}
