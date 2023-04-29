use crossterm::event::{read, Event, KeyCode, KeyEventKind};

pub enum Control {
    Up,
    Down,
    Left,
    Right,
    Help,
    Quit,
    Action,
    Select,
    GotoTop,
    GotoBottom,
    None,
}

impl Control {
    pub fn read_input() -> Self {
        if let Ok(Event::Key(event)) = read() {
            if event.kind == KeyEventKind::Release {
                return Self::None;
            }
            return match event.code {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => Self::Up,
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => Self::Down,
                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => Self::Left,
                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => Self::Right,
                KeyCode::Char('g') => Self::GotoTop,
                KeyCode::Char('G') => Self::GotoBottom,
                KeyCode::Char(' ') => Self::Action,
                KeyCode::Char('H') => Self::Help,
                KeyCode::Char('q') => Self::Quit,
                KeyCode::Enter => Self::Select,
                _ => Self::None,
            };
        }
        Self::None
    }
}
