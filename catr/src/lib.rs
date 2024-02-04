use anyhow::Result;
use clap::Parser;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(default_value = "-", help = "Input files")]
    files: Vec<String>,

    #[arg(short = 'n', help = "Number lines")]
    number_lines: bool,

    #[arg(short = 'b', help = "Number nonblock lines")]
    number_nonblank_lines: bool,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    for file_name in args.files {
        match open(&file_name) {
            Err(err) => eprintln!("Failed to open {}: {}", file_name, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if args.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(file: &str) -> Result<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
