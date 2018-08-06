use super::tokens::*;
use super::*;

use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn left(&self, bounds: &Bounds) -> Position {
        Position {
            x: (self.x + bounds.max_x - 1) % bounds.max_x,
            y: self.y,
        }
    }

    pub fn right(&self, bounds: &Bounds) -> Position {
        Position {
            x: (self.x + 1) % bounds.max_x,
            y: self.y,
        }
    }

    pub fn up(&self, bounds: &Bounds) -> Position {
        Position {
            y: (self.y + bounds.max_y - 1) % bounds.max_y,
            x: self.x,
        }
    }

    pub fn down(&self, bounds: &Bounds) -> Position {
        Position {
            y: (self.y + 1) % bounds.max_y,
            x: self.x,
        }
    }
}

#[derive(Clone, Default)]
pub struct Board {
    bounds: Bounds,
    tokens: Vec<(Position, Token)>,
}

impl Board {
    pub fn bounds(&self) -> Bounds {
        Bounds { ..self.bounds }
    }

    pub fn is_pacman_at(&self, pos: &Position) -> bool {
        self.tokens
            .iter()
            .filter(|&(posn, token)| posn == pos && token.is_pacman())
            .count() > 0
    }

    pub fn is_ghost_at(&self, pos: &Position) -> bool {
        self.tokens
            .iter()
            .filter(|&(posn, token)| posn == pos && token.is_ghost())
            .count() > 0
    }

    pub fn is_pill_at(&self, pos: &Position) -> bool {
        self.tokens
            .iter()
            .filter(|&(posn, token)| posn == pos && token.is_pill())
            .count() > 0
    }

    pub fn is_wall_at(&self, pos: &Position) -> bool {
        self.tokens
            .iter()
            .filter(|&(posn, token)| posn == pos && token.is_wall())
            .count() > 0
    }

    pub fn is_gate_at(&self, pos: &Position) -> bool {
        self.tokens
            .iter()
            .filter(|&(posn, token)| posn == pos && token.is_gate())
            .count() > 0
    }

    pub fn is_force_field_at(&self, pos: &Position) -> bool {
        self.tokens
            .iter()
            .filter(|&(posn, token)| posn == pos && token.is_force_field())
            .count() > 0
    }

    pub fn get_pacman(&self) -> Option<&tokens::Pacman> {
        self.tokens
            .iter()
            .filter(|&(_, token)| token.is_pacman())
            .next()
            .and_then(|(_, token)| token.get_pacman())
    }

    pub fn get_pill_at(&self, pos: &Position) -> Option<&tokens::Pill> {
        self.tokens
            .iter()
            .filter(|&(posn, token)| posn == pos && token.is_pill())
            .next()
            .and_then(|(_, token)| token.get_pill())
    }

    pub fn tick(&mut self) {
        let bounds = &self.bounds;
        let mut new_tokens = self.tokens.clone();

        for (ref mut posn, ref mut token) in new_tokens.iter_mut() {
            if !token.is_pacman() && !token.is_ghost() {
                continue;
            }

            let move_options = MoveOptions::from(posn, bounds, self);
            if let Some(new_pos) = token.next_pos(move_options) {
                *posn = new_pos;
            }
        }

        self.tokens = new_tokens;
    }
}

impl Renderable for Board {
    fn render(&self, opts: &RenderOptions) -> String {
        let rendered_tokens = {
            let mut tokens = self.tokens.clone();
            tokens.sort_by(|(posn1, token1), (posn2, token2)| {
                posn1.cmp(posn2).then(token1.cmp(token2))
            });
            tokens.dedup_by(|t2, t1| t2.0 == t1.0);

            tokens
                .into_iter()
                .map(|(posn, token)| (posn, token.render(opts)))
                .collect::<HashMap<Position, String>>()
        };

        let mut output = String::with_capacity(self.bounds.max_y * self.bounds.max_x);
        for y in 0..self.bounds.max_y + 1 {
            for x in 0..self.bounds.max_x {
                if let Some(token) = rendered_tokens.get(&Position { x, y }) {
                    output.push_str(token);
                } else {
                    output.push_str(" ");
                }
            }
            if y != self.bounds.max_y {
                output.push('\n');
            }
        }
        output
    }
}

impl Refreshable for Board {
    fn refresh_from_str(&self, s: &str) -> Result<Self, ParseError> {
        let mut cols = 0;
        let mut x = 0;
        let mut y = 0;

        let mut board = Board::default();

        for token_char in s.chars() {
            if token_char == '\n' {
                cols = max(cols, x);
                x = 0;
                y += 1;
                continue;
            }

            if let Some(token) = Token::from_char(token_char) {
                let posn = Position { x, y };
                board.tokens.push((posn, token));
            }

            x += 1;
        }

        board.bounds = Bounds {
            max_x: cols,
            max_y: y,
        };

        Ok(board)
    }
}
