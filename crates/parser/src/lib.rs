use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod error;
pub mod utils;

lalrpop_mod!(
    #[allow(clippy::all)]
    pub script
);
