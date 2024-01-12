use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExtremelyCoolLooking => write!(f, "X"),
            Self::Musical => write!(f, "M"),
            Self::Aerodynamic => write!(f, "A"),
            Self::Shiny => write!(f, "S"),
        }
    }
}

impl TryFrom<char> for Category {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Self::ExtremelyCoolLooking),
            'm' => Ok(Self::Musical),
            'a' => Ok(Self::Aerodynamic),
            's' => Ok(Self::Shiny),
            _ => Err(()),
        }
    }
}
