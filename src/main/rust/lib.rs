pub mod board;
pub mod game;
pub mod parse_error;
pub mod stats;
pub mod tokens;

pub use self::board::*;
pub use self::game::*;
pub use self::parse_error::*;
pub use self::stats::*;
pub use self::tokens::*;

/// This struct contains options that Renderable types may use to affect how they render themselves.
pub struct RenderOptions {
    pub screen_width: usize,
}

/// The Renderable trait is implemented by any type that needs to be drawn directly onto the
/// game screen. Types are not informed as to where they are on the screen, but rather are
/// expected to draw themselves into a suitable string for display in the appropriate location.
pub trait Renderable {
    fn render(&self, &RenderOptions) -> String;
}

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
    pub fn from(pos: &Position, bounds: &Bounds, board: &Board) -> MoveOptions {
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
