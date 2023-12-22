use std::{path, fs, collections};

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
    fn peek(&self, position: u16) -> char;
    fn consume(&mut self, position: u16) -> char;
    fn is_eof(&self) -> bool;
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

    fn peek(&self, position: u16) -> char {
        self.contents.chars().nth(position as usize).expect("Position not found")
    }

    fn consume(&mut self, position: u16) -> char {
        // TODO
        'a'
    }

    fn is_eof(&self) -> bool {
        // TODO
        false
    }
}
