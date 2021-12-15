extern crate fajt_macros;

use fajt_codegen::generate_code;
use fajt_macros::for_each_file;
use fajt_parser::parse_program;
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read file.");
    data
}

fn run_test(file_path: &str) {
    let input = read_file(file_path);
    let ast = parse_program(&input).unwrap();
    let output = generate_code(ast);

    assert_eq!(output, input);
}

macro_rules! generate_test_cases {
    ("js", $file_path:literal, $ident:ident) => {
        #[test]
        fn $ident() {
            run_test($file_path);
        }
    };
}

for_each_file!("tests/js", generate_test_cases);

#[test]
fn dummy() {
    // This is just so IDE recognize this is a runnable file.
}
