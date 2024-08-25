use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    name: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    
    // Debug info
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    Keys {
        #[clap(subcommand)]
        subcommands: KeysSubcommands,
    },
    Config {
        #[arg(long)]
        keys_path: PathBuf,
    },
    Encrypt {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        key: String,
    },
    Decrypt {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        key: String,
    }
}

#[derive(Subcommand)]
pub enum KeysSubcommands {
    Generate {
        #[arg(short, long)]
        name: String,

        #[arg(long)]
        width: usize,

        #[arg(long)]
        height: usize,
    },
}