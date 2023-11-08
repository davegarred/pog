#[derive(Debug, PartialEq, Clone)]
pub enum InteractionError {
    MissingComponent(String, String),
    InvalidRequestPayload(String),
}

impl From<&str> for InteractionError {
    fn from(value: &str) -> Self {
        Self::InvalidRequestPayload(value.to_string())
    }
}
impl From<(&str, &str)> for InteractionError {
    fn from(value: (&str, &str)) -> Self {
        Self::MissingComponent(value.0.to_string(), value.1.to_string())
    }
}

impl std::fmt::Display for InteractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InteractionError {}
