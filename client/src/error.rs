use serde::de::StdError;
use std::fmt::{Debug, Display, Formatter};

pub enum Error {
    ClientCreate(String),
    ClientUpdate(String),
    ClientDelete(String),
    Gemini(String),
    UnknownCommunication(String),
    NoGeminiCandidatesReceived,
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::UnknownCommunication(value.to_string())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::ClientCreate(msg) => msg,
            Error::ClientUpdate(msg) => msg,
            Error::ClientDelete(msg) => msg,
            Error::Gemini(msg) => msg,
            Error::UnknownCommunication(msg) => msg,
            Error::NoGeminiCandidatesReceived => "Gemini did not produce a valid candidate",
        };
        write!(f, "{}", msg)
    }
}

impl StdError for Error {}
