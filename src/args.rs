use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    encode {
        file: String,
        chunk_type: String,
        content: String,
    },
    decode {
        file: String,
        chunk_type: String,
    },
    remove {
        file: String,
        chunk_type: String,
    },
    print {
        file: String,
    },
}
