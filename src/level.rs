use crate::{direction::Direction, menu::*, node::*};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::fs;
use std::io::stdout;

pub enum LevelLossReason {
    Zapper,
    Quit,
}

pub struct Level {
    pub name: String,
    pub author: String,
    pub description: String,
    pub nodes: Vec<Node>,
    pub rows: u16,
    pub cols: u16,
    pub term_rows: u16,
    pub term_cols: u16,
    pub row_offset: u16,
    pub col_offset: u16,
    pub player_index: Option<usize>,
}

pub struct LevelResult {
    pub has_won: bool,
    pub reason_for_loss: Option<LevelLossReason>,
}

struct PlayState {
    is_playing: bool,
    has_won: bool,
    reason_for_loss: Option<LevelLossReason>,
}

impl Level {
    fn draw_walls(&self) -> crossterm::Result<()> {
        let mut stdout = stdout();
        for r in self.row_offset..(self.rows + self.row_offset) {
            for c in self.col_offset..(self.cols + self.col_offset) {
                if r == self.row_offset
                    || r == self.rows + self.row_offset - 1
                    || c == self.col_offset
                    || c == self.cols + self.col_offset - 1
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

    fn draw_nodes(&self) -> crossterm::Result<()> {
        for i in 0..self.nodes.len() {
            self.nodes[i].draw((self.row_offset, self.col_offset))?;
        }
        Ok(())
    }

    fn draw_node_overlays(&self) -> crossterm::Result<()> {
        for i in 0..self.nodes.len() {
            self.nodes[i].draw_overlay((self.row_offset, self.col_offset))?;
        }
        Ok(())
    }

    fn draw(&self) -> crossterm::Result<()> {
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All))?;
        self.draw_walls()?;
        self.draw_node_overlays()?;
        self.draw_nodes()?;
        Ok(())
    }

    fn set_lasers_shooting_at(&mut self) {
        for i in 0..self.nodes.len() {
            match &self.nodes[i].node_type {
                NodeType::Laser(l) => {
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
                        if let Some(i) =
                            self.get_node_index_at((current_row as u16, current_col as u16))
                        {
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
                                            self.nodes[i].toggle()
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                    }
                    self.nodes[i].set_shooting_at(shooting_at);
                }
                _ => (),
            }
        }
    }

    fn get_surrounding_nodes(&self, pos: (u16, u16)) -> Vec<usize> {
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
        let surrounding_nodes = &self
            .get_surrounding_nodes((self.nodes[player_index].row, self.nodes[player_index].col));
        for &i in surrounding_nodes.iter() {
            if !self.nodes[i].is_player_toggleable()
                || matches!(self.nodes[i].node_type, NodeType::Button(_))
            {
                continue;
            }
            self.nodes[i].toggle();
        }
    }

    fn get_node_index_at(&self, pos: (u16, u16)) -> Option<usize> {
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
        if let Some(i) = self.get_node_index_at(new_pos) {
            if !self.nodes[i].is_moveable() {
                return;
            }
            let new_pos = self.nodes[i].would_move_to(dir);
            if !self.is_valid_pos(new_pos) {
                return;
            }
            if self.get_node_index_at(new_pos) != None {
                return;
            }
            self.nodes[i].move_in_dir(dir);
        }
        self.nodes[player_index].move_in_dir(dir);
    }

    fn get_play_state(&self) -> PlayState {
        let mut all_statues_lit = true;
        for i in 0..self.nodes.len() {
            match &self.nodes[i].node_type {
                NodeType::Statue(s) => {
                    if !s.lit {
                        all_statues_lit = false;
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
                _ => (),
            }
        }
        if all_statues_lit {
            return PlayState {
                is_playing: false,
                has_won: true,
                reason_for_loss: None,
            };
        }
        PlayState {
            is_playing: true,
            has_won: false,
            reason_for_loss: None,
        }
    }

    pub fn new(filename: String, term_rows: u16, term_cols: u16) -> Result<Level, &'static str> {
        let file_content = fs::read_to_string(filename).unwrap_or("".to_string());
        if file_content.trim().len() == 0 {
            return Err("Empty level file.");
        }
        let lines: Vec<&str> = file_content.trim().split('\n').collect();
        let rows = lines.len() as u16;
        if rows < 6 {
            return Err("Level file must include a line for the `name`, `author`, `description`, and lines representing the level grid.");
        }
        let cols = lines[4].len() as u16;
        if cols < 3 {
            return Err("Level grid must be made up of at least one grid space and an even wall of `I` characters representing the walls.");
        }
        let name: String = lines[0].to_string();
        let author: String = lines[1].to_string();
        let description: String = lines[2].to_string();
        let mut nodes: Vec<Node> = vec![];
        let mut player_index: Option<usize> = None;
        for r in 3..rows {
            for (c, ch) in lines[r as usize].chars().enumerate() {
                if r == 3 || r == rows - 1 || c == 0 || c == cols as usize - 1 {
                    if ch != 'I' {
                        return Err("Level grid must be made up of at least one grid space and an even wall of `I` characters representing the walls.");
                    }
                    continue;
                }
                if ch == ' ' {
                    continue;
                }
                let node = Node::new(ch, r - 3, c as u16);
                if matches!(node.node_type, NodeType::Player(_)) {
                    player_index = Some(nodes.len());
                }
                nodes.push(node);
            }
        }
        Ok(Level {
            name,
            author,
            description,
            nodes,
            rows: rows - 3,
            cols,
            term_rows,
            term_cols,
            player_index,
            row_offset: (term_rows - rows - 3) / 2,
            col_offset: (term_cols - cols) / 2,
        })
    }

    pub fn play(&mut self) -> Result<LevelResult, &str> {
        loop {
            self.set_lasers_shooting_at();
            self.draw().ok();
            let state = self.get_play_state();
            if !state.is_playing {
                return Ok(LevelResult {
                    has_won: state.has_won,
                    reason_for_loss: state.reason_for_loss,
                });
            }
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char('w') => self.move_player(Direction::UP),
                    KeyCode::Char('s') => self.move_player(Direction::DOWN),
                    KeyCode::Char('a') => self.move_player(Direction::LEFT),
                    KeyCode::Char('d') => self.move_player(Direction::RIGHT),
                    KeyCode::Char(' ') => self.player_action(),
                    KeyCode::Char('q') => {
                        if let Some(s) = Menu::draw(
                            MenuType::YesNoSelection("Are you sure you want to quit?".to_string()),
                            self.term_rows,
                            self.term_cols,
                        ) {
                            match s {
                                Selection::Yes => {
                                    return Ok(LevelResult {
                                        has_won: false,
                                        reason_for_loss: Some(LevelLossReason::Quit),
                                    })
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
