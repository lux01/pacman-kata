pub mod board;
mod stats;

pub use super::ParseError;

pub use self::board::Board;
pub use self::board::Position;
pub use self::stats::Stats;

use std::str::FromStr;

#[derive(Clone, Default)]
pub struct Game {
    pub stats: Stats,
    pub board: Board,
}

impl Game {
    pub fn refresh_from_state(&self, s: &str) -> Result<Game, ParseError> {
        let end_of_stats_line = s.find('\n').ok_or(ParseError::CannotFindEndOfStatsLine)?;
        let (stats, board) = s.split_at(end_of_stats_line);

        let new_stats = self.stats.refresh_from_state(stats)?;
        let new_board = self.board.refresh_from_state(&board[1..])?;

        Ok(Game {
            stats: new_stats,
            board: new_board,
        })
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let end_of_stats_line = s.find('\n').ok_or(ParseError::CannotFindEndOfStatsLine)?;
        let (stats, board) = s.split_at(end_of_stats_line);

        Ok(Game {
            stats: stats.parse()?,
            board: board[1..].parse()?,
        })
    }
}
