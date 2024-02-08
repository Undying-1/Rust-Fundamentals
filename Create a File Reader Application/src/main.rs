use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

fn main() {
    // Open the file in read-write mode
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let file = File::open(input.trim());
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }


}