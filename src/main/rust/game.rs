pub use super::*;

#[derive(Clone, Default)]
pub struct Game {
    pub stats: Stats,
    pub board: Board,
}

impl Game {
    pub fn render_game(&self) -> String {
        let opts = RenderOptions {
            screen_width: self.board.bounds().max_x,
        };

        self.render(&opts)
    }

    pub fn tick(&mut self) {
        let score_change = self.board.tick();
        self.stats.score += score_change;
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
