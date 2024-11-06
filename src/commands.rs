use std::fs;
use std::str::FromStr;

use crate::Result;
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;
use crate::png::Png;

pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    
    png.append_chunk(chunk);

    let output_path = args.output_file.unwrap_or(args.file_path);
    fs::write(output_path, png.as_bytes())?;
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;
    
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        let message = chunk.data_as_string()?;
        println!("Hidden message: {}", message);
        Ok(())
    } else {
        Err(format!("Chunk type {} not found", args.chunk_type).into())
    }
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    
    let chunk = png.remove_first_chunk(&args.chunk_type)?;
    println!("Removed chunk: {}", chunk);
    
    fs::write(&args.file_path, png.as_bytes())?;
    Ok(())
}

pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;
    println!("{}", png);
    Ok(())
}