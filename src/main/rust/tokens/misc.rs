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

impl Renderable for Gate {
    fn render(&self, _opts: &RenderOptions) -> String {
        "=".to_owned()
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

impl Renderable for ForceField {
    fn render(&self, _opts: &RenderOptions) -> String {
        "#".to_owned()
    }
}
