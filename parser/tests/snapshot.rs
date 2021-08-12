extern crate fajt_macros;

use fajt_common::io::PeekReader;
use fajt_lexer::Lexer;
use fajt_macros::for_each_file;
use fajt_parser::ast::Expr;
use fajt_parser::Parser;

// TODO handle and assert errors
// TODO expr vs stmt vs program (different folders?)
fn parse_js_string(data: &str) -> Expr {
    let lexer = Lexer::new(&data).unwrap();
    let mut reader = PeekReader::new(lexer).unwrap();
    let mut parser = Parser::new(&mut reader).unwrap();
    parser.parse_expression().unwrap()
}

fn snapshot_runner(test_file: &str) {
    println!("Parsing {}", test_file);

    let markdown = md::Markdown::from_file(test_file.as_ref());
    let result = parse_js_string(&markdown.js_block);

    if let Some(expected_data) = &markdown.json_block {
        let expected_expr: Expr = serde_json::from_str(&expected_data).unwrap();
        assert_eq!(result, expected_expr)
    } else {
        let json = serde_json::to_string_pretty(&result).unwrap();
        markdown.append_json_block(&json);
        panic!("No ast found in this test. Json generated, verify and rerun.");
    }
}

macro_rules! generate_test_case {
    ("md", $file_path:literal, $ident:ident) => {
        #[test]
        fn $ident() {
            snapshot_runner($file_path)
        }
    };
    ($extension:literal, $file_path:literal, $ident:ident) => {
        // Unknown file extensions, ignore...
    };
}

for_each_file!("parser/tests/snapshots", generate_test_case);

mod md {
    use std::fs::{File, OpenOptions};
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::{Path, PathBuf};

    pub struct Markdown {
        path: PathBuf,
        pub js_block: String,
        pub json_block: Option<String>,
    }

    impl Markdown {
        pub fn from_file(path: &Path) -> Self {
            let data = read_string(path);

            let js_block = get_code_block(&data, "js")
                .expect("JS input required.")
                .to_owned();
            let json_block = get_code_block(&data, "json").map(&str::to_owned);
            Markdown {
                path: PathBuf::from(path),
                js_block,
                json_block,
            }
        }

        pub fn append_json_block(&self, data: &str) {
            let block = generate_code_block(&data, "json");

            let mut file = OpenOptions::new().write(true).open(&self.path).unwrap();

            file.seek(SeekFrom::End(0)).unwrap();
            file.write_all("\n\n".as_bytes()).unwrap();
            file.write_all(block.as_bytes()).unwrap();
        }
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
        format!(
            "{}{}\n{}\n{}\n",
            BLOCK_DELIMITER, annotation, data, BLOCK_DELIMITER
        )
    }

    fn read_string(path: &Path) -> String {
        let mut file = File::open(path).expect("Failed to open file.");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read file.");
        data
    }
}
