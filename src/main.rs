use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear},
    ExecutableCommand,
};
use l1t::level::*;
use l1t::menu::*;
use std::io::stdout;
use std::{thread, time};

fn main() {
    let mut stdout = stdout();
    let (cols, rows) = size().unwrap_or((0, 0));
    let filename = String::from("test_level.l1t");
    let mut level = match Level::new(filename, rows, cols) {
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
    let selection = Menu::draw(MenuType::MainSelection, rows, cols).unwrap_or(Selection::Play);
    match selection {
        Selection::Play => {
            let result = level.play();
            thread::sleep(time::Duration::from_millis(500));
            stdout.execute(cursor::Show).ok();
            disable_raw_mode().ok();
            stdout
                .execute(Clear(crossterm::terminal::ClearType::All))
                .ok();
            match result {
                Ok(result) => {
                    if result.has_won {
                        println!("YAY, You Won!");
                    } else if let Some(r) = result.reason_for_loss {
                        match r {
                            LevelLossReason::Zapper => {
                                println!("Uh oh, you lit a zapper!");
                            }
                            LevelLossReason::Quit => {
                                //Menu::draw(
                                //    stdout,
                                //    MenuType::Message("Hello, World!".to_string()),
                                //    term_rows,
                                //    term_cols,
                                //);
                            }
                        }
                    } else {
                        println!("See you later!");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Selection::Help => {
            stdout.execute(cursor::Show).ok();
            disable_raw_mode().ok();
            stdout
                .execute(Clear(crossterm::terminal::ClearType::All))
                .ok();
            println!("Coming soon...");
        }
        Selection::Quit => {
            stdout.execute(cursor::Show).ok();
            disable_raw_mode().ok();
            stdout
                .execute(Clear(crossterm::terminal::ClearType::All))
                .ok();
        }
    }
}
