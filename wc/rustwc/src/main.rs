use std::fs;

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short('c'), long("bytes"), action(ArgAction::SetTrue))]
    bytes: bool,
    #[clap(short('l'), long("lines"), action(ArgAction::SetTrue))]
    lines: bool,
    #[clap(short('m'), long("chars"), action(ArgAction::SetTrue))]
    chars: bool,
    #[clap(short('w'), long("words"), action(ArgAction::SetTrue))]
    words: bool,
    file_name: String,
}

fn show_bytes(content: &String) -> u64 {
    content.len() as u64
}

fn show_word_count(content: &String) -> u64 {
    content.trim().split(' ').count() as u64
}

fn show_char_count(content: &String) -> u64 {
    content.chars().count() as u64
}

fn show_line_count(content: &String) -> u64 {
    content.split_terminator("\n").count() as u64
}


fn main() {
    let args = Args::parse();
    let contents: String;
    let _output: String;

    let can_read = fs::read_to_string(&args.file_name);
    match can_read {
        Ok(value) => {
            contents = value;
        },
        Err(_) => {
            println!("{} not found", &args.file_name);
            std::process::exit(1);
        }
    }
    if args.bytes {
       println!("{}", show_bytes(&contents));
    }
    if args.words {
        println!("{}", show_word_count(&contents))
    }
    if args.chars {
        println!("{}", show_char_count(&contents))
    }
    if args.lines {
        println!("{}", show_line_count(&contents))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_count() {
        let file_path = "./test.txt";
        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(342190, show_bytes(&contents))

    }

    #[test]
    fn test_line_count() {
        let file_path = "./test.txt";
        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(7145, show_line_count(&contents))
    }

    #[test]
    fn test_word_count() {
        let file_path = "./test.txt";
        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(58164, show_word_count(&contents))
    }

    #[test]
    fn test_char_count() {
        let file_path = "./test.txt";
        let contents = fs::read_to_string(file_path).unwrap();

        assert_eq!(339292, show_char_count(&contents))
    }
}
