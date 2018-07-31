use super::ParseError;

use std::str::FromStr;

#[derive(Debug)]
pub struct Stats {
    pub lives: u64,
    pub score: u64,
}

impl FromStr for Stats {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let end_of_lives = s.find(' ').ok_or(ParseError::CannotFindEndOfLives)?;
        let (lives, rest) = s.split_at(end_of_lives);

        let start_of_score = s.rfind(' ').ok_or(ParseError::CannotFindStartOfScore)?;
        let (_, score) = rest.split_at(start_of_score);

        Ok(Stats {
            lives: lives.parse()?,
            score: score.parse()?,
        })
    }
}
