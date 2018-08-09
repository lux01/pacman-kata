use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wall {
    pub token: char,
}

impl Wall {
    pub fn try_parse(c: char) -> Option<Wall> {
        match c {
            '-' | '+' | '|' => Some(Wall { token: c }),
            _ => None,
        }
    }

}

impl From<Wall> for Cell {
    fn from(wall: Wall) -> Cell {
        From::from(&wall)
    }
}

impl<'a> From<&'a Wall> for Cell {
    fn from(wall: &'a Wall) -> Cell {
        Cell::new(wall.token)
    }
}

