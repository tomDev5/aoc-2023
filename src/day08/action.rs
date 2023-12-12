#[derive(Debug)]
pub enum Action {
    L,
    R,
}

impl TryFrom<char> for Action {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Action::L),
            'R' => Ok(Action::R),
            _ => Err(format!("Couldn't convert '{value}' to a direction")),
        }
    }
}
