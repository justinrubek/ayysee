use clap::ValueEnum;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub(crate) enum CompilationType {
    Ast,
    Mips,
}

impl Default for CompilationType {
    fn default() -> Self {
        Self::Mips
    }
}

impl std::fmt::Display for CompilationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationType::Ast => write!(f, "ast"),
            CompilationType::Mips => write!(f, "mips"),
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    /// Invoke the ayysee compiler
    Compile {
        /// The file to compile
        file: PathBuf,
        /// Select what type of output to generate
        #[clap(short, long, value_enum, default_value_t = CompilationType::default())]
        output: CompilationType,
    },
}
