use std::fmt::{Display, Formatter};

use discord_api::InteractionError;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    NotAuthorized,
    ClientFailure(String),
    Invalid(String),
    DatabaseFailure(String),
    UnresolvedDiscordUser,
}

impl From<&str> for Error {
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

impl From<InteractionError> for Error {
    fn from(error: InteractionError) -> Self {
        let message = match error {
            InteractionError::MissingComponent(parent, component) => format!(
                "expected a '{}' field on the '{}' object",
                component, parent
            ),
            InteractionError::InvalidRequestPayload(message) => message,
        };
        Error::Invalid(message)
    }
}
