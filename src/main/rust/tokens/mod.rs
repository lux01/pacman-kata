use std::rc::Rc;

mod pacman;
pub use self::pacman::*;

pub enum Token {
    PacmanToken(Pacman),
}

impl Token {
    pub fn from_char(c: char) -> Option<Rc<Token>> {
        if let Some(pacman) = Pacman::try_parse(c) {
            Some(Rc::new(Token::PacmanToken(pacman)))
        } else {
            None
        }
    }
}
