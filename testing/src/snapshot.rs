use std::collections::HashMap;
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub trait TestRunner {
    type TestResult;

    /// Run the test case and return result.
    fn run_test(&self, input: &str) -> Self::TestResult;

    /// Interpret result by comparing to the appropriate file.
    fn interpret_result(&self, result: Self::TestResult, files: &HashMap<String, PathBuf>) -> Option<bool>;

    /// Generate result and output to file.
    fn generate_result(&self, result: Self::TestResult, files: &HashMap<String, PathBuf>);

    fn read_string(path: &PathBuf) -> String {
        let mut file = File::open(path).expect("Failed to open file.");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read file.");
        data
    }
}

#[derive(Debug)]
pub struct TestCase {
    pub name: String,
    pub test_file: String,
    pub result_files: HashMap<String, String>,
}

#[derive(Debug)]
pub struct TestSuite {
    pub test_cases: Vec<TestCase>,
}

pub struct TestSuiteLoader {
    test_endings: Vec<String>,
    result_endings: Vec<String>,
    test_cases: Vec<TestCase>,
}

impl TestSuiteLoader {
    pub fn new(
        mut test_endings: Vec<String>,
        mut result_endings: Vec<String>,
    ) -> TestSuiteLoader {
        // Sort by number of dots so that '.error.json' comes before '.json'.
        let dot_cmp = |a: &String, b: &String| b.matches('.').count().cmp(&a.matches('.').count());
        test_endings.sort_by(dot_cmp);
        result_endings.sort_by(dot_cmp);

        TestSuiteLoader {
            test_endings,
            result_endings,
            test_cases: Vec::new(),
        }
    }

    pub fn build(self) -> TestSuite {
        TestSuite {
            test_cases: self.test_cases,
        }
    }

    pub fn load_dir<P: AsRef<Path>>(&mut self, path: P) -> &Self {
        let mut tests = self.discover_tests(path);
        self.test_cases.append(&mut tests);
        self
    }

    fn discover_tests<P: AsRef<Path>>(&self, path: P) -> Vec<TestCase> {
        let (files, dirs) = self.read_files_and_dirs(path);

        let mut nested_tests: Vec<TestCase> = dirs
            .into_iter()
            .flat_map(|d| self.discover_tests(d.path()))
            .collect();

        let mut tests = self.files_to_tests(files);
        tests.append(&mut nested_tests);

        tests
    }

    fn files_to_tests(&self, files: Vec<DirEntry>) -> Vec<TestCase> {
        let mut test_files = HashMap::new();
        let mut result_files = HashMap::new();

        for file in files {
            let name = file.file_name().into_string().unwrap();
            if let Some(ext) = self.test_endings.iter().find(|&ext| name.ends_with(ext)) {
                let test_name = name[0..name.len() - ext.len()].to_string();
                test_files.insert(test_name, file.path().clone().into_os_string().into_string().unwrap());
            }

            if let Some(ext) = self.result_endings.iter().find(|&ext| name.ends_with(ext)) {
                let test_name = name[0..name.len() - ext.len()].to_string();
                result_files
                    .entry(test_name)
                    .or_insert(HashMap::new())
                    .insert(ext.to_string(), file.path().clone().into_os_string().into_string().unwrap());
            }
        }

        let test_cases = test_files
            .into_iter()
            .map(|(name, path)| TestCase {
                name: name.to_owned(),
                test_file: path,
                result_files: result_files.remove(&name).unwrap_or_else(|| HashMap::new()),
            })
            .collect();

        if !result_files.is_empty() {
            let orphaned_result_files: Vec<String> = result_files
                .into_iter()
                .flat_map(|(_, path)| path.into_iter().map(|a| a.1))
                .collect();
            eprintln!(
                "Warning, found result files without test files {:#?}",
                orphaned_result_files
            );
        }

        test_cases
    }

    fn read_files_and_dirs<P: AsRef<Path>>(&self, path: P) -> (Vec<DirEntry>, Vec<DirEntry>) {
        let mut files = Vec::new();
        let mut dirs = Vec::new();

        let entries = read_dir(path).expect("Failed to read directory.");
        for entry in entries {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() {
                dirs.push(entry);
            } else if file_type.is_file() {
                files.push(entry);
            }
        }

        (files, dirs)
    }
}

impl TestSuite {
    pub fn loader(
        test_endings: &[String],
        result_endings: &[String],
    ) -> TestSuiteLoader {
        TestSuiteLoader::new(test_endings.to_owned(), result_endings.to_owned())
    }
}