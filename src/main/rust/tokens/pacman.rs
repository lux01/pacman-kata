use super::super::*;

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Orientation {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Orientation, ParseError> {
        match s {
            "LEFT" => Ok(Orientation::Left),
            "RIGHT" => Ok(Orientation::Right),
            "UP" => Ok(Orientation::Up),
            "DOWN" => Ok(Orientation::Down),
            _ => Err(ParseError::InvalidOrientation),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Pacman {
    pub orientation: Orientation,
    pub alive: bool,
}

impl Pacman {
    pub fn try_parse(c: char) -> Option<Pacman> {
        match c {
            '>' => Some(Pacman {
                orientation: Orientation::Left,
                alive: true,
            }),
            '<' => Some(Pacman {
                orientation: Orientation::Right,
                alive: true,
            }),
            'V' => Some(Pacman {
                orientation: Orientation::Up,
                alive: true,
            }),
            'Λ' => Some(Pacman {
                orientation: Orientation::Down,
                alive: true,
            }),
            '*' => Some(Pacman {
                orientation: Orientation::Down,
                alive: false,
            }),
            _ => None,
        }
    }

    pub fn die(&mut self) {
        self.alive = false;
    }
}

impl Mobile for Pacman {
    fn next_pos(&mut self, options: MoveOptions) -> Option<Position> {
        match self.orientation {
            Orientation::Left => options.left,
            Orientation::Right => options.right,
            Orientation::Up => options.up,
            Orientation::Down => options.down,
        }
    }
}

impl From<Pacman> for Cell {
    fn from(pacman: Pacman) -> Cell {
        From::from(&pacman)
    }
}

impl<'a> From<&'a Pacman> for Cell {
    fn from(pacman: &'a Pacman) -> Cell {
        if pacman.alive {
            match pacman.orientation {
                Orientation::Left => Cell::new('>'),
                Orientation::Right => Cell::new('<'),
                Orientation::Up => Cell::new('V'),
                Orientation::Down => Cell::new('Λ')
            }
        } else {
            Cell::new('*')
        }
    }
}