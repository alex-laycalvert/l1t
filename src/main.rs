mod l1t;

use crossterm::terminal::size;
use l1t::*;
use std::io::stdout;

fn main() {
    let mut stdout = stdout();
    let (cols, rows) = size().unwrap_or((0, 0));
    let mut game = L1t::new();
    game.play(&mut stdout, "test_level.l1t".to_string(), rows, cols)
        .expect("Failed to load level.");
}
