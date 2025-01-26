use std::fmt::{Display, Formatter};

use discord_api::InteractionError;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    NotAuthorized,
    ClientFailure(String),
    Invalid(String),
    DatabaseFailure(String),
    UnresolvedDiscordUser,
    Unexpected(String),
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Invalid(value.to_string())
    }
}

impl From<pog_common::error::Error> for Error {
    fn from(err: pog_common::error::Error) -> Self {
        match err {
            pog_common::error::Error::Unexpected(msg) => Error::Unexpected(msg),
            pog_common::error::Error::Database(msg) => Error::DatabaseFailure(msg),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

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

#[cfg(feature = "gcp")]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ClientFailure(value.to_string())
    }
}
