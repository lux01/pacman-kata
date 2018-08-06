use super::{ParseError, Refreshable, RenderOptions, Renderable};

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub lives: u64,
    pub score: u64,
}

impl Default for Stats {
    fn default() -> Stats {
        Stats { lives: 3, score: 0 }
    }
}

impl Stats {
    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn lives(&self) -> u64 {
        self.lives
    }
}

impl Refreshable for Stats {
    fn refresh_from_str(&self, s: &str) -> Result<Stats, ParseError> {
        let end_of_lives = s.find(' ').ok_or(ParseError::CannotFindEndOfLives)?;
        let (lives, rest) = s.split_at(end_of_lives);
        let start_of_score = s.rfind(' ').ok_or(ParseError::CannotFindStartOfScore)?;
        let (_, score) = rest.split_at(start_of_score);

        let mut new_stats = self.clone();

        if lives != "?" {
            new_stats.lives = lives.parse()?;
        }
        if score != "?" {
            new_stats.score = score.parse()?;
        }

        Ok(new_stats)
    }
}

impl Renderable for Stats {
    fn render(&self, opts: &RenderOptions) -> String {
        let len_lives = (self.lives as f64).log10().ceil() as usize;
        let padding_width = opts.screen_width - len_lives;
        format!("{}{:width$}", self.lives, self.score, width = padding_width)
    }
}
