use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    #[arg(short, long)]
    pub output_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
