use clap::Parser;
use std::process;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use args::{Cli, PngMeArgs};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        PngMeArgs::Encode(args) => commands::encode(args),
        PngMeArgs::Decode(args) => commands::decode(args),
        PngMeArgs::Remove(args) => commands::remove(args),
        PngMeArgs::Print(args) => commands::print_chunks(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}