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
    let filename = String::from("test_level.l1t");
    let mut level = match Level::new(filename) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    stdout
        .execute(Clear(crossterm::terminal::ClearType::All))
        .ok();
    enable_raw_mode().ok();
    stdout.execute(cursor::Hide).ok();
    let selection = Menu::draw(MenuType::MainSelection).unwrap_or(Selection::Play);
    match selection {
        Selection::Play => {
            let result = level.play();
            match result {
                Ok(result) => {
                    if result.has_won {
                        thread::sleep(time::Duration::from_millis(250));
                        Menu::draw(MenuType::Message("YAY, You Won!".to_string()));
                    } else if let Some(r) = result.reason_for_loss {
                        match r {
                            LevelLossReason::Zapper => {
                                thread::sleep(time::Duration::from_millis(250));
                                Menu::draw(MenuType::Message(
                                    "Uh oh, you lit a zapper!".to_string(),
                                ));
                            }
                            _ => (),
                        }
                    } else {
                        println!("See you later!");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Selection::Help => {
            println!("Coming soon...");
        }
        Selection::Quit => {}
        _ => (),
    }
    stdout.execute(cursor::Show).ok();
    disable_raw_mode().ok();
    stdout.execute(cursor::MoveTo(0, 0)).ok();
    stdout
        .execute(Clear(crossterm::terminal::ClearType::All))
        .ok();
}
