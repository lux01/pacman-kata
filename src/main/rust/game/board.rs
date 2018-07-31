use super::ParseError;

use std::cmp::max;
use std::str::FromStr;

#[derive(Debug)]
pub struct Board {
    pub rows: usize,
    pub cols: usize,
}

impl FromStr for Board {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cols = 0;
        let mut x = 0;
        let mut y = 0;

        for token_char in s.chars() {
            if token_char == '\n' {
                cols = max(cols, x);
                x = 0;
                y += 1;
                continue;
            }
            x += 1;
        }

        Ok(Board { rows: y, cols })
    }
}
