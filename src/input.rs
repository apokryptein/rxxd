use crate::cli::Args;
use anyhow::Result;
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
            let mut file = File::open(filename)?;

            // Handle seek for files:
            // Set file cursor to desired offset
            if seek > 0 {
                file.seek(SeekFrom::Start(seek))?;
            }

            let mut buffer = Vec::new();

            // If reading specific length
            if args.len > 0 {
                // Using a temporary buffer to read entire length specificied
                // by args.len, then truncating to the actual size read. I am doing this to avoid
                // sizing a buffer that exceeds the data length
                let mut temp_buffer = Vec::new();
                //let bytes_read = file.read_to_end(&mut temp_buffer)?;
                file.read_to_end(&mut temp_buffer)?;

                // Take just args.len bytes
                buffer = temp_buffer[..args.len].to_vec();

            // Otherwise read to end of file
            } else {
                file.read_to_end(&mut buffer)?;
            }
            Ok(buffer)
        }
        None => {
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
                let end = if args.len > 0 {
                    // End is the smaller of data length and seek + length
                    std::cmp::min(start + args.len, buffer.len())
                } else {
                    // If no length is provided read to end
                    buffer.len()
                };
                Ok(buffer[start..end].to_vec())
            }
        }
    }
}
