use {
    std::{env, path, collections},
    pyo3::prelude::*
};
mod character_reader;

include!("character_reader.rs");

#[pyfunction]
fn main() {
    let cli: Vec<String> = env::args().collect();
    let file_path = match cli.len() {
        x if x > 1 => &cli[1],
        _ => return println!("Error: File not specified")
    };
    let reader = character_reader::Reader::new(&path::Path::new(file_path));
    let mut empty_vec: Vec<&collections::HashMap<&str, &str>> = Vec::new();
}

#[pymodule]
fn TathyaLang(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}

