use std::path::{PathBuf, Path};
use std::collections::HashMap;

extern crate fajt_macros;
use fajt_testing::snapshot_tests;
use fajt_lexer::Lexer;
use fajt_common::io::PeekReader;
use fajt_parser::Parser;
use fajt_parser::ast::Expr;
use std::fs::File;
use std::io::Read;
use std::fmt::Display;

fn read_string<P>(path: &P) -> String where P: AsRef<Path> {
    let mut file = File::open(path).expect("Failed to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read file.");
    data
}

fn snapshot_runner<P>(test_file: P, result_files: HashMap<&str, P>) where P: AsRef<Path> + Display {
    println!("Parsing {}", test_file);

    let input = read_string(&test_file);
    let lexer = Lexer::new(&input).unwrap();
    let mut reader = PeekReader::new(lexer).unwrap();
    let mut parser = Parser::new(&mut reader).unwrap();
    let result = parser.parse_expression().unwrap();

    if result_files.is_empty() {
        println!("Generating ast file...");
        let mut output_path = PathBuf::from(test_file.to_string());
        output_path.set_extension("json");

        let out_file = File::create(output_path).unwrap();
        serde_json::to_writer_pretty(out_file, &result).unwrap();
    }

    let path = result_files.get("json").unwrap();
    let expected_json = read_string(path);
    let expected_expr: Expr = serde_json::from_str(&expected_json).unwrap();


    assert_eq!(result, expected_expr)
}

snapshot_tests!(
    dirs: ["parser/tests/snapshots"],
    test_endings: ["js"],
    result_endings: ["json"],
    runner: snapshot_runner,
);