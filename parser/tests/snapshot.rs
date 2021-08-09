use std::path::Path;
use std::collections::HashMap;

extern crate fajt_macros;
use fajt_testing::snapshot_tests;
use fajt_lexer::Lexer;
use fajt_common::io::PeekReader;
use fajt_parser::Parser;
use fajt_parser::ast::Expr;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::fmt::Display;

fn read_string<P>(path: &P) -> String where P: AsRef<Path> {
    let mut file = File::open(path).expect("Failed to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read file.");
    data
}

const BLOCK_DELIMITER: &str = "```";

fn get_code_block<'a>(source: &'a str, annotation: &str) -> Option<&'a str> {
    let block_start = format!("{}{}\n", BLOCK_DELIMITER, annotation);
    if let Some(start) = source.find(&block_start) {
        // Block start without preceding new line is only valid if block starts at first line.
        if start != 0 && &source[start - 1..start] != "\n" {
            return None;
        }

        // The data starts after the start pattern.
        let start = start + block_start.len();
        (&source[start..]).find(BLOCK_DELIMITER).map(|end| {
            // \n``` without a new line after is only valid if file ends
            &source[start..end + start]
        })
    } else {
        None
    }
}

fn generate_code_block(data: &str, annotation: &str) -> String {
    format!("{}{}\n{}\n{}\n", BLOCK_DELIMITER, annotation, data, BLOCK_DELIMITER)
}

fn snapshot_runner<P>(test_file: P, _result_files: HashMap<&str, P>) where P: AsRef<Path> + Display {
    println!("Parsing {}", test_file);

    let input = read_string(&test_file);

    let js_data = get_code_block(&input, "js").expect("JS input required.");
    let json_data = get_code_block(&input, "json");

    let lexer = Lexer::new(&js_data).unwrap();
    let mut reader = PeekReader::new(lexer).unwrap();
    let mut parser = Parser::new(&mut reader).unwrap();
    let result = parser.parse_expression().unwrap();

    if let Some(data) = json_data {
        let expected_expr: Expr = serde_json::from_str(data).unwrap();
        assert_eq!(result, expected_expr)
    } else {
        let json = serde_json::to_string_pretty(&result).unwrap();
        let json_block = generate_code_block(&json, "json");

        let mut file = OpenOptions::new()
            .write(true)
            .open(test_file).unwrap();
        file.seek(SeekFrom::End(0)).unwrap();
        file.write_all("\n\n".as_bytes()).unwrap();
        file.write_all(json_block.as_bytes()).unwrap();

        panic!("No ast found in this test. Json generated, verify and rerun.");
    }
}

snapshot_tests!(
    dirs: ["parser/tests/snapshots"],
    test_endings: ["md"],
    result_endings: ["json"],
    runner: snapshot_runner,
);