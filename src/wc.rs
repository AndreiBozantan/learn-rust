use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

/// Implements the wc command, which counts the number of words in a file.
pub fn main() {
    let mut total = 0;
    let args: std::env::Args = std::env::args();
    if args.len() == 1 {
        total += process_file("-"); // if no file names are provided, use stdin
    } else {
        for file_name in args.skip(1) {
            total += process_file(&file_name);
        }
    }
    print!("Total: {total} words");
}

fn process_file(file_name: &str) -> u64 {
    let reader = match file_name {
        "-" => Some(Box::new(io::stdin().lock()) as Box<dyn BufRead>),
        _ => File::open(&file_name)
                .inspect_err(|err| eprintln!("{file_name}: ERROR file open ({err})"))
                .map(|file| Box::new(BufReader::new(file)) as Box<dyn BufRead>)
                .ok()
    };

    let mut result = 0;
    if let Some(mut reader) = reader {
        let file_name = if file_name == "-" { "stdin" } else { file_name };
        match word_count(&mut reader) {
            Ok(count) => { println!("{file_name}: {count} words"); result = count; }
            Err(err) => { eprintln!("{file_name}: ERROR file read ({err})"); }
        }
    }
    result
}

/// Process the contents of the file, character by character
fn word_count(reader: &mut dyn BufRead) -> Result<u64, std::io::Error> {
    let mut count = 0;
    let mut in_word = false;

    // Read file character by character
    for byte in reader.bytes() {
        let byte = byte?;
        if byte.is_ascii_whitespace() || byte.is_ascii_punctuation() {
            if in_word {
                count += 1;
                in_word = false;
            }
        } else {
            in_word = true;
        }
    }

    // If the reader ends while still in a word, count that word
    if in_word {
        count += 1;
    }

    Ok(count)
}

