use crate::controls::Control;
use crate::{direction::Direction, menu::*, node::*};
use crossterm::{
    cursor, execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};
use human_sort::sort;
use std::fs;
use std::io::stdout;

#[derive(Debug)]
pub enum LevelLossReason {
    Zapper,
    Quit,
    Death,
}

#[derive(Debug, Clone)]
pub enum LevelSource {
    File(String),
    Url(String),
    Core(usize),
}

#[derive(Debug)]
pub struct Level {
    pub info: LevelInfo,
    pub nodes: Vec<Node>,
    pub rows: u16,
    pub cols: u16,
    pub player_index: Option<usize>,
}

#[derive(Debug)]
pub struct LevelInfo {
    pub source: LevelSource,
    pub name: String,
    pub author: String,
    pub description: String,
}

#[derive(Debug)]
pub struct LevelResult {
    pub has_won: bool,
    pub reason_for_loss: Option<LevelLossReason>,
}

#[derive(Debug)]
struct PlayState {
    is_playing: bool,
    has_won: bool,
    reason_for_loss: Option<LevelLossReason>,
}

impl Level {
    pub const NUM_CORE_LEVELS: usize = 4;
    pub const CORE_LEVELS: [&str; Level::NUM_CORE_LEVELS] = [
        "Level 1
alex-laycalvert
The First Level
IIIIIIIIIIIIIIIII
I               I
I\\           /  I
I               I
I1     X     S  I
I               I
IIIIIIIIIIIIIIIII",
        "Level 2
alex-laycalvert
Reverse Statues
IIIIIIIIIIIIIIIII
I               I
I           R   I
I               I
I     X         I
I               I
I4          /   I
IS          \\   I
IIIIIIIIIIIIIIIII",
        "Level 3
alex-laycalvert
Using Your Surroundings
IIIIIIIIIIIIIIIIIII
III               I
III  R            I
III               I
III               I
I4   /   B    X   I
III               I
III               I
III  R            I
III               I
IIIIIIIIIIIIIIIIIII",
        "Level 4
alex-laycalvert
What's That Special Block Over There?
IIIIIIIIIIIIIIIIIIIII
I              Z  \\ I
I                   I
I                   I
I    X    4    \\    I
I                   I
I                   I
I         S    \\  \\ I
IIIIIIIIIIIIIIIIIIIII",
    ];

