use crate::direction::Direction;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
};
use std::io::stdout;

#[derive(Debug)]
pub struct Player {
    pub dead: bool,
}
#[derive(Debug)]
pub struct Block;
#[derive(Debug)]
pub struct Wall;
#[derive(Debug)]
pub struct Switch {
    pub on: bool,
}
#[derive(Debug)]
pub struct ToggleBlock {
    pub visible: bool,
}
#[derive(Debug)]
pub struct Button {
    pub pressed: bool,
}
#[derive(Debug)]
pub struct Mirror {
    pub dir: Direction,
}
#[derive(Debug)]
pub struct Laser {
    pub on: bool,
    pub dir: Direction,
    pub shooting_at: Vec<(u16, u16, char, char)>,
}
#[derive(Debug)]
pub struct Statue {
    pub lit: bool,
    pub reversed: bool,
}
#[derive(Debug)]
pub struct Zapper {
    pub lit: bool,
}

#[derive(Debug)]
pub enum NodeType {
    Player(Player),
    Block(Block),
    Wall(Wall),
    Switch(Switch),
    ToggleBlock(ToggleBlock),
    Button(Button),
    Mirror(Mirror),
    Laser(Laser),
    Statue(Statue),
    Zapper(Zapper),
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub row: u16,
    pub col: u16,
    moveable: bool,
}

const RED: Color = Color::Rgb { r: 255, g: 0, b: 0 };
const DIM_RED: Color = Color::Rgb { r: 100, g: 0, b: 0 };
const YELLOW: Color = Color::Rgb {
    r: 255,
    g: 255,
    b: 0,
};
const DIM_YELLOW: Color = Color::Rgb {
    r: 100,
    g: 100,
    b: 0,
};

impl Node {
    pub fn new(ch: char, row: u16, col: u16) -> Node {
        match ch {
            'X' => Node {
                row,
                col,
                node_type: NodeType::Player(Player { dead: false }),
                moveable: true,
            },
            'B' => Node {
                row,
                col,
                node_type: NodeType::Block(Block),
                moveable: true,
            },
            'T' => Node {
                row,
                col,
                node_type: NodeType::ToggleBlock(ToggleBlock { visible: true }),
                moveable: false,
            },
            'b' => Node {
                row,
                col,
                node_type: NodeType::Button(Button { pressed: false }),
                moveable: false,
            },
            's' => Node {
                row,
                col,
                node_type: NodeType::Switch(Switch { on: false }),
                moveable: false,
            },
            'S' => Node {
                row,
                col,
                node_type: NodeType::Statue(Statue {
                    lit: false,
                    reversed: false,
                }),
                moveable: false,
            },
            'R' => Node {
                row,
                col,
                node_type: NodeType::Statue(Statue {
                    lit: false,
                    reversed: true,
                }),
                moveable: false,
            },
            'Z' => Node {
                row,
                col,
                node_type: NodeType::Zapper(Zapper { lit: false }),
                moveable: false,
            },
            '/' => Node {
                row,
                col,
                node_type: NodeType::Mirror(Mirror {
                    dir: Direction::FORWARD,
                }),
                moveable: false,
            },
            '\\' => Node {
                row,
                col,
                node_type: NodeType::Mirror(Mirror {
                    dir: Direction::BACKWARD,
                }),
                moveable: false,
            },
            '?' => Node {
                row,
                col,
                node_type: NodeType::Mirror(Mirror {
                    dir: Direction::FORWARD,
                }),
                moveable: true,
            },
            '|' => Node {
                row,
                col,
                node_type: NodeType::Mirror(Mirror {
                    dir: Direction::BACKWARD,
                }),
                moveable: true,
            },
            '1' => Node {
                row,
                col,
                node_type: NodeType::Laser(Laser {
                    on: true,
                    dir: Direction::UP,
                    shooting_at: vec![],
                }),
                moveable: false,
            },
            '2' => Node {
                row,
                col,
                node_type: NodeType::Laser(Laser {
                    on: true,
                    dir: Direction::DOWN,
                    shooting_at: vec![],
                }),
                moveable: false,
            },
            '3' => Node {
                row,
                col,
                node_type: NodeType::Laser(Laser {
                    on: true,
                    dir: Direction::LEFT,
                    shooting_at: vec![],
                }),
                moveable: false,
            },
            '4' => Node {
                row,
                col,
                node_type: NodeType::Laser(Laser {
                    on: true,
                    dir: Direction::RIGHT,
                    shooting_at: vec![],
                }),
                moveable: false,
            },
            '5' => Node {
                row,
                col,
                node_type: NodeType::Laser(Laser {
                    on: false,
                    dir: Direction::UP,
                    shooting_at: vec![],
                }),
                moveable: false,
            },
            '6' => Node {
                row,
                col,
                node_type: NodeType::Laser(Laser {
                    on: false,
                    dir: Direction::DOWN,
                    shooting_at: vec![],
                }),
                moveable: false,
            },
            '7' => Node {
                row,
                col,
                node_type: NodeType::Laser(Laser {
                    on: false,
                    dir: Direction::LEFT,
                    shooting_at: vec![],
                }),
                moveable: false,
            },
            '8' => Node {
                row,
                col,
                node_type: NodeType::Laser(Laser {
                    on: false,
                    dir: Direction::RIGHT,
                    shooting_at: vec![],
                }),
                moveable: false,
            },
            _ => Node {
                row,
                col,
                node_type: NodeType::Wall(Wall),
                moveable: false,
            },
        }
    }

