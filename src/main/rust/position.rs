use super::Bounds;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn left(&self, bounds: &Bounds) -> Position {
        Position {
            x: (self.x + bounds.max_x - 1) % bounds.max_x,
            y: self.y,
        }
    }

    pub fn right(&self, bounds: &Bounds) -> Position {
        Position {
            x: (self.x + 1) % bounds.max_x,
            y: self.y,
        }
    }

    pub fn up(&self, bounds: &Bounds) -> Position {
        Position {
            y: (self.y + bounds.max_y - 1) % bounds.max_y,
            x: self.x,
        }
    }

    pub fn down(&self, bounds: &Bounds) -> Position {
        Position {
            y: (self.y + 1) % bounds.max_y,
            x: self.x,
        }
    }
}