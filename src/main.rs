mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;
use args::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli.command);

    match cli.command {
        Commands::Encode(args) => {
            commands::encode(args)?;
        }

        Commands::Decode(args) => {
            commands::decode(args)?;
        }

        Commands::Remove(args) => {
            commands::remove(args)?;
        }
        Commands::Print(args) => {
            commands::print(args)?;
        }
    }
    Ok(())
}
