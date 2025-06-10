use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, stdin};

#[derive(Parser, Debug)]
#[command(
    author = "apokryptein",
    version,
    about = "xxd-like hex dump utility written in Rust"
)]
struct Args {
    // Filename
    #[arg(short, long)]
    filename: Option<String>,

    // Columns
    #[arg(
        short,
        long,
        default_value_t = 16,
        help = "Number of octets (bytes) per line"
    )]
    cols: usize,

    // Group size
    // Default: 2 -> follows xxd standard
    #[arg(
        short,
        long,
        default_value_t = 2,
        help = "Byte groupsize within each line"
    )]
    groupsize: usize,

    // Length -> stop after number of bytes
    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "Number of bytes to read. Defaults to entire input."
    )]
    len: usize,

    // Seek -> starts at offset
    // Default: 0 -> start of input
    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "Start at <seek> bytes abs infile offset."
    )]
    seek: u64,

    // Option for Little Endian byte order
    #[arg(short, long, action=clap::ArgAction::SetTrue, help="Display bytes in Little Endian byte order. [default: Big Endian]")]
    endian: bool,

    // Colorize output
    #[arg(long, action=clap::ArgAction::SetTrue, help="Colorize output")]
    color: bool,
}

fn main() -> Result<()> {
    // Parse args
    let args = Args::parse();

    // Necessary color overried when using colored crate
    if args.color {
        colored::control::set_override(true);
    }

    // Read data from either provided file or stdin into
    // a byte buffer -> Vec<u8>
    let buffer = match &args.filename {
        // If a file has been provided
        Some(filename) => {
            println!("FILE: {}", filename.red());
            let mut file = File::open(filename)?;

            // Handle seek for files:
            // Set file cursor to desired offset
            if args.seek > 0 {
                file.seek(SeekFrom::Start(args.seek))?;
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
            buffer
        }
        None => {
            // Read all data from stdin into memory
            let mut buffer = Vec::new();
            stdin().read_to_end(&mut buffer)?;

            // Get stdin size
            let start = args.seek as usize;

            // Make sure we aren't trying to start after the end of the data
            if start >= buffer.len() {
                Vec::new()
            } else {
                // If length is provided
                let end = if args.len > 0 {
                    // End is the smaller of data length and seek + length
                    std::cmp::min(start + args.len, buffer.len())
                } else {
                    // If no length is provided read to end
                    buffer.len()
                };
                buffer[start..end].to_vec()
            }
        }
    };

    // Track offset address
    let mut address: usize = 0;

    // Construct and print hex according to desired format via
    // cols and groupsize
    for chunk in buffer.chunks(args.cols) {
        let hex_string = chunk
            .chunks(args.groupsize)
            .map(|byte_size| {
                let bytes: Vec<u8> = if args.endian {
                    // If little endian, reverse byte order
                    byte_size.iter().rev().cloned().collect()
                } else {
                    // If not little endian keep ordiginal order (big endian)
                    byte_size.to_vec()
                };

                bytes
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(" ");

        // Calculate padding needed for consistent alignment
        let hex_chars_per_byte = 2;
        let spaces_between_groups = (args.cols / args.groupsize) - 1;
        let expected_hex_width = (args.cols * hex_chars_per_byte) + spaces_between_groups;
        let padded_hex = format!("{:<width$}", hex_string, width = expected_hex_width);

        // Print line
        if args.color {
            println!(
                "{}: {} {}",
                format!("{:08x}", address).blue(),
                padded_hex.white(),
                get_string(chunk).green(),
            );
        } else {
            println!("{:08x}:  {}  {}", address, padded_hex, get_string(chunk),);
        }

        // Update address
        address += args.cols;
    }

    Ok(())
}

// get_string converts a byte array to a printable ASCII string
fn get_string(byte_array: &[u8]) -> String {
    let build_string_vec: Vec<String> = byte_array
        .iter()
        .map(|num| {
            if *num > 32 && *num <= 126 {
                (*num as char).to_string()
            } else {
                '.'.to_string()
            }
        })
        .collect();

    build_string_vec.join("")
}
