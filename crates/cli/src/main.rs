use crate::{commands::Commands, error::Result};
use ayysee_compiler::generate_program;
use ayysee_parser::grammar::ProgramParser;
use clap::Parser;

mod commands;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Compile { file } => {
            let file_contents = tokio::fs::read_to_string(file).await.unwrap();

            let parser = ProgramParser::new();

            let parsed = parser.parse(&file_contents).unwrap();

            let compiled = generate_program(parsed)?;

            println!("{}", compiled);
        }
    }

    Ok(())
}
