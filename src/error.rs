use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    NotAuthorized,
    ClientFailure(String),
    Invalid(String),
    DatabaseFailure(String),
    UnresolvedDiscordUser,
}

impl From<&str> for crate::error::Error {
    fn from(value: &str) -> Self {
        Self::Invalid(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::DatabaseFailure(value.to_string())
    }
}
