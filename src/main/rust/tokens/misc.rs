#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
