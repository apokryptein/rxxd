use crate::cli::Args;
use anyhow::{Result, anyhow};
use colored::Colorize;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, stdin};

/// read_input checks if a file was provided in the CLI and will read it into a Vec<u8>
/// If a file is not provided, it will read from STDIN
pub fn read_input(args: &Args, seek: u64) -> Result<Vec<u8>> {
    match &args.filename {
        // If a file has been provided
        Some(filename) => {
            println!("FILE: {}", filename.red());

            // Read data from file
            read_from_file(filename, seek, args.len)
        }
        None => read_from_stdin(seek, args.len),
    }
}

/// read_from_file reads data from a file given seek and number of bytes to read
fn read_from_file(filename: &str, seek: u64, length: usize) -> Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let file_size = file.metadata()?.len();

    // Ensure seek isn't beyond the end of the file
    if seek > file_size {
        return Err(anyhow!("[ERR] seek value is beyond end of file"));
    }

    // Handle seek for files:
    // Set file cursor to desired offset
    if seek > 0 {
        file.seek(SeekFrom::Start(seek))?;
    }

    // Calculate how much data we can read
    let bytes_available = file_size - seek;
    let bytes_to_read = if length > 0 {
        std::cmp::min(length as u64, bytes_available) as usize
    } else {
        bytes_available as usize
    };

    // Instantiate the appropriately size vec
    let mut buffer = vec![0u8; bytes_to_read];

    // Read exact number of bytes
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

/// read_from_stdin reads bytes from stdin given seek and number of bytes to read
fn read_from_stdin(seek: u64, length: usize) -> Result<Vec<u8>> {
    // Read all data from stdin into memory
    let mut buffer = Vec::new();
    stdin().read_to_end(&mut buffer)?;

    // Get stdin size
    let start = seek as usize;

    // Make sure we aren't trying to start after the end of the data
    if start >= buffer.len() {
        Ok(Vec::new())
    } else {
        // If length is provided
        let end = if length > 0 {
            // End is the smaller of data length and seek + length
            std::cmp::min(start + length, buffer.len())
        } else {
            // If no length is provided read to end
            buffer.len()
        };
        Ok(buffer[start..end].to_vec())
    }
}
