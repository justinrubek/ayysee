#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("usage of undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("usage of undefined function: {0}")]
    UndefinedFunction(String),
    #[error("main function not defined")]
    UndefinedMain,
    #[error(transparent)]
    Mips(#[from] stationeers_mips::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
