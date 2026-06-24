use std::io::{self, BufRead, BufReader};
use std::fs::File;

use clap::Parser;
use anyhow::Result;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {

    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short('n'), 
        long,
        value_name = "LINES", 
        default_value = "10",  
        conflicts_with = "bytes",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    lines: u64, 

    /// Number of bytes
    #[arg(
        short('c'), 
        long,
        value_name = "BYTES",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                println!("Opened {filename}");
                let mut line = String::new();
                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes = file.read(&mut buffer)?;
                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..bytes])
                    );
                }
                else {
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    } 
}
