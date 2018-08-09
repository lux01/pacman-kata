use super::tokens::*;
use super::*;

use std::cmp::max;

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

    pub fn tick(&mut self) -> TickResult {
        let mut tick_result = TickResult::default();
        self.move_all_tokens();
        self.process_collisions(&mut tick_result);
        tick_result
    }

    fn move_all_tokens(&mut self) {
        if let Some(ref mut pacman) = self.mobile.pacman {
            pacman.tick(&self.bounds, &self.immobile);
        }

        for ghost in self.mobile.ghosts.iter_mut() {
            ghost.tick(&self.bounds, &self.immobile);
        }
    }

    fn process_collisions(&mut self, tick_result: &mut TickResult) {
        if let Some(ref mut pacman_token) = self.mobile.pacman {
            Board::process_collisions_with_pacman(
                pacman_token,
                &mut self.immobile.pills,
                &mut self.mobile.ghosts,
                tick_result,
            );
        }
    }

    fn process_collisions_with_pacman(
        pacman_token: &mut BoardToken<Pacman>,
        pills: &mut Vec<BoardToken<Pill>>,
        ghosts: &mut Vec<BoardToken<Ghost>>,
        tick_result: &mut TickResult,
    ) {
        let pacman_posn = pacman_token.posn();

        pills.retain(|pill| {
            if pill.posn() == pacman_posn {
                tick_result.score_increase += pill.as_ref().score_value;
                false
            } else {
                true
            }
        });

        for ref mut ghost_token in ghosts.iter_mut() {
            let ghost_posn = ghost_token.posn();
            let ref mut ghost = ghost_token.as_ref_mut();

            if tick_result.score_increase >= 50 {
                ghost.scare();
            }

            if ghost_posn == pacman_posn {
                Board::collide_pacman_with_ghost(pacman_token.as_ref_mut(), ghost, tick_result);
            }
        }
    }

    fn collide_pacman_with_ghost(pacman: &mut Pacman, ghost: &mut Ghost, tick_result: &mut TickResult) {
        if ghost.is_scared() {

        } else {
            tick_result.did_pacman_die = true;
            pacman.die();
        }
    }
}

impl<'a, 'b> Renderable<'a, 'b> for Board {
    fn render(&'a self, screen: &'b mut RenderScreen) {
        self.immobile.render(screen);
        self.mobile.render(screen);
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
