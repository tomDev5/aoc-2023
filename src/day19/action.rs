#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Accept,
    Reject,
    SendTo(String),
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            other => Self::SendTo(other.to_string()),
        }
    }
}
