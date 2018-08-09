pub use super::*;

#[derive(Clone, Default)]
pub struct Game {
    pub stats: Stats,
    pub board: Board,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TickResult {
    pub score_increase: u64,
    pub did_pacman_die: bool,
}

impl Game {
    pub fn tick(&mut self) {
        let tick_result = self.board.tick();
        self.stats.score += tick_result.score_increase;

        if tick_result.did_pacman_die {
            self.stats.lives -= 1;
        }
    }

    pub fn render_game(&self) -> String {
        let mut screen = RenderScreen::new(&self.board.bounds());
        self.board.render(&mut screen);

        if self.stats.lives == 0 {
            let x_offset = if self.board.bounds().cols() > 6 { 2 } else { 1 };
            screen.set_str(Position::new(x_offset, 1), "GAME");
            screen.set_str(Position::new(x_offset, 2), "OVER");
        }

        format!("{}\n{}", self.stats, screen)
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
