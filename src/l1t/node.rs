use crate::direction::Direction;
use crossterm::style::Color;

#[derive(Clone, Debug)]
pub enum NodeType {
    Player,
    Block,
    Wall,
    Switch,
    ToggleBlock,
    Button,
    Mirror(Direction),
    Laser(Direction),
}

impl NodeType {
    pub fn from(ch: char) -> Option<NodeType> {
        match ch {
            'X' => Some(NodeType::Player),
            'K' => Some(NodeType::Block),
            'I' => Some(NodeType::Wall),
            'S' => Some(NodeType::Switch),
            'B' => Some(NodeType::Button),
            'T' => Some(NodeType::ToggleBlock),
            '/' => Some(NodeType::Mirror(Direction::RIGHT)),
            '\\' => Some(NodeType::Mirror(Direction::LEFT)),
            '1' => Some(NodeType::Laser(Direction::UP)),
            '2' => Some(NodeType::Laser(Direction::DOWN)),
            '3' => Some(NodeType::Laser(Direction::LEFT)),
            '4' => Some(NodeType::Laser(Direction::RIGHT)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub col: u16,
    pub row: u16,
    pub ch: char,
    pub fg_color: Color,
    pub bg_color: Color,
    pub toggled_fg_color: Color,
    pub toggled_bg_color: Color,
    pub dir: Direction,
    pub toggled: bool,
    pub looking_at: Vec<(u16, u16, char, char)>,
}

impl Node {
    pub fn parse(ch: char, row: u16, col: u16) -> Option<Node> {
        match NodeType::from(ch) {
            Some(t) => Some(Node::new(t, row, col)),
            None => None,
        }
    }

    pub fn new(node_type: NodeType, row: u16, col: u16) -> Node {
        let mut dir = Direction::UP;
        let (ch, fg_color, bg_color, toggled_fg_color, toggled_bg_color) = match node_type {
            NodeType::Player => ('X', Color::Green, Color::Green, Color::Green, Color::Green),
            NodeType::Block => ('K', Color::Grey, Color::Grey, Color::Grey, Color::Grey),
            NodeType::Wall => ('I', Color::White, Color::White, Color::White, Color::White),
            NodeType::Switch => ('S', Color::Yellow, Color::Red, Color::Red, Color::Yellow),
            NodeType::Button => ('B', Color::Yellow, Color::Red, Color::Red, Color::Yellow),
            NodeType::ToggleBlock => (
                'T',
                Color::Magenta,
                Color::Magenta,
                Color::Magenta,
                Color::Magenta,
            ),
            NodeType::Mirror(d) => {
                dir = d;
                (
                    if matches!(d, Direction::RIGHT) {
                        '/'
                    } else {
                        '\\'
                    },
                    Color::White,
                    Color::Reset,
                    Color::White,
                    Color::Reset,
                )
            }
            NodeType::Laser(d) => {
                dir = d;
                ('L', Color::Red, Color::Red, Color::Red, Color::Red)
            }
        };

        Node {
            node_type,
            row,
            col,
            ch,
            fg_color,
            bg_color,
            toggled_fg_color,
            toggled_bg_color,
            dir,
            toggled: false,
            looking_at: vec![],
        }
    }
}
