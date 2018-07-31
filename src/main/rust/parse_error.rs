use std::num;

#[derive(Debug)]
pub enum ParseError {
    ParseIntError(num::ParseIntError),
    CannotFindEndOfLives,
    CannotFindStartOfScore,
    CannotFindEndOfStatsLine,
}

impl From<num::ParseIntError> for ParseError {
    fn from(e: num::ParseIntError) -> ParseError {
        ParseError::ParseIntError(e)
    }
}
