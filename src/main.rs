use clap::Parser;
use colored::Colorize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser)]
#[command (author, version, about, long_about = None)]
struct Args {
    /// Path to the file to display
    file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.file)?;
    let _reader = BufReader::new(file);

    for line_result in _reader.lines() {
        let line = line_result?;
        if line.starts_with('+') && !line.starts_with("+++") {
            println!("{}", line.green());
        } else if line.starts_with('-') && !line.starts_with("---") {
            println!("{}", line.red());
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}
