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

pub use super::{RenderOptions, Renderable};

macro_rules! token {
    ($($name:ident($inner:tt),)*) => {
        pub enum Token {
            $($name($inner),)*
        }

        impl Token {
            pub fn from_char(c: char) -> Option<Token> {
                let mut result = None;

                $(result = result.or_else(|| $inner::try_parse(c).map(|v| Token::$name(v)));)*

                return result;
            }
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
    PacmanToken(Pacman),
    GhostToken(Ghost),
    PillToken(Pill),
    WallToken(Wall),
    ForceFieldToken(ForceField),
    GateToken(Gate),
}
