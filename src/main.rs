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
    match cli.command {
        Commands::encode {
            file,
            chunk_type,
            content,
        } => {
            println!("encode {file} {chunk_type} {content}");
            commands::encode(file, chunk_type, content);
        }

        Commands::decode { file, chunk_type } => {
            println!("decode {file} {chunk_type}");
            commands::decode(file, chunk_type);
        }

        Commands::remove { file, chunk_type } => {
            println!("remove {file} {chunk_type}");
            commands::remove(file, chunk_type);
        }
        Commands::print { file } => {
            println!("print {:?}", file);
            commands::print(file);
        }
    }
    Ok(())
}
