use std::{env, path};
mod character_reader;
use character_reader::ReaderFuncs;

fn main() {
    let cli: Vec<String> = env::args().collect();
    let file_path = match cli.len() {
        x if x > 1 => &cli[1],
        _ => return println!("Error: File not specified")
    };
    let tokens: Vec<String> = Vec::new();
    let mut reader = character_reader::Reader::new(path::Path::new(file_path));
    for _ in 0..5 {
        println!("{}", reader.consume());
    }
}

