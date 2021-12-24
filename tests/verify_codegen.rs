extern crate fajt_macros;

mod markdown;

use fajt_codegen::{generate_code, GeneratorContext};
use fajt_parser::parse_program;
use markdown::TestFile;

fn run_test_file(filename: &str) {
    println!("Running: {}", filename);

    let test_file = TestFile::from(&filename);
    let input = test_file.source;
    let input_min = test_file.source_min;
    let mut ast = parse_program(&input).unwrap();

    let output = generate_code(&mut ast, GeneratorContext::new());

    assert_eq!(output, input);

    if let Some(input_min) = input_min {
        let mut ctx = GeneratorContext::new();
        ctx.minified = true;

        let output_min = generate_code(&mut ast, ctx);

        assert_eq!(output_min, input_min.trim(), "Minified output mismatch.");
    }
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
    source_type: Script,
    folders: ["tests/cases/decl"],
);

generate_test_module!(
    mod_name: stmt,
    source_type: Script,
    folders: ["tests/cases/stmt"],
);

generate_test_module!(
    mod_name: expr,
    source_type: Script,
    folders: ["tests/cases/expr"],
);

#[test]
fn dummy() {
    // This is just so IDE recognize this is a runnable file.
}
