pub mod board;
pub mod board_token;
pub mod game;
pub mod parse_error;
pub mod position;
pub mod render;
pub mod stats;
pub mod tokens;

pub use self::board::*;
pub use self::board_token::*;
pub use self::game::*;
pub use self::parse_error::*;
pub use self::position::*;
pub use self::render::*;
pub use self::stats::*;
pub use self::tokens::*;

/// The Refreshable trait is implemented by a type that carries state that may need refreshing
/// upon level change.
pub trait Refreshable: Sized {
    fn refresh_from_str(&self, state: &str) -> Result<Self, ParseError>;
}

#[derive(Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord, Clone, Copy)]
pub struct Bounds {
    max_x: usize,
    max_y: usize,
}

impl Bounds {
    pub fn new(rows: usize, cols: usize) -> Bounds {
        Bounds {
            max_x: rows,
            max_y: cols,
        }
    }

    pub fn rows(&self) -> usize {
        self.max_y + 1
    }

    pub fn cols(&self) -> usize {
        self.max_x
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MoveOptions {
    pub left: Option<Position>,
    pub right: Option<Position>,
    pub up: Option<Position>,
    pub down: Option<Position>,
}

impl MoveOptions {
    pub fn from(pos: &Position, bounds: &Bounds, board: &ImmobileTokens) -> MoveOptions {
        let left = pos.left(bounds);
        let right = pos.right(bounds);
        let up = pos.up(bounds);
        let down = pos.down(bounds);

        MoveOptions {
            left: if !board.is_wall_at(&left) {
                Some(left)
            } else {
                None
            },
            right: if !board.is_wall_at(&right) {
                Some(right)
            } else {
                None
            },
            up: if !board.is_wall_at(&up) {
                Some(up)
            } else {
                None
            },
            down: if !board.is_wall_at(&down) {
                Some(down)
            } else {
                None
            },
        }
    }

    pub fn as_vec(self) -> Vec<Position> {
        let mut vec = Vec::with_capacity(4);
        if let Some(left) = self.left {
            vec.push(left);
        }
        if let Some(right) = self.right {
            vec.push(right);
        }
        if let Some(up) = self.up {
            vec.push(up);
        }
        if let Some(down) = self.down {
            vec.push(down);
        }
        vec
    }
}

pub trait Mobile {
    fn next_pos(&mut self, options: MoveOptions) -> Option<Position>;
}
