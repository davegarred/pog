#[derive(Debug, Clone)]
pub enum Error {
    ClientFailure(String),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ClientFailure(value.to_string())
    }
}
