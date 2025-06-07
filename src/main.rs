use anyhow::{Result, bail};
use clap::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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
    let mut file = match File::open(args.filename) {
        Ok(file) => file,
        Err(e) => {
            bail!("File open: {}", e);
        }
    };

    // Get file length
    let meta = file.metadata()?;
    let file_len = meta.len() as usize;

    // Read file
    // Leaving start at 0 for now and end at end of file
    // Will change this to values specified from CLI
    file.seek(SeekFrom::Start(0))?;
    let mut buffer = vec![0; file_len];
    file.read_exact(&mut buffer)?;

    // Output results
    println!("{:?}", buffer);

    Ok(())
}
