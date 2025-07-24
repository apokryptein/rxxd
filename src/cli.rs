use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "apokryptein",
    version,
    about = "xxd-like hexdump utility written in Rust"
)]
pub struct Args {
    /// Filename
    #[arg(short, long)]
    pub filename: Option<String>,

    /// Number of columns per row
    #[arg(
        short,
        long,
        default_value_t = 16,
        help = "Number of octets (bytes) per line"
    )]
    pub cols: usize,

    /// Byte group size
    #[arg(
        short,
        long,
        default_value_t = 2,
        help = "Byte groupsize within each line"
    )]
    pub groupsize: usize,

    /// Number of bytes to read
    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "Number of bytes to read. Defaults to entire input."
    )]
    pub len: usize,

    /// Start reading at offset
    #[arg(short, long, help = "Start at <seek> bytes abs infile offset.")]
    pub seek: Option<u64>,

    /// Flip byte order
    #[arg(short, long, action=clap::ArgAction::SetTrue, help="Display bytes in Little Endian byte order. [default: Big Endian]")]
    pub endian: bool,

    /// Colorize output
    #[arg(long, action=clap::ArgAction::SetTrue, help="Colorize output")]
    pub color: bool,
}
