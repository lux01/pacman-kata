use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn next_pos(&mut self, _valid_neighbours: MoveOptions) -> Option<Position> {
        None
    }
}

impl Renderable for Pill {
    fn render(&self, _opts: &RenderOptions) -> String {
        match self.score_value {
            10 => ".",
            50 => "o",
            _ => unreachable!(),
        }.to_owned()
    }
}
