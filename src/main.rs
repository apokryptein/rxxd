use crate::format::{format_line, print_dump_line};
use anyhow::Result;
use clap::Parser;

mod cli;
mod format;
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
        // Get formatted line
        let line = format_line(chunk, &args);

        // Print line
        print_dump_line(address, chunk, &line, &args);

        // Update address
        address += args.cols as u64;
    }

    Ok(())
}
