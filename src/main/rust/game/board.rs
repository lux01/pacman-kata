use super::super::tokens::{self, Token};
use super::ParseError;

use std::cmp::max;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pacman: Option<(Position, tokens::Pacman)>,
    ghosts: Vec<(Position, tokens::Ghost)>,
    pills: Vec<(Position, tokens::Pill)>,
}

impl Board {
    pub fn is_pacman_at(&self, pos: &Position) -> bool {
        if let Some((ref position, ref _pacman)) = self.pacman {
            position == pos
        } else {
            false
        }
    }

    pub fn is_ghost_at(&self, pos: &Position) -> bool {
        self.ghosts.iter()
            .filter(|&(p, _)| p == pos)
            .count() > 0
    }

    pub fn pacman(&self) -> Option<&tokens::Pacman> {
        match self.pacman {
            Some((_position, ref pacman)) => Some(pacman),
            None => None,
        }
    }

    pub fn get_pill_at(&self, pos: &Position) -> Option<&tokens::Pill> {
        self.pills.iter()
            .filter(|&(pill_pos, _)| pill_pos == pos)
            .map(|(_, pill)| pill)
            .next()
    }
}

impl FromStr for Board {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cols = 0;
        let mut x = 0;
        let mut y = 0;

        let mut board = Board {
            rows: 0,
            cols: 0,
            pacman: None,
            ghosts: vec![],
            pills: vec![],
        };

        for token_char in s.chars() {
            if token_char == '\n' {
                cols = max(cols, x);
                x = 0;
                y += 1;
                continue;
            }

            if let Some(token) = Token::from_char(token_char) {
                match token {
                    Token::PacmanToken(pacman) => board.pacman = Some((Position {x, y}, pacman)),
                    Token::GhostToken(ghost) => board.ghosts.push((Position { x, y }, ghost)),
                    Token::PillToken(pill) => board.pills.push((Position { x, y }, pill)),
                }
            }

            x += 1;
        }

        board.rows = y + 1;
        board.cols = cols;
        Ok(board)
    }
}
