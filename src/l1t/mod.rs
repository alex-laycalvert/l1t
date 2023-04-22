pub mod level;
pub mod node;
pub mod direction;

use level::Level;
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
        level.play(stdout).ok();
        Ok(false)
    }
}
