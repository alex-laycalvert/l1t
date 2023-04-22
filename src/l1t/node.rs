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
    Laser(Direction, bool),
    Statue,
    Zapper,
}

impl NodeType {
    pub fn from(ch: char) -> Option<NodeType> {
        match ch {
            'X' => Some(NodeType::Player),
            'K' => Some(NodeType::Block),
            'I' => Some(NodeType::Wall),
            's' => Some(NodeType::Switch),
            'b' => Some(NodeType::Button),
            'T' => Some(NodeType::ToggleBlock),
            '/' => Some(NodeType::Mirror(Direction::RIGHT)),
            '\\' => Some(NodeType::Mirror(Direction::LEFT)),
            '1' => Some(NodeType::Laser(Direction::UP, true)),
            '2' => Some(NodeType::Laser(Direction::DOWN, true)),
            '3' => Some(NodeType::Laser(Direction::LEFT, true)),
            '4' => Some(NodeType::Laser(Direction::RIGHT, true)),
            '5' => Some(NodeType::Laser(Direction::UP, false)),
            '6' => Some(NodeType::Laser(Direction::DOWN, false)),
            '7' => Some(NodeType::Laser(Direction::LEFT, false)),
            '8' => Some(NodeType::Laser(Direction::RIGHT, false)),
            'S' => Some(NodeType::Statue),
            'Z' => Some(NodeType::Zapper),
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
        let mut toggled = false;
        let (ch, fg_color, bg_color, toggled_fg_color, toggled_bg_color) = match node_type {
            NodeType::Player => ('X', Color::Green, Color::Green, Color::Green, Color::Green),
            NodeType::Block => ('K', Color::Grey, Color::Grey, Color::Grey, Color::Grey),
            NodeType::Wall => ('I', Color::White, Color::White, Color::White, Color::White),
            NodeType::Switch => ('s', Color::Black, Color::Red, Color::Black, Color::Yellow),
            NodeType::Button => ('b', Color::Black, Color::Red, Color::Black, Color::Yellow),
            NodeType::Zapper => (
                'Z',
                Color::Yellow,
                Color::Black,
                Color::Yellow,
                Color::Black,
            ),
            NodeType::Statue => (
                'S',
                Color::Rgb {
                    r: 100,
                    g: 100,
                    b: 0,
                },
                Color::Rgb {
                    r: 100,
                    g: 100,
                    b: 0,
                },
                Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 0,
                },
                Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 0,
                },
            ),
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
            NodeType::Laser(d, on) => {
                dir = d;
                toggled = !on;
                (
                    'L',
                    Color::Rgb { r: 255, g: 0, b: 0 },
                    Color::Rgb { r: 255, g: 0, b: 0 },
                    Color::Rgb { r: 150, g: 0, b: 0 },
                    Color::Rgb { r: 150, g: 0, b: 0 },
                )
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
            toggled,
            looking_at: vec![],
        }
    }
}
