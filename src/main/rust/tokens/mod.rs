mod ghost;
mod misc;
mod pacman;
mod pill;
mod wall;

pub use self::ghost::*;
pub use self::misc::*;
pub use self::pacman::*;
pub use self::pill::*;
pub use self::wall::*;

pub enum Token {
    PacmanToken(Pacman),
    GhostToken(Ghost),
    PillToken(Pill),
    WallToken(Wall),
    ForceFieldToken(ForceField),
    GateToken(Gate),
}

impl Token {
    pub fn from_char(c: char) -> Option<Token> {
        if let Some(pacman) = Pacman::try_parse(c) {
            Some(Token::PacmanToken(pacman))
        } else if let Some(ghost) = Ghost::try_parse(c) {
            Some(Token::GhostToken(ghost))
        } else if let Some(pill) = Pill::try_parse(c) {
            Some(Token::PillToken(pill))
        } else if let Some(wall) = Wall::try_parse(c) {
            Some(Token::WallToken(wall))
        } else if let Some(force_field) = ForceField::try_parse(c) {
            Some(Token::ForceFieldToken(force_field))
        } else if let Some(gate) = Gate::try_parse(c) {
            Some(Token::GateToken(gate))
        } else {
            None
        }
    }
}
