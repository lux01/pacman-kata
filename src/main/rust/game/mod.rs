pub mod board;
mod stats;

pub use self::board::Board;
pub use self::board::Position;
pub use self::stats::Stats;

pub use super::{ParseError, Refreshable, RenderOptions, Renderable};

#[derive(Clone, Default)]
pub struct Game {
    pub stats: Stats,
    pub board: Board,
}

impl Game {
    pub fn render_game(&self) -> String {
        let opts = RenderOptions {
            screen_width: self.board.cols,
        };

        self.render(&opts)
    }
}

impl Renderable for Game {
    fn render(&self, opts: &RenderOptions) -> String {
        format!("{}\n{}", self.stats.render(opts), self.board.render(opts))
    }
}

impl Refreshable for Game {
    fn refresh_from_str(&self, s: &str) -> Result<Self, ParseError> {
        let end_of_stats_line = s.find('\n').ok_or(ParseError::CannotFindEndOfStatsLine)?;
        let (stats, board) = s.split_at(end_of_stats_line);

        let new_stats = self.stats.refresh_from_str(stats)?;
        let new_board = self.board.refresh_from_str(&board[1..])?;

        Ok(Game {
            stats: new_stats,
            board: new_board,
        })
    }
}
