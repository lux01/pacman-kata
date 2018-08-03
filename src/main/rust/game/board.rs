use super::super::tokens::Token;
use super::ParseError;

use std::cmp::max;
use std::collections::HashMap;
use std::rc::Rc;
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
    tokens: HashMap<Position, Vec<Rc<Token>>>,
    pacman_posn: Option<Position>,
    pacman_token: Option<Rc<Token>>,
    ghost_posns: Vec<Position>,
    ghost_tokens: Vec<Rc<Token>>,
}

impl Board {
    pub fn is_pacman_at(&self, pos: &Position) -> bool {
        self.pacman_posn.map(|p| p == *pos).unwrap_or(false)
    }

    pub fn is_ghost_at(&self, pos: &Position) -> bool {
        self.ghost_posns.contains(pos)
    }

    pub fn pacman_token(&self) -> Option<Rc<Token>> {
        self.pacman_token.clone()
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
            tokens: HashMap::new(),
            pacman_posn: None,
            pacman_token: None,
            ghost_posns: vec![],
            ghost_tokens: vec![],
        };

        for token_char in s.chars() {
            if token_char == '\n' {
                cols = max(cols, x);
                x = 0;
                y += 1;
                continue;
            }

            match Token::from_char(token_char) {
                Some(token) => {
                    match token.as_ref() {
                        Token::PacmanToken(_) => {
                            board.pacman_posn = Some(Position { x, y });
                            board.pacman_token = Some(token.clone());
                        }
                        Token::GhostToken(_) => {
                            board.ghost_posns.push(Position { x, y });
                            board.ghost_tokens.push(token.clone());
                        }
                    }

                    board
                        .tokens
                        .entry(Position { x, y })
                        .or_insert(vec![])
                        .push(token);
                }
                None => {}
            }

            x += 1;
        }

        board.rows = y + 1;
        board.cols = cols;
        Ok(board)
    }
}
