pub mod direction;
pub mod level;
//pub mod menu;
pub mod node;

use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand,
};
use level::*;
//use menu::*;
use std::{io::Stdout, thread, time};

/// Represents a level of `ascii-portal`.
pub struct L1t {}

impl L1t {
    pub fn new() -> L1t {
        L1t {}
    }

    pub fn play(
        &mut self,
        stdout: &mut Stdout,
        filename: String,
        term_rows: u16,
        term_cols: u16,
    ) -> Result<bool, &str> {
        enable_raw_mode().ok();
        stdout.execute(cursor::Hide).ok();
        stdout
            .execute(Clear(crossterm::terminal::ClearType::All))
            .ok();
        //let selection = Menu::draw(
        //    stdout,
        //    MenuType::MainSelection(
        //        vec![
        //            "             /-------L       ".to_string(),
        //            "    ___      |__      _      ".to_string(),
        //            "   |_  |  <--/  |    | \\_    ".to_string(),
        //            "     | |     `| |    | __|   ".to_string(),
        //            "     | |      | |    | |     ".to_string(),
        //            "     | |_    _|_|_   | |_    ".to_string(),
        //            "   --\\___\\  |_____| --\\__|   ".to_string(),
        //            "     |                |      ".to_string(),
        //            "     v                v      ".to_string(),
        //            "     S                       ".to_string(),
        //        ],
        //        vec!["PLAY".to_string(), "HELP".to_string(), "QUIT".to_string()],
        //    ),
        //    term_rows,
        //    term_cols,
        //)
        //.unwrap_or(0);
        let mut level = match Level::new(filename, term_rows, term_cols) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        let result = level.play();
        thread::sleep(time::Duration::from_millis(500));
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
        stdout.execute(cursor::Show).ok();
        disable_raw_mode().ok();
        Ok(true)
    }
}
