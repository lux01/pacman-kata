use super::{Bounds, Position};

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct RenderScreen {
    rows: usize,
    cols: usize,
    cells: HashMap<Position, Cell>,
}

impl RenderScreen {
    pub fn new(bounds: &Bounds) -> RenderScreen {
        RenderScreen {
            rows: bounds.rows(),
            cols: bounds.cols(),
            cells: HashMap::new(),
        }
    }

    pub fn set(&mut self, posn: &Position, cell: Cell) {
        self.cells.insert(posn.clone(), cell);
    }

    pub fn set_str(&mut self, start: Position, s: &str) {
        for (idx, ch) in s.chars().enumerate() {
            self.cells.insert(Position { x: start.x + idx, .. start }, Cell::new(ch));
        }
    }
}

impl fmt::Display for RenderScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                if let Some(cell) = self.cells.get(&Position { x, y }) {
                    write!(f, "{}", cell.token)?;
                } else {
                    write!(f, "{}", ' ')?;
                }
            }
            if y != self.rows - 1 {
                write!(f, "{}", '\n')?;
            }
        }

        Ok(())
    }
}

pub trait Renderable<'a, 'b> {
    fn render(&'a self, screen: &'b mut RenderScreen);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cell {
    token: char,
}

impl Cell {
    pub fn new(token: char) -> Cell {
        Cell { token }
    }
}
