use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub mod markdown;

pub fn write_string(path: &Path, string: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
    file.write_all(string.as_bytes()).unwrap();
    file.write_all("\n".as_bytes()).unwrap();
}

pub fn read_string(path: &Path) -> String {
    let mut file = File::open(path).expect("Failed to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read file.");
    data
}
