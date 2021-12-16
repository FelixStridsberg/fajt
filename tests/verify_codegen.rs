extern crate fajt_macros;

mod markdown;

use fajt_codegen::generate_code;
use fajt_parser::parse_program;
use markdown::TestFile;
use std::path::Path;

fn run_test_file(filename: &str) {
    let path = Path::new(filename);
    if path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .starts_with("fail-")
    {
        println!(
            "Skipping {}, cannot generate code from failing source.",
            filename
        );
        return;
    }

    println!("Running: {}", filename);

    let test_file = TestFile::from(&filename);
    let input = test_file.source;
    let ast = parse_program(&input).unwrap();
    let output = generate_code(ast);

    assert_eq!(output, input);
}

macro_rules! generate_test_cases {
    ("md", $file_path:literal, $ident:ident) => {
        #[test]
        fn $ident() {
            $crate::run_test_file($file_path)
        }
    };
    ("md_ignore", $file_path:literal, $ident:ident) => {
        #[ignore]
        #[test]
        fn $ident() {
            $crate::run_test_file($file_path)
        }
    };
    ($extension:literal, $file_path:literal, $ident:ident) => {};
}

macro_rules! generate_test_module {
    (
        mod_name: $mod_name:ident,
        ast_type: $ast_type:ident,
        source_type: $source_type:ident,
        folders: [$( $folder:literal ),*],
    ) => {
        mod $mod_name {
            use fajt_macros::for_each_file;

            $(
                for_each_file!($folder, generate_test_cases);
            )*
        }
    }
}

generate_test_module!(
    mod_name: decl,
    ast_type: Stmt,
    source_type: Script,
    folders: ["tests/cases/decl"],
);

#[test]
fn dummy() {
    // This is just so IDE recognize this is a runnable file.
}
