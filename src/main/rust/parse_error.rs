use std::num;

#[derive(Debug)]
pub enum ParseError {
    NotParsed,
    ParseIntError(num::ParseIntError),
    CannotFindEndOfLives,
    CannotFindStartOfScore,
    CannotFindEndOfStatsLine,
    InvalidOrientation,
}

impl Default for ParseError {
    fn default() -> Self {
        ParseError::NotParsed
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(e: num::ParseIntError) -> ParseError {
        ParseError::ParseIntError(e)
    }
}
