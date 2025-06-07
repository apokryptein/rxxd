use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // Filename
    #[arg(short, long)]
    filename: String,
}

fn main() -> Result<()> {
    // Parse args
    let args = Args::parse();

    // Open file
    println!("Opening: {}", args.filename);
    let mut file = File::open(args.filename)?;

    // Read file into contents
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Output results
    println!("{}", contents);

    Ok(())
}
