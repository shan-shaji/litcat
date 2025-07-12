use clap::Parser;
use colored::Colorize;
use std::{
    fs::File,
    io::{BufRead, BufReader, stdin},
};

enum InputSource {
    File(BufReader<File>),
    Stdin(BufReader<std::io::Stdin>),
}

#[derive(Parser)]
#[command (author, version, about, long_about = None)]
struct Args {
    /// Path to the file to display. If not provided, reads from stdin.
    file: Option<String>,
}

fn process_lines<R: BufRead>(reader: R) -> std::io::Result<()> {
    for line_result in reader.lines() {
        let line = line_result?;
        if line.starts_with("+++ ") || line.starts_with("--- ") {
            println!("{}", line.cyan());
        } else if line.starts_with("@@") {
            println!("{}", line.yellow());
        } else if line.starts_with('+') && !line.starts_with("+++") {
            println!("{}", line.green());
        } else if line.starts_with('-') && !line.starts_with("---") {
            println!("{}", line.red());
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let input = match args.file {
        Some(path) => InputSource::File(BufReader::new(File::open(path)?)),
        None => InputSource::Stdin(BufReader::new(stdin())),
    };

    match input {
        InputSource::File(reader) => process_lines(reader),
        InputSource::Stdin(reader) => process_lines(reader),
    }
}
