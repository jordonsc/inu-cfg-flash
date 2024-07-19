use std::fs::File;
use std::io::{Read, Write};
use std::mem;
use std::process::ExitCode;

use clap::Parser;
use colored::Colorize;

// Data-type for size-of-data header
type HeaderType = u32;
// The size of each header item
const SIZE_LEN: usize = mem::size_of::<HeaderType>();
const MD5_LEN: usize = 16;
// Total header len
const HEADER_LEN: usize = SIZE_LEN + MD5_LEN;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to settings JSON file
    #[arg(short, long)]
    input: String,

    /// Path to binary output
    #[arg(short, long, default_value = "settings.bin")]
    output: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    println!("Input file:    {}", args.input.yellow());
    println!("Output file:   {}", args.output.yellow());

    let file = File::open(args.input);
    if let Err(e) = file {
        println!("Failed to open settings file: {}", e.to_string().red());
        return ExitCode::FAILURE;
    }

    let mut data = vec![];
    match file.unwrap().read_to_end(&mut data) {
        Ok(s) => {
            println!("Data size:     {}", s.to_string().yellow());
            println!("Payload size:  {}", (s + HEADER_LEN).to_string().yellow());
        }
        Err(e) => {
            println!("Failed to read data from settings file: {}", e.to_string().red());
            return ExitCode::FAILURE;
        }
    }

    let mut bin = vec![];
    // Add size of data to the first 4 bytes
    bin.extend_from_slice(&(data.len() as HeaderType).to_le_bytes());

    // Add MD5 hash of data
    let md5 = md5::compute(&data);
    bin.extend_from_slice(&md5.0);
    // Add the data itself
    bin.extend_from_slice(&data);
    assert_eq!(bin.len(), data.len() + HEADER_LEN);

    match File::create(args.output) {
        Ok(mut f) => {
            match f.write_all(&bin) {
                Ok(_) => println!("{}", "Data written to output file.".green()),
                Err(e) => {
                    println!("Failed to write data to output file: {}", e.to_string().red());
                    return ExitCode::FAILURE;
                }
            }
        }
        Err(e) => {
            println!("Failed to create output file: {}", e.to_string().red());
            return ExitCode::FAILURE;
        }
    }


    ExitCode::SUCCESS
}
