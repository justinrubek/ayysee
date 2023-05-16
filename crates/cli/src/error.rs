#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    CompileError(#[from] ayysee_compiler::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
