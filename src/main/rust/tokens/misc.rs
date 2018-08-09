use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gate;

impl Gate {
    pub fn try_parse(c: char) -> Option<Gate> {
        if c == '=' {
            Some(Gate)
        } else {
            None
        }
    }
}

impl From<Gate> for Cell {
    fn from(gate: Gate) -> Cell {
        From::from(&gate)
    }
}

impl<'a> From<&'a Gate> for Cell {
    fn from(_gate: &'a Gate) -> Cell {
        Cell::new('=')
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForceField;

impl ForceField {
    pub fn try_parse(c: char) -> Option<ForceField> {
        if c == '#' {
            Some(ForceField)
        } else {
            None
        }
    }
}

impl From<ForceField> for Cell {
    fn from(ff: ForceField) -> Cell {
        From::from(&ff)
    }
}

impl<'a> From<&'a ForceField> for Cell {
    fn from(_ff: &'a ForceField) -> Cell {
        Cell::new('#')
    }
}