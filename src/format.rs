use crate::cli::Args;
use colored::Colorize;

/// format_line handles all line chunking and formatting, and returns
/// the line as  String
pub fn format_line(chunk: &[u8], args: &Args) -> String {
    // Chunk bytes according to group size then map to Vec using the desire byte order
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
        .collect::<Vec<String>>() // convert to String vec
        .join(" "); // convert to String

    // Calculate padding needed for consistent alignment
    let hex_chars_per_byte = 2;
    let spaces_between_groups = (args.cols / args.groupsize) - 1;
    let expected_hex_width = (args.cols * hex_chars_per_byte) + spaces_between_groups;

    format!("{hex_string:<expected_hex_width$}")
}

/// get_string converts a byte array to a printable ASCII string
pub fn get_string(byte_array: &[u8]) -> String {
    let build_string_vec: Vec<String> = byte_array
        .iter()
        .map(|num| {
            if *num > 32 && *num <= 126 {
                // if ascii convert to string
                (*num as char).to_string()
            } else {
                // if not ascii use a '.' instead
                '.'.to_string()
            }
        })
        .collect();

    build_string_vec.join("")
}

/// print_dump_line prints one line of a hexdump to stdout
pub fn print_dump_line(address: u64, chunk: &[u8], line: &str, args: &Args) {
    if args.color {
        println!(
            "{}: {} {}",
            format!("{address:08x}").blue(),
            line.white(),
            get_string(chunk).green(),
        );
    } else {
        println!("{:08x}:  {}  {}", address, line, get_string(chunk),);
    }
}
