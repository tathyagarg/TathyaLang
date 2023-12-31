use std::{path, fs, collections};

struct IndexingError { message: String }

pub fn format_bool(status: bool) -> &'static str {
    if status {
        "passed"
    } else {
        "failed"
    }
}

pub struct Reader<'a> {
    file_path: &'a path::Path,
    contents: String,
}

pub trait ReaderFuncs<'a> {
    fn new(file_path: &'a path::Path) -> Self;
    fn verify_path(&self) -> bool;
    fn initialize(&mut self) -> &mut Self;
    fn peek(&self, position: u16) -> std::result::Result<char, IndexingError>;
    fn consume(&mut self) -> char;
    fn is_eof(&self) -> bool;
    fn file_length(&self) -> usize;
}

impl<'a> ReaderFuncs<'a> for Reader<'a> {
    fn new(file_path: &'a path::Path) -> Reader<'a> {
        let mut reader = Reader {
            file_path: file_path,
            contents: String::from("")
        };
        reader.initialize();
        reader
    }

    fn verify_path(&self) -> bool {
        self.file_path.exists()
    }

    fn initialize(&mut self) -> &mut Reader<'a> {
        let mut pass_count: u8 = 0;
        let mut check_count: u8 = 0;
        let funcs = collections::HashMap::from([
            ("File path verification", self.verify_path())
        ]);
        for (message, status) in funcs {
            println!("{message}: {}", format_bool(status));
            if status { pass_count += 1; }
            check_count += 1;
        }
        println!("Passed {pass_count} checks out of {check_count}");

        let text: Vec<u8> = fs::read(self.file_path).expect("Error!");
        self.contents = String::from_utf8(text).expect("Error!");
        self
    }

    fn peek(&self, position: u16) -> std::result::Result<char, IndexingError> {
        let length: u16 = self.contents.chars().count() as u16;

        match position {
            n if (length..).contains(&n) => Err(IndexingError { message: String::from("Querying position more than contents length") }),
            _ => Ok(self.contents.chars().nth(position as usize).expect("Position not found")),
        }
    }

    fn consume(&mut self) -> char {
        let mut char_contents: std::str::Chars = self.contents.chars();
        let popped_char: char = char_contents.next().expect("Error");
        self.contents = String::from(char_contents.as_str());
        popped_char
    }

    fn is_eof(&self) -> bool {
        match self.peek(0) {
            Ok(_) => false,
            Err(_) => true,
        }
    }

    fn file_length(&self) -> usize { self.contents.chars().count() }
}
