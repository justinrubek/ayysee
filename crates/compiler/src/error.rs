#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("usage of undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("usage of undefined function: {0}")]
    UndefinedFunction(String),
    #[error("main function not defined")]
    UndefinedMain,
    #[error("unknown built-in function: {0}")]
    UnknownBuiltin(String),
    #[error("wrong number of arguments for {0}: expected {1}, got {2}")]
    WrongArgCount(String, usize, usize),
    #[error("invalid batch mode: {0} (expected average, sum, min, or max)")]
    InvalidBatchMode(String),
    #[error("break used outside of a loop")]
    BreakOutsideLoop,
    #[error(transparent)]
    Mips(#[from] stationeers_mips::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
