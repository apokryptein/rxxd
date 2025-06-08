use anyhow::{Result, bail};
use clap::Parser;
use colored::Colorize;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Parser, Debug)]
#[command(
    author = "apokryptein",
    version,
    about = "xxd-like hex dump utility written in Rust"
)]
struct Args {
    // Filename
    #[arg(short, long)]
    filename: String,

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

    // Colorize output
    #[arg(long, action=clap::ArgAction::SetTrue)]
    color: bool,
}

fn main() -> Result<()> {
    // Parse args
    let args = Args::parse();

    // Necessary color overried when using colored crate
    if args.color {
        colored::control::set_override(true);
    }

    // Open file
    println!("FILE: {}", args.filename.red());
    let mut file = match File::open(args.filename) {
        Ok(file) => file,
        Err(e) => {
            bail!("File open: {}", e);
        }
    };

    // Get file size
    let meta = file.metadata()?;
    let file_size = meta.len() as usize;

    // Set read length based on args.len and args.seek
    let read_len: usize = match args.len {
        // If no length arg is provided
        0 => {
            // If no seek then starting at 0
            if args.seek == 0 {
                // Read length will be size of input (file)
                file_size
            // If args.seek provided
            } else {
                // Read length is size of file minus args.seek
                file_size - (args.seek as usize)
            }
        }
        // If not 0, then just use provided size
        _ => args.len,
    };

    // Ensure requested read size does not exceed the size of the provided data
    if read_len > file_size {
        bail!("[!] Cannot specify a read size larger than the file");
    }

    // Read file from args.seek offset (default of 0)
    file.seek(SeekFrom::Start(args.seek))?;
    let mut buffer = vec![0; read_len];
    file.read_exact(&mut buffer)?;

    // Track offset address
    let mut address: usize = 0;

    // Construct and print hex according to desired format via
    // cols and groupsize
    for chunk in buffer.chunks(args.cols) {
        let hex_string = chunk
            .chunks(args.groupsize)
            .map(|byte_size| {
                byte_size
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
