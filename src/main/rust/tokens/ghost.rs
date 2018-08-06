use super::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Ghost {
    current_pos: Option<Position>,
    previous_pos: Option<Position>,
}

impl Ghost {
    pub fn try_parse(c: char) -> Option<Ghost> {
        match c {
            'M' => Some(Ghost {
                current_pos: None,
                previous_pos: None,
            }),
            _ => None,
        }
    }

    pub fn next_pos(&mut self, valid_neighbours: MoveOptions) -> Option<Position> {
        let mut valid_posns = valid_neighbours.as_vec();
        if let Some(prev_posn) = self.previous_pos {
            for i in 0..valid_posns.len() {
                if valid_posns[i] == prev_posn {
                    valid_posns.remove(i);
                    break;
                }
            }
        }

        let next_pos = if valid_posns.len() > 0 {
            Some(valid_posns[0])
        } else {
            self.current_pos
        };

        self.previous_pos = self.current_pos;
        self.current_pos = next_pos;
        next_pos
    }
}

impl Renderable for Ghost {
    fn render(&self, _opts: &RenderOptions) -> String {
        "M".to_owned()
    }
}
