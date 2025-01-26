#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Unexpected(String),
    Database(String),
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Unexpected(value.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err.to_string())
    }
}
