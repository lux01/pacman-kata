#[derive(Debug, Clone, Copy)]
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
