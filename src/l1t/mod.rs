pub mod direction;
pub mod level;
pub mod node;

use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use level::*;
use std::io::Stdout;

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
        let mut level = match Level::new(filename, term_rows, term_cols) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        enable_raw_mode().ok();
        stdout.execute(cursor::Hide).ok();
        let result = level.play(stdout);
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
                            println!("See you later!");
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
