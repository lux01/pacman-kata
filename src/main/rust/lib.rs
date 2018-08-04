pub mod game;
pub mod parse_error;

pub mod tokens;

pub use self::game::Board;
pub use self::game::Game;
pub use self::game::Stats;

pub use self::parse_error::ParseError;

/// This struct contains options that Renderable types may use to affect how they render themselves.
pub struct RenderOptions {
    pub screen_width: usize,
}

/// The Renderable trait is implemented by any type that needs to be drawn directly onto the
/// game screen. Types are not informed as to where they are on the screen, but rather are
/// expected to draw themselves into a suitable string for display in the appropriate location.
pub trait Renderable {
    fn render(&self, &RenderOptions) -> String;
}

/// The Refreshable trait is implemented by a type that carries state that may need refreshing
/// upon level change.
pub trait Refreshable: Sized {
    fn refresh_from_str(&self, state: &str) -> Result<Self, ParseError>;
}
