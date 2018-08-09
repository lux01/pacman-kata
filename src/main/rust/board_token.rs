use super::*;


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

impl<'a, 'b, T> Renderable<'a, 'b> for BoardToken<T> where Cell: From<&'a T>, T: 'a {
    fn render(&'a self, screen: &'b mut RenderScreen) {
        screen.set(&self.posn, From::from(&self.token));
    }
}


#[derive(Debug, Clone, Default)]
pub struct MobileTokens {
    pub pacman: Option<BoardToken<Pacman>>,
    pub ghosts: Vec<BoardToken<Ghost>>,
}

impl MobileTokens {
    pub fn is_pacman_at(&self, pos: &Position) -> bool {
        self.pacman.as_ref().map(|t| t.is_at(pos)).unwrap_or(false)
    }

    pub fn is_ghost_at(&self, pos: &Position) -> bool {
        self.ghosts.iter().filter(|t| t.is_at(pos)).next().is_some()
    }

    pub fn get_pacman(&self) -> Option<&Pacman> {
        self.pacman.as_ref().map(|token| token.as_ref())
    }
}

impl<'a, 'b> Renderable<'a, 'b> for MobileTokens {
    fn render(&'a self, screen: &'b mut RenderScreen) {
        for ghost in self.ghosts.iter() {
            ghost.render(screen);
        }
        for pacman in self.pacman.iter() {
            pacman.render(screen);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImmobileTokens {
    pub pills: Vec<BoardToken<Pill>>,
    pub walls: Vec<BoardToken<Wall>>,
    pub gates: Vec<BoardToken<Gate>>,
    pub force_fields: Vec<BoardToken<ForceField>>,
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

impl<'a, 'b> Renderable<'a, 'b> for ImmobileTokens {
    fn render(&'a self, screen: &'b mut RenderScreen) {
        for ff in self.force_fields.iter() {
            ff.render(screen);
        }
        
        for gate in self.gates.iter() {
            gate.render(screen);
        }

        for wall in self.walls.iter() {
            wall.render(screen);
        }

        for pill in self.pills.iter() {
            pill.render(screen);
        }
    }
}