use fajt_common::io::PeekReader;
use fajt_lexer::Lexer;
use fajt_parser::ast::Expr;
use fajt_parser::Parser;
use std::collections::HashMap;
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct SnapshotTest {
    input: PathBuf,
    output: Option<PathBuf>,
}

impl SnapshotTest {
    fn run(&self) {
        let result = self.parse_input();
        let expected_result = self.parse_output();

        if let Some(expected) = expected_result {
            assert_eq!(
                expected,
                result,
                "Processing input file: {:?}\n          output file: {:?}",
                self.input,
                self.output.as_ref().unwrap()
            );
        } else {
            let output_path = self.get_output_path();
            let output = File::create(output_path).unwrap();
            serde_json::to_writer_pretty(output, &result).unwrap();
        }
    }

    fn get_output_path(&self) -> PathBuf {
        let input_name = self
            .input
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        let mut output_name = input_name[0..input_name.len() - 3].to_owned();
        output_name.push_str(".json");

        let mut output_path = self.input.clone();
        output_path.set_file_name(output_name);
        output_path
    }

    // TODO should not be Expr
    fn parse_input(&self) -> Expr {
        let input = Self::read_string(&self.input);
        let lexer = Lexer::new(&input).unwrap();
        let mut reader = PeekReader::new(lexer).unwrap();
        let mut parser = Parser::new(&mut reader).unwrap();

        // TODO should not unwrap, we can test errors as well
        parser.parse_expression().unwrap()
    }

    // TODO should not be Expr
    fn parse_output(&self) -> Option<Expr> {
        self.output.as_ref().map(|path| {
            let output = Self::read_string(path);
            serde_json::from_str(&output).unwrap()
        })
    }

    fn read_string(path: &PathBuf) -> String {
        let mut file = File::open(path).expect("Failed to open file.");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read file.");
        data
    }
}


///
/// Concept:
///
/// arbitrary file endings:
///     .js         - syntax                - input
///     .json       - ast                   - output
///     .error.txt  - error output          - error output
///
/// run() -> result<output type>
///     read the output type expected
///     if no output type at all exists, create that output file with data
///

trait SnapshotConverter<T> {
    fn from_string(data: &str) -> T;
}

struct SnapshotTester {
    tests: Vec<SnapshotTest>,
}

impl SnapshotTester {
    fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let tests = Self::discover_tests(path);
        SnapshotTester { tests }
    }

    fn discover_tests<P: AsRef<Path>>(path: P) -> Vec<SnapshotTest> {
        let mut files = Vec::new();
        let mut directories = Vec::new();

        let entries = read_dir(path).expect("Failed to read directory.");
        for entry in entries {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() {
                directories.push(entry);
            } else if file_type.is_file() {
                files.push(entry);
            }
        }

        let mut nested_tests: Vec<SnapshotTest> = directories
            .into_iter()
            .flat_map(|d| Self::discover_tests(d.path()))
            .collect();

        let mut tests = Self::files_to_tests(files);
        tests.append(&mut nested_tests);

        tests
    }

    fn files_to_tests(files: Vec<DirEntry>) -> Vec<SnapshotTest> {
        let mut input_map = HashMap::new();
        let mut output_map = HashMap::new();

        for file in files {
            let name = file.file_name().into_string().unwrap();
            if name.ends_with(".js") {
                input_map.insert(name[0..name.len() - 3].to_owned(), file.path());
            } else if name.ends_with(".json") {
                output_map.insert(name[0..name.len() - 5].to_owned(), file.path());
            }
        }

        input_map
            .into_iter()
            .map(|(name, path)| SnapshotTest {
                input: path,
                output: output_map.remove(&name),
            })
            .collect()
    }

    fn run(&self) {
        for test in &self.tests {
            test.run();
        }
    }
}

#[test]
fn snapshot_test() {
    let mut snapshot_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    snapshot_directory.push("tests/snapshots");

    let tester = SnapshotTester::from_path(snapshot_directory);
    tester.run();
}
