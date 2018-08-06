use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn next_pos(&mut self, _valid_neighbours: MoveOptions) -> Option<Position> {
        None
    }
}

impl Renderable for Wall {
    fn render(&self, _opts: &RenderOptions) -> String {
        format!("{}", self.token)
    }
}
