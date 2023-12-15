use std::fmt::Debug;

#[derive(PartialEq, Clone, Copy)]
pub enum CosmicElement {
    Void,
    Galaxy,
}

impl Debug for CosmicElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "."),
            Self::Galaxy => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for CosmicElement {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(CosmicElement::Void),
            '#' => Ok(CosmicElement::Galaxy),
            _ => Err(()),
        }
    }
}
