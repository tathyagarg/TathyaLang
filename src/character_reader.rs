use {
    std::fs,
    pyo3::prelude::*
};

pub struct IndexingError { message: String }

pub fn format_bool(status: bool) -> &'static str {
    if status {
        "passed"
    } else {
        "failed"
    }
}

#[pyclass]
pub struct Reader<'a> {
    pub file_path: &'a std::path::Path,
    pub contents: String,
}

pub trait ReaderFunctionality {
    fn new(file_path: &std::path::Path) -> Self;
    fn verify_path(&self) -> bool;
    fn initialize(&mut self) -> &mut Self;
    fn peek(&self, position: u16) -> std::result::Result<char, IndexingError>;
    fn consume(&mut self) -> char;
    fn is_eof(&self) -> bool;
    fn file_length(&self) -> usize;
    fn first_instance_of(&self, character: char) -> u16;
}

#[pymethods]
impl Reader {
    #[new]
    fn new(file_path: &path::Path) -> Reader {
        let mut reader = Reader {
            file_path: file_path,
            contents: String::from("")
        };
        reader.initialize();
        return reader;
    }

    fn verify_path(&self) -> bool { self.file_path.exists() }

    fn initialize(&mut self) -> &mut Reader {
        let mut pass_count: u8 = 0;
        let mut check_count: u8 = 0;
        let funcs = std::collections::HashMap::from([
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
        return self;
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
        return popped_char;
    }

    fn is_eof(&self) -> bool {
        match self.peek(0) {
            Ok(_) => false,
            Err(_) => true,
        }
    }

    fn file_length(&self) -> usize { self.contents.chars().count() }
    
    fn first_instance_of(&self, character: char) -> u16 {
        for (i, letter) in self.contents.chars().enumerate() {
            if letter == character { return i.try_into().unwrap() }
        }
        return u16::MAX
    }
}

#[pyclass]
struct ReaderMethodsPy {}

#[pymethods]
impl ReaderMethodsPy {
    fn verify_path(&self, obj: &Reader) -> bool { obj.verify_path() }
    fn initialize(&mut self, obj: &Reader) -> &mut Reader { obj.initialize() }
    fn peek(&self, position: u16, obj: &Reader) -> std::result::Result<char, IndexingError> { obj.peek(position) }
    fn consume(&mut self, obj: &Reader) -> char { obj.consume() }
    fn is_eof(&self, obj: &Reader) -> bool { obj.is_eof() }
    fn file_length(&self, obj: &Reader) -> usize { obj.file_length() }
    fn first_instance_of(&self, character: char, obj: &Reader) -> u16 { obj.first_instance_of(character) }
}

#[pymodule]
fn character_reader_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Reader>()?;
    m.add_class::<ReaderMethodsPy>()?;
    Ok(())
}
