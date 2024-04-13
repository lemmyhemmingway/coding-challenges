
use std::fs;
use std::io::{self, Read};

use clap::Parser;
#[derive(Parser)]
#[command(version, about)]
struct Cli {

    filename: Option<String>,

    #[arg(short('c'), value_name = "BYTES", action = clap::ArgAction::SetTrue)]
    bytes: bool,

    #[arg(short('l'), value_name = "LINES", action = clap::ArgAction::SetTrue)]
    lines: bool,

    #[arg(short('w'), value_name = "WORDS", action = clap::ArgAction::SetTrue)]
    words: bool,

    #[arg(short('m'), value_name = "CHARACTERS", action = clap::ArgAction::SetTrue)]
    chars: bool,
}

fn read_file_contents(filename: &str) -> String {
    let content = fs::read_to_string(&filename).expect("Couldn't read file");
    content
}

fn read_stdin_contents() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
    buffer
}


fn main() {
    
    let cli = Cli::parse();
    let mut contents = String::new();

    if let Some(filename) = cli.filename {
        contents = read_file_contents(&filename);
    } else {
        contents = read_stdin_contents();
    }

    if cli.bytes {
        print!("{} ", contents.clone().into_bytes().len());
    }

    if cli.lines {
        print!("{} ", contents.clone().lines().count());
    }

    if cli.words {
        print!("{} ", contents.clone().split_whitespace().count());
    }

    if cli.chars {
        print!("{} ", contents.clone().chars().count());
    }


}
