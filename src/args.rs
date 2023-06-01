use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    pub file: PathBuf,
    pub chunk_type: ChunkType,
    pub content: String,
    pub output_file_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub file: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub file: PathBuf,
    #[arg(help = "chunk type like rust")]
    pub chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    pub file: PathBuf,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
