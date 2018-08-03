pub mod board;
mod stats;

pub use super::ParseError;

pub use self::board::Board;
pub use self::board::Position;
pub use self::stats::Stats;

use std::str::FromStr;

#[derive(Clone)]
pub struct Game {
    pub stats: Stats,
    pub field: Board,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let end_of_stats_line = s.find('\n').ok_or(ParseError::CannotFindEndOfStatsLine)?;
        let (stats, board) = s.split_at(end_of_stats_line);

        Ok(Game {
            stats: stats.parse()?,
            field: board[1..].parse()?,
        })
    }
}
