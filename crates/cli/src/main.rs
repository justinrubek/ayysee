use crate::{commands::Commands, error::Result};
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

            tracing::info!("{:?}", parsed);
        }
    }

    Ok(())
}