    pub fn draw_overlay(&self, offset: (u16, u16)) -> crossterm::Result<()> {
        let mut stdout = stdout();
        match &self.node_type {
            NodeType::Laser(l) => {
                if l.shooting_at.len() == 0 {
                    return Ok(());
                }
                for i in 0..(l.shooting_at.len() - 1) {
                    let pos = l.shooting_at[i];
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Rgb { r: 255, g: 0, b: 0 }),
                        MoveTo(pos.1 + offset.1, pos.0 + offset.0),
                    )?;
                    if i == l.shooting_at.len() - 2 {
                        execute!(stdout, Print(pos.3.bold()),)?;
                    } else {
                        execute!(stdout, Print(pos.2.bold()),)?;
                    }
                }
            }
            _ => (),
        };
        execute!(stdout, ResetColor)
    }

    pub fn draw(&self, offset: (u16, u16)) -> crossterm::Result<()> {
        let mut stdout = stdout();
        match &self.node_type {
            NodeType::Player(p) => execute!(
                stdout,
                SetForegroundColor(if p.dead { RED } else { Color::Green }),
                SetBackgroundColor(Color::Green),
                MoveTo(self.col + offset.1, self.row + offset.0),
                Print("X".bold()),
            ),
            NodeType::Block(_) => execute!(
                stdout,
                SetForegroundColor(Color::Grey),
                SetBackgroundColor(Color::Grey),
                MoveTo(self.col + offset.1, self.row + offset.0),
                Print("B".bold()),
            ),
            NodeType::Wall(_) => execute!(
                stdout,
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::White),
                MoveTo(self.col + offset.1, self.row + offset.0),
                Print("I".bold()),
            ),
            NodeType::Switch(s) => execute!(
                stdout,
                SetForegroundColor(Color::Black),
                SetBackgroundColor(if s.on { Color::Yellow } else { Color::Red }),
                MoveTo(self.col + offset.1, self.row + offset.0),
                Print("s".bold()),
            ),
            NodeType::ToggleBlock(t) => {
                if t.visible {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Magenta),
                        SetBackgroundColor(Color::Magenta),
                        MoveTo(self.col + offset.1, self.row + offset.0),
                        Print("T".bold())
                    )
                } else {
                    Ok(())
                }
            }
            NodeType::Button(b) => execute!(
                stdout,
                SetForegroundColor(Color::Black),
                SetBackgroundColor(if b.pressed { Color::Yellow } else { Color::Red }),
                MoveTo(self.col + offset.1, self.row + offset.0),
                Print("b".bold()),
            ),
            NodeType::Mirror(m) => execute!(
                stdout,
                SetForegroundColor(if self.moveable {
                    Color::Black
                } else {
                    Color::White
                }),
                SetBackgroundColor(if self.moveable {
                    Color::White
                } else {
                    Color::Reset
                }),
                MoveTo(self.col + offset.1, self.row + offset.0),
                Print(if matches!(m.dir, Direction::FORWARD) {
                    "/".bold()
                } else {
                    "\\".bold()
                }),
            ),
            NodeType::Laser(l) => execute!(
                stdout,
                SetForegroundColor(if l.on { RED } else { DIM_RED }),
                SetBackgroundColor(if l.on { RED } else { DIM_RED }),
                MoveTo(self.col + offset.1, self.row + offset.0),
                Print("L".bold()),
            ),
            NodeType::Statue(s) => {
                if s.reversed {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Black),
                        SetBackgroundColor(if s.lit { DIM_YELLOW } else { YELLOW }),
                        MoveTo(self.col + offset.1, self.row + offset.0),
                        Print("R".bold()),
                    )
                } else {
                    execute!(
                        stdout,
                        SetForegroundColor(if s.lit { YELLOW } else { DIM_YELLOW }),
                        SetBackgroundColor(if s.lit { YELLOW } else { DIM_YELLOW }),
                        MoveTo(self.col + offset.1, self.row + offset.0),
                        Print("S".bold()),
                    )
                }
            }
            NodeType::Zapper(z) => execute!(
                stdout,
                SetForegroundColor(if z.lit { Color::Black } else { Color::Yellow }),
                SetBackgroundColor(if z.lit { Color::Yellow } else { Color::Black }),
                MoveTo(self.col + offset.1, self.row + offset.0),
                Print("Z".bold()),
            ),
        }?;
        execute!(stdout, ResetColor)
    }

    pub fn would_move_to(&mut self, dir: Direction) -> (u16, u16) {
        if !self.moveable {
            return (self.row, self.col);
        }
        let mut row = self.row as i16;
        let mut col = self.col as i16;
        if self.row as i16 + dir.0 >= 0 {
            row = self.row as i16 + dir.0;
        }
        if self.col as i16 + dir.1 >= 0 {
            col = self.col as i16 + dir.1;
        }
        (row as u16, col as u16)
    }

    pub fn move_in_dir(&mut self, dir: Direction) {
        if !self.moveable {
            return;
        }
        if self.row as i16 + dir.0 >= 0 {
            self.row = (self.row as i16 + dir.0) as u16
        }
        if self.col as i16 + dir.1 >= 0 {
            self.col = (self.col as i16 + dir.1) as u16
        }
    }

    pub fn is_moveable(&self) -> bool {
        self.moveable
    }

    pub fn is_player_toggleable(&self) -> bool {
        match &self.node_type {
            NodeType::Laser(_) => true,
            NodeType::Mirror(_) => true,
            NodeType::Switch(_) => true,
            _ => false,
        }
    }

    pub fn is_laser_toggleable(&self) -> bool {
        match &self.node_type {
            NodeType::Player(_) => true,
            NodeType::Laser(_) => true,
            NodeType::Statue(_) => true,
            NodeType::Zapper(_) => true,
            _ => false,
        }
    }

    pub fn turn_on(&mut self) {
        match &mut self.node_type {
            NodeType::Player(p) => p.dead = true,
            NodeType::Laser(l) => l.on = true,
            NodeType::Statue(s) => s.lit = true,
            NodeType::Zapper(z) => z.lit = true,
            NodeType::Button(b) => b.pressed = true,
            NodeType::Switch(s) => s.on = true,
            NodeType::ToggleBlock(t) => t.visible = true,
            _ => (),
        }
    }

    pub fn turn_off(&mut self) {
        match &mut self.node_type {
            NodeType::Player(p) => p.dead = false,
            NodeType::Laser(l) => l.on = false,
            NodeType::Statue(s) => s.lit = false,
            NodeType::Zapper(z) => z.lit = false,
            NodeType::Button(b) => b.pressed = false,
            NodeType::Switch(s) => s.on = false,
            NodeType::ToggleBlock(t) => t.visible = false,
            _ => (),
        }
    }

    pub fn toggle(&mut self) {
        match &mut self.node_type {
            NodeType::Player(p) => p.dead = !p.dead,
            NodeType::Laser(l) => l.on = !l.on,
            NodeType::Statue(s) => s.lit = !s.lit,
            NodeType::Zapper(z) => z.lit = !z.lit,
            NodeType::Mirror(m) => {
                if matches!(m.dir, Direction::FORWARD) {
                    m.dir = Direction::BACKWARD;
                } else {
                    m.dir = Direction::FORWARD;
                }
            }
            NodeType::Button(b) => b.pressed = !b.pressed,
            NodeType::Switch(s) => s.on = !s.on,
            NodeType::ToggleBlock(t) => t.visible = !t.visible,
            _ => (),
        }
    }

    pub fn set_shooting_at(&mut self, shooting_at: Vec<(u16, u16, char, char)>) {
        match &mut self.node_type {
            NodeType::Laser(l) => l.shooting_at = shooting_at,
            _ => (),
        }
    }
}
