#[derive(Debug)]
pub enum Error {
    NoPool,
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Database(e)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Error::NoPool => write!(f, "No pool"),
            Error::Database(e) => write!(f, "{}", e),
        }
    }
}
