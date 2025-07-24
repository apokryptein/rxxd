use anyhow::Result;
use clap::Parser;
use colored::Colorize;

mod cli;
mod input;

fn main() -> Result<()> {
    // Parse args
    let args = cli::Args::parse();

    // Necessary color overried when using colored crate
    if args.color {
        colored::control::set_override(true);
    }

    // Track offset address
    let mut address = args.seek.unwrap_or_default();

    // Read input
    let buffer = input::read_input(&args, address)?;

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
                    .map(|b| format!("{b:02x}"))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(" ");

        // Calculate padding needed for consistent alignment
        let hex_chars_per_byte = 2;
        let spaces_between_groups = (args.cols / args.groupsize) - 1;
        let expected_hex_width = (args.cols * hex_chars_per_byte) + spaces_between_groups;
        let padded_hex = format!("{hex_string:<expected_hex_width$}");

        // Print line
        if args.color {
            println!(
                "{}: {} {}",
                format!("{address:08x}").blue(),
                padded_hex.white(),
                get_string(chunk).green(),
            );
        } else {
            println!("{:08x}:  {}  {}", address, padded_hex, get_string(chunk),);
        }

        // Update address
        address += args.cols as u64;
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
