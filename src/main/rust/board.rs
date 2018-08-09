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

#[derive(Debug, Clone)]
pub struct BoardToken<T> {
    posn: Position,
    token: T,
}

impl<T> BoardToken<T> {
    pub fn new(posn: Position, token: T) -> BoardToken<T> {
        BoardToken { posn, token }
    }
    pub fn is_at(&self, pos: &Position) -> bool {
        self.posn == *pos
    }

    pub fn unwrap(self) -> T {
        self.token
    }

    pub fn as_ref(&self) -> &T {
        &self.token
    }

    pub fn as_ref_mut(&mut self) -> &mut T {
        &mut self.token
    }

    pub fn posn(&self) -> Position {
        self.posn.clone()
    }
}

impl<T: Mobile> BoardToken<T> {
    pub fn tick(&mut self, bounds: &Bounds, immobiles: &ImmobileTokens) {
        let move_opts = MoveOptions::from(&self.posn, bounds, immobiles);
        if let Some(new_pos) = self.token.next_pos(move_opts) {
            self.posn = new_pos;
        }
    }
}

impl<T: Renderable> Renderable for BoardToken<T> {
    fn render(&self, opts: &RenderOptions) -> String {
        self.token.render(opts)
    }
}

#[derive(Debug, Clone, Default)]
pub struct MobileTokens {
    pacman: Option<BoardToken<Pacman>>,
    ghosts: Vec<BoardToken<Ghost>>,
}

impl MobileTokens {

    pub fn is_pacman_at(&self, pos: &Position) -> bool {
        self.pacman.as_ref().map(|t| t.is_at(pos)).unwrap_or(false)
    }

    pub fn is_ghost_at(&self, pos: &Position) -> bool {
        self.ghosts
            .iter()
            .filter(|t| t.is_at(pos))
            .next()
            .is_some()
    }

    pub fn get_pacman(&self) -> Option<&Pacman> {
        self.pacman.as_ref().map(|token| token.as_ref())
    }

}

#[derive(Debug, Clone, Default)]
pub struct ImmobileTokens {
    pills: Vec<BoardToken<Pill>>,
    walls: Vec<BoardToken<Wall>>,
    gates: Vec<BoardToken<Gate>>,
    force_fields: Vec<BoardToken<ForceField>>,
}

impl ImmobileTokens {
    pub fn is_pill_at(&self, pos: &Position) -> bool {
        self.pills.iter().filter(|t| t.is_at(pos)).next().is_some()
    }

    pub fn is_wall_at(&self, pos: &Position) -> bool {
        self.walls.iter().filter(|t| t.is_at(pos)).next().is_some()
    }

    pub fn is_gate_at(&self, pos: &Position) -> bool {
        self.gates.iter().filter(|t| t.is_at(pos)).next().is_some()
    }

    pub fn is_force_field_at(&self, pos: &Position) -> bool {
        self.force_fields
            .iter()
            .filter(|t| t.is_at(pos))
            .next()
            .is_some()
    }

    pub fn get_pill_at(&self, pos: &Position) -> Option<&Pill> {
        self.pills
            .iter()
            .filter(|pill| pill.is_at(pos))
            .map(|pill| pill.as_ref())
            .next()
    }
}

#[derive(Clone, Default)]
pub struct Board {
    bounds: Bounds,
    mobile: MobileTokens,
    immobile: ImmobileTokens,
    power_pill_active: bool,
}

impl Board {
    pub fn bounds(&self) -> Bounds {
        Bounds { ..self.bounds }
    }

    pub fn immobile_tokens(&self) -> &ImmobileTokens {
        &self.immobile
    }

    pub fn mobile_tokens(&self) -> &MobileTokens {
        &self.mobile
    }

    pub fn tick(&mut self) -> u64 {
        self.move_all_tokens();
        self.process_collisions()
    }

    fn move_all_tokens(&mut self) {
        if let Some(ref mut pacman) = self.mobile.pacman {
            pacman.tick(&self.bounds, &self.immobile);
        }

        for ghost in self.mobile.ghosts.iter_mut() {
            ghost.tick(&self.bounds, &self.immobile);
        }
    }

    fn process_collisions(&mut self) -> u64 {
        let mut score_increase = 0;

        if let Some(ref mut pacman_token) = self.mobile.pacman {
            let pacman_posn = pacman_token.posn();

            self.immobile.pills.retain(|pill| {
                if pill.posn() == pacman_posn {
                    score_increase += pill.as_ref().score_value;
                    false
                } else {
                    true
                }
            });

            for ref mut ghost_token in self.mobile.ghosts.iter_mut() {
                let ghost_posn = ghost_token.posn();
                let ref mut ghost = ghost_token.as_ref_mut();
                
                if score_increase >= 50 {
                    ghost.scare();
                }

                if ghost_posn == pacman_posn {
                    // Do something
                }
            }
        }

        score_increase
    }
}

impl Renderable for Board {
    fn render(&self, opts: &RenderOptions) -> String {
        let rendered_tokens = {
            let mut rendered_map = HashMap::new();
            rendered_map.extend(self.immobile.force_fields.iter().map(|token| (token.posn(), token.render(opts))));
            rendered_map.extend(self.immobile.gates.iter().map(|token| (token.posn(), token.render(opts))));
            rendered_map.extend(self.immobile.walls.iter().map(|token| (token.posn(), token.render(opts))));
            rendered_map.extend(self.immobile.pills.iter().map(|token| (token.posn(), token.render(opts))));
            rendered_map.extend(self.mobile.ghosts.iter().map(|token| (token.posn(), token.render(opts))));
            rendered_map.extend(self.mobile.pacman.iter().map(|token| (token.posn(), token.render(opts))));
            rendered_map
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
                match token {
                    Token::PacmanToken(pacman) => {
                        board.mobile.pacman = Some(BoardToken::new(posn, pacman))
                    }
                    Token::GhostToken(ghost) => {
                        board.mobile.ghosts.push(BoardToken::new(posn, ghost))
                    }
                    Token::PillToken(pill) => {
                        board.immobile.pills.push(BoardToken::new(posn, pill))
                    }
                    Token::GateToken(gate) => {
                        board.immobile.gates.push(BoardToken::new(posn, gate))
                    }
                    Token::WallToken(wall) => {
                        board.immobile.walls.push(BoardToken::new(posn, wall))
                    }
                    Token::ForceFieldToken(ff) => {
                        board.immobile.force_fields.push(BoardToken::new(posn, ff))
                    }
                }
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
