use super::{RenderOptions, Renderable};

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

impl Renderable for Ghost {
    fn render(&self, _opts: &RenderOptions) -> String {
        "M".to_owned()
    }
}
