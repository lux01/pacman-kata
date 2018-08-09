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
}

impl From<Pill> for Cell {
    fn from(pill: Pill) -> Cell {
        From::from(&pill)
    }
}

impl<'a> From<&'a Pill> for Cell {
    fn from(pill: &'a Pill) -> Cell {
        match pill.score_value {
            10 => Cell::new('.'),
            50 => Cell::new('o'),
            _ => unreachable!()
        }
    }
}