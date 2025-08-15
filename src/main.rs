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

/// A cat-like CLI tool for colored diff viewing
///
/// litcat is a simple Rust CLI tool that displays patch files with syntax highlighting.
/// It recognizes diff markers and colors added (+), removed (-), and context lines accordingly.
///
/// # Examples
/// litcat diff.patch
/// cat diff.patch | litcat
#[derive(Parser)]
#[command(author, version)]
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
