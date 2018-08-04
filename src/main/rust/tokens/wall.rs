use super::{RenderOptions, Renderable};

#[derive(Debug, Clone, Copy)]
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

impl Renderable for Wall {
    fn render(&self, _opts: &RenderOptions) -> String {
        format!("{}", self.token)
    }
}
