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

pub use super::{MoveOptions, Position, RenderOptions, Renderable, Mobile};

macro_rules! token {
    ($($name:ident($inner:tt) -> $is_fn:ident, $get_immut:ident, $get_mut:ident,)*) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
        pub enum Token {
            $($name($inner),)*
        }

        impl Token {
            /// Attempt to parse the given `char` to a Token.
            pub fn from_char(c: char) -> Option<Token> {
                let mut result = None;

                $(result = result.or_else(|| $inner::try_parse(c).map(|v| Token::$name(v)));)*

                return result;
            }

            $(
                pub fn $is_fn(&self) -> bool {
                    match self {
                        Token::$name(_) => true,
                        _ => false,
                    }
                }

                pub fn $get_immut(&self) -> Option<&$inner> {
                    match self {
                        Token::$name(val) => Some(val),
                        _ => None,
                    }
                }

                pub fn $get_mut(&mut self) -> Option<&mut $inner> {
                    match self {
                        Token::$name(val) => Some(val),
                        _ => None,
                    }
                }
            )*
        }

        impl Renderable for Token {
            fn render(&self, opts: &RenderOptions) -> String {
                match self {
                    $(Token::$name(v) => v.render(opts),)*
                }
            }
        }

        $(
            impl From<$inner> for Token {
                fn from(v: $inner) -> Token {
                    Token::$name(v)
                }
            }
        )*
    }
}

token! {
    PacmanToken(Pacman) -> is_pacman, get_pacman, get_pacman_mut,
    GhostToken(Ghost) -> is_ghost, get_ghost, get_ghost_mut,
    PillToken(Pill) -> is_pill, get_pill, get_pill_mut,
    WallToken(Wall) -> is_wall, get_wall, get_wall_mut,
    ForceFieldToken(ForceField) -> is_force_field, get_force_field, get_force_field_mut,
    GateToken(Gate) -> is_gate, get_gate, get_gate_mut,
}
