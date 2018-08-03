use std::rc::Rc;

mod pacman;
pub use self::pacman::*;

pub struct Ghost;
impl Ghost {
    pub fn try_parse(c: char) -> Option<Ghost> {
        match c {
            'M' => Some(Ghost),
            _ => None,
        }
    }
}

pub enum Token {
    PacmanToken(Pacman),
    GhostToken(Ghost),
}

impl Token {
    pub fn from_char(c: char) -> Option<Rc<Token>> {
        if let Some(pacman) = Pacman::try_parse(c) {
            Some(Rc::new(Token::PacmanToken(pacman)))
        } else if let Some(ghost) = Ghost::try_parse(c) {
            Some(Rc::new(Token::GhostToken(ghost)))
        } else {
            None
        }
    }
}
