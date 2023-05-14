#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("todo")]
    Todo,
    #[error("failed to parse")]
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
