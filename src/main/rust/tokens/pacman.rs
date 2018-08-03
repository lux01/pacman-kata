use super::super::ParseError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

#[derive(Clone)]
pub struct Pacman {
    pub orientation: Orientation,
    pub alive: bool
}

impl Pacman {
    pub fn try_parse(c: char) -> Option<Pacman> {
        match c {
            '>' => Some(Pacman { orientation: Orientation::Left, alive: true }),
            '<' => Some(Pacman { orientation: Orientation::Right, alive: true }),
            'V' => Some(Pacman { orientation: Orientation::Up, alive: true }),
            'Λ' => Some(Pacman { orientation: Orientation::Down, alive: true }),
            '*' => Some(Pacman { orientation: Orientation::Down, alive: false }),
            _ => None,
        }
    }
}
