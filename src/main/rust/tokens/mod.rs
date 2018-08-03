mod pacman;
pub use self::pacman::*;

#[derive(Debug, Clone, Copy)]
pub struct Ghost;

impl Ghost {
    pub fn try_parse(c: char) -> Option<Ghost> {
        match c {
            'M' => Some(Ghost),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pill {
    pub score_value: u64,
}

impl Pill {
    pub fn try_parse(c: char) -> Option<Pill> {
        match c {
            '.' => Some(Pill { score_value: 10 }),
            'o' => Some(Pill { score_value: 50 }),
            _ => None,
        }
    }
}

pub enum Token {
    PacmanToken(Pacman),
    GhostToken(Ghost),
    PillToken(Pill),
}

impl Token {
    pub fn from_char(c: char) -> Option<Token> {
        if let Some(pacman) = Pacman::try_parse(c) {
            Some(Token::PacmanToken(pacman))
        } else if let Some(ghost) = Ghost::try_parse(c) {
            Some(Token::GhostToken(ghost))
        } else if let Some(pill) = Pill::try_parse(c) {
            Some(Token::PillToken(pill))
        } else {
            None
        }
    }
}
