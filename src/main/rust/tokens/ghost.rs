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