    fn draw_walls(&self, row_offset: u16, col_offset: u16) -> crossterm::Result<()> {
        let mut stdout = stdout();
        for r in row_offset..(self.rows + row_offset) {
            for c in col_offset..(self.cols + col_offset) {
                if r == row_offset
                    || r == self.rows + row_offset - 1
                    || c == col_offset
                    || c == self.cols + col_offset - 1
                {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::White),
                        SetBackgroundColor(Color::White),
                        cursor::MoveTo(c, r),
                        Print('I'.bold()),
                    )?;
                }
            }
        }
        execute!(
            stdout,
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),
        )
    }

    fn draw_nodes(&self, row_offset: u16, col_offset: u16) -> crossterm::Result<()> {
        for i in 0..self.nodes.len() {
            self.nodes[i].draw((row_offset, col_offset))?;
        }
        Ok(())
    }

    fn draw_node_overlays(&self, row_offset: u16, col_offset: u16) -> crossterm::Result<()> {
        for i in 0..self.nodes.len() {
            self.nodes[i].draw_overlay((row_offset, col_offset))?;
        }
        Ok(())
    }

    fn draw(&self) -> crossterm::Result<()> {
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All))?;
        let (term_cols, term_rows) = size().unwrap_or((0, 0));
        let row_offset = (term_rows - self.rows) / 2;
        let col_offset = (term_cols - self.cols) / 2;
        self.draw_walls(row_offset, col_offset)?;
        self.draw_node_overlays(row_offset, col_offset)?;
        self.draw_nodes(row_offset, col_offset)?;
        Ok(())
    }

    fn set_lasers_shooting_at(&mut self) {
        for i in 0..self.nodes.len() {
            if let NodeType::Laser(l) = &self.nodes[i].node_type {
                if !l.on {
                    self.nodes[i].set_shooting_at(vec![]);
                    continue;
                }
                let mut shooting_at: Vec<(u16, u16, char, char)> = vec![];
                let mut current_row: i16 = self.nodes[i].row as i16;
                let mut current_col: i16 = self.nodes[i].col as i16;
                let mut current_dir: Direction = l.dir;
                loop {
                    if !self.is_valid_pos((current_row as u16, current_col as u16)) {
                        break;
                    }
                    current_row += current_dir.0;
                    current_col += current_dir.1;
                    shooting_at.push((
                        current_row as u16,
                        current_col as u16,
                        match current_dir {
                            Direction::UP | Direction::DOWN => '|',
                            _ => '-',
                        },
                        match current_dir {
                            Direction::UP => '^',
                            Direction::DOWN => 'v',
                            Direction::LEFT => '<',
                            _ => '>',
                        },
                    ));
                    if let Some(i) = self.node_index_at((current_row as u16, current_col as u16)) {
                        match &self.nodes[i].node_type {
                            NodeType::Mirror(m) => {
                                if current_dir.0 == 0 {
                                    current_dir.0 = current_dir.1.abs();
                                    if current_dir.1 == (m.dir.0 + m.dir.1) {
                                        current_dir.0 = -current_dir.0
                                    }
                                    current_dir.1 = 0;
                                } else {
                                    current_dir.1 = current_dir.0.abs();
                                    if current_dir.0 == (m.dir.0 + m.dir.1) {
                                        current_dir.1 = -current_dir.1
                                    }
                                    current_dir.0 = 0;
                                }
                            }
                            _ => {
                                if self.nodes[i].is_laser_toggleable() {
                                    if let NodeType::Laser(_) = &self.nodes[i].node_type {
                                        self.nodes[i].turn_off();
                                    } else {
                                        self.nodes[i].turn_on()
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
                self.nodes[i].set_shooting_at(shooting_at);
            }
        }
    }

    fn surrounding_nodes(&self, pos: (u16, u16)) -> Vec<usize> {
        let mut nodes: Vec<usize> = vec![];
        for i in 0..self.nodes.len() {
            let n = &self.nodes[i];
            if (n.row == pos.0 && (n.col == pos.1 - 1 || n.col == pos.1 + 1))
                || (n.col == pos.1 && (n.row == pos.0 - 1 || n.row == pos.0 + 1))
            {
                nodes.push(i);
            }
        }
        nodes
    }

    fn player_action(&mut self) {
        let player_index = match self.player_index {
            Some(i) => i,
            None => return,
        };
        let surrounding_nodes =
            &self.surrounding_nodes((self.nodes[player_index].row, self.nodes[player_index].col));
        for &i in surrounding_nodes.iter() {
            if !self.nodes[i].is_player_toggleable()
                || matches!(self.nodes[i].node_type, NodeType::Button(_))
            {
                continue;
            }
            self.nodes[i].toggle();
        }
    }

    fn node_index_at(&self, pos: (u16, u16)) -> Option<usize> {
        self.nodes
            .iter()
            .position(|n| n.row == pos.0 && n.col == pos.1)
    }

    fn is_valid_pos(&self, pos: (u16, u16)) -> bool {
        pos.0 >= 1 && pos.0 < self.rows - 1 && pos.1 >= 1 && pos.1 < self.cols - 1
    }

    fn move_player(&mut self, dir: Direction) {
        let player_index = match self.player_index {
            Some(i) => i,
            None => return,
        };
        let new_pos = self.nodes[player_index].would_move_to(dir);
        if !self.is_valid_pos(new_pos) {
            return;
        }
        if let Some(i) = self.node_index_at(new_pos) {
            if !self.nodes[i].is_moveable() {
                return;
            }
            let new_pos = self.nodes[i].would_move_to(dir);
            if !self.is_valid_pos(new_pos) {
                return;
            }
            if self.node_index_at(new_pos).is_some() {
                return;
            }
            self.nodes[i].move_in_dir(dir);
        }
        self.nodes[player_index].move_in_dir(dir);
    }

    fn reset_statues(&mut self) {
        for i in 0..self.nodes.len() {
            if let NodeType::Statue(_) = &self.nodes[i].node_type {
                self.nodes[i].turn_off();
            }
        }
    }

    fn play_state(&self) -> PlayState {
        let mut all_statues_lit = true;
        for i in 0..self.nodes.len() {
            match &self.nodes[i].node_type {
                NodeType::Statue(s) => {
                    if s.reversed {
                        all_statues_lit = all_statues_lit && !s.lit;
                    } else {
                        all_statues_lit = all_statues_lit && s.lit;
                    }
                }
                NodeType::Zapper(z) => {
                    if z.lit {
                        return PlayState {
                            is_playing: false,
                            has_won: false,
                            reason_for_loss: Some(LevelLossReason::Zapper),
                        };
                    }
                }
                NodeType::Player(p) => {
                    if p.dead {
                        return PlayState {
                            is_playing: false,
                            has_won: false,
                            reason_for_loss: Some(LevelLossReason::Death),
                        };
                    }
                }
                _ => (),
            }
        }
        if !all_statues_lit {
            return PlayState {
                is_playing: true,
                has_won: false,
                reason_for_loss: None,
            };
        }
        PlayState {
            is_playing: false,
            has_won: true,
            reason_for_loss: None,
        }
    }

    pub fn available_levels(level_dir: String) -> Result<Vec<LevelInfo>, String> {
        let files = match fs::read_dir(level_dir) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };
        let mut filenames = Vec::<String>::new();
        for f in files {
            let f = match f {
                Ok(f) => f,
                Err(e) => return Err(e.to_string()),
            };
            let path = f.path();
            let s = path.to_string_lossy();
            filenames.push(s.to_string());
        }
        let mut filenames: Vec<&str> = filenames.iter().map(|s| &**s).collect();
        sort(&mut filenames);
        let mut levels = Vec::<LevelInfo>::new();
        for f in filenames {
            let content = match fs::read_to_string(f) {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };
            let lines: Vec<&str> = content.split('\n').collect();
            levels.push(LevelInfo {
                source: LevelSource::File(f.to_string()),
                name: lines[0].to_string(),
                author: lines[1].to_string(),
                description: lines[2].to_string(),
            });
        }
        Ok(levels)
    }

    fn parse_grid(content: &[&str], info: LevelInfo) -> Result<Level, &'static str> {
        let rows = content.len() as u16;
        if rows < 3 {
            return Err("Level file must include a line for the `name`, `author`, `description`, and lines representing the level grid.");
        }
        let cols = content[0].len() as u16;
        if cols < 3 {
            return Err("Level grid must be made up of at least one grid space and an even wall of `I` characters representing the walls.");
        }
        let mut nodes: Vec<Node> = vec![];
        let mut player_index: Option<usize> = None;
        for r in 0..rows {
            for (c, ch) in content[r as usize].chars().enumerate() {
                if r == 0 || r == rows - 1 || c == 0 || c == cols as usize - 1 {
                    if ch != 'I' {
                        return Err("Level grid must be made up of at least one grid space and an even wall of `I` characters representing the walls.");
                    }
                    continue;
                }
                if ch == ' ' {
                    continue;
                }
                let node = Node::new(ch, r, c as u16);
                if matches!(node.node_type, NodeType::Player(_)) {
                    player_index = Some(nodes.len());
                }
                nodes.push(node);
            }
        }
        Ok(Level {
            info,
            nodes,
            rows,
            cols,
            player_index,
        })
    }

    fn parse_full(content: &[&str], source: LevelSource) -> Result<Level, &'static str> {
        if content.len() < 3 {
            return Err("Empty level file.");
        }
        let info = LevelInfo {
            source,
            name: content[0].to_string(),
            author: content[1].to_string(),
            description: content[2].to_string(),
        };
        Level::parse_grid(&content[3..], info)
    }

    pub fn file(filename: String) -> Result<Level, &'static str> {
        let content: String = fs::read_to_string(&filename).unwrap_or("".to_string());
        let content: Vec<&str> = content.trim().split('\n').collect();
        Level::parse_full(&content, LevelSource::File(filename))
    }

    pub fn url(_url: String) -> Result<Level, &'static str> {
        todo!()
    }

    pub fn core(level: usize) -> Result<Level, &'static str> {
        let content = Level::CORE_LEVELS[level];
        let content: Vec<&str> = content.trim().split('\n').collect();
        Level::parse_full(&content, LevelSource::Core(level))
    }

    pub fn play(&mut self) -> Result<LevelResult, &str> {
        loop {
            self.reset_statues();
            self.set_lasers_shooting_at();
            self.draw().ok();
            let state = self.play_state();
            if !state.is_playing {
                return Ok(LevelResult {
                    has_won: state.has_won,
                    reason_for_loss: state.reason_for_loss,
                });
            }
            match Control::read_input() {
                Control::Up => self.move_player(Direction::UP),
                Control::Down => self.move_player(Direction::DOWN),
                Control::Left => self.move_player(Direction::LEFT),
                Control::Right => self.move_player(Direction::RIGHT),
                Control::Action => self.player_action(),
                Control::Help => {
                    Menu::open(MenuType::HelpMenu);
                }
                Control::Quit => {
                    if let Some(Selection::Yes) = Menu::open(MenuType::YesNoSelection(
                        "Are you sure you want to quit?".to_string(),
                    )) {
                        return Ok(LevelResult {
                            has_won: false,
                            reason_for_loss: Some(LevelLossReason::Quit),
                        });
                    }
                }
                _ => (),
            }
        }
    }
}
