//! This generates test cases for all markdown files in ./cases
//!
//! The markdown are expected to have the format:
//!     ### Source
//!     ```js
//!     var a = 1;
//!     ```
//!
//!     ### Output: minified
//!     ```js
//!     var a=1
//!     ```
//!
//!     ### Output: ast
//!     ```json
//!     {
//!         ...
//!     }
//!     ```
//!
//! The "Output" sections are optional. If you add the title of a section and run the test the
//! actual output will be generated.
//!
//! For example, add a file with:
//!     ### Source
//!     ```js
//!     var a = 1;
//!     ```
//!
//!     ### Output: minified
//!     ### Output: ast
//! and run the test, the minified and ast output will be generated from the provided input.
//!
//!
//! The input can have optional attributes to control how it is parsed.
//! Example:
//!         ```js parse:expr
//!         ...
//!         ```
//! will parse the code as an expression and not a whole program.
//!
//! Other values are:
//! - `parse:stmt`
//! - `parse:program`
//! - `source:script`
//! - `source:module`
//! - `source:unknown`
//!

extern crate fajt_macros;
extern crate fajt_testing;

use fajt_ast::traverse::Traverse;
use fajt_ast::{Expr, Program, SourceType, Stmt};
use fajt_codegen::{generate_code, GeneratorContext};
use fajt_parser::error::{ErrorKind, Result};
use fajt_parser::{parse, Parse};
use fajt_testing::markdown::Markdown;
use fajt_testing::{read_string, write_string};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

const SOURCE_SECTION: &str = "Source";
const MINIFIED_SECTION: &str = "Output: minified";
const AST_SECTION: &str = "Output: ast";

// This runs for each .md file in the ./cases folder.
fn run_test(path: &str) {
    println!("Running: {}", path);

    let data = read_string(path.as_ref());
    let test = Markdown::from_string(&data);

    if let Some(source_block) = test.get_block(SOURCE_SECTION) {
        let parse_type = get_attribute(source_block.language, "parse:").unwrap_or("program");
        let source_type = get_source_type(source_block.language);
        match parse_type {
            "expr" => parse_and_test::<Expr>(path, test, source_type),
            "stmt" => parse_and_test::<Stmt>(path, test, source_type),
            _ => parse_and_test::<Program>(path, test, source_type),
        };
    }
}

fn parse_and_test<T>(path: &str, test: Markdown, source_type: SourceType)
where
    T: Parse + Serialize + DeserializeOwned + PartialEq + Debug + Traverse,
{
    let mut regenerate_ast = false;
    let mut regenerate_min = None;

    let mut test = test;
    let source_block = test.get_block(SOURCE_SECTION).unwrap();
    let source = source_block.contents;
    let mut result = parse::<T>(&source, source_type);

    if let Some(ast_section) = test.get_section(AST_SECTION) {
        if let Some(ast) = ast_section.get_code() {
            assert_result(&result, ast);
        } else {
            regenerate_ast = true;
        }
    }

    let parse_type = get_attribute(source_block.language, "parse:").unwrap_or("program");
    let code_output = generate_code(result.as_mut().unwrap(), GeneratorContext::new());

    if parse_type == "expr" {
        assert_eq!(code_output.trim(), source.trim());
    } else {
        assert_eq!(code_output, source);
    }

    if let Some(minified_section) = test.get_section(MINIFIED_SECTION) {
        let mut ctx = GeneratorContext::new();
        ctx.minified = true;

        let output_min = generate_code(result.as_mut().unwrap(), ctx);

        if let Some(expected_minified) = minified_section.get_code() {
            assert_eq!(
                output_min,
                expected_minified.trim(),
                "Minified output mismatch."
            );
        } else {
            regenerate_min = Some(output_min);
        }
    }

    #[allow(unused_assignments)]
    let mut ast_output = None; // Just to make sure it lives until written
    if regenerate_ast {
        ast_output = Some(result_to_string(result));
        test.set_block_content(AST_SECTION, "json", ast_output.as_ref().unwrap());
    }

    if let Some(min) = regenerate_min.as_ref() {
        test.set_block_content(MINIFIED_SECTION, "js", min);
    }

    if regenerate_ast || regenerate_min.is_some() {
        write_string(path.as_ref(), &test.to_string());
        panic!("Output generated. Verify and rerun test.");
    }
}

fn assert_result<T>(result: &Result<T>, ast_json: &str)
where
    T: Parse + Serialize + DeserializeOwned + PartialEq + Debug,
{
    if let Ok(result) = result {
        let expected_expr: T = serde_json::from_str(&ast_json).unwrap();
        assert_eq!(result, &expected_expr)
    } else {
        let error = result.as_ref().unwrap_err();
        println!("Error: {:?}", error);

        let expected_error: ErrorKind = serde_json::from_str(&ast_json).unwrap();
        assert_eq!(error.kind(), &expected_error)
    }
}

fn get_source_type(language: &str) -> SourceType {
    let source_type = get_attribute(language, "source:").unwrap_or("unknown");
    match source_type {
        "script" => SourceType::Script,
        "module" => SourceType::Module,
        _ => SourceType::Unknown,
    }
}

fn get_attribute<'a>(language: &'a str, attribute: &str) -> Option<&'a str> {
    language
        .split(' ')
        .find(|s| s.starts_with(attribute))
        .map(|attr| attr.split(':').next_back())
        .flatten()
}

fn result_to_string<T>(result: Result<T>) -> String
where
    T: Parse + Serialize + Debug,
{
    if let Ok(result) = result {
        serde_json::to_string_pretty(&result).unwrap()
    } else {
        serde_json::to_string_pretty(&result.unwrap_err().kind()).unwrap()
    }
}

macro_rules! generate_test_cases {
    ("md", $file_path:literal, $ident:ident) => {
        #[test]
        fn $ident() {
            $crate::run_test($file_path)
        }
    };
    ("md_ignore", $file_path:literal, $ident:ident) => {
        #[ignore]
        #[test]
        fn $ident() {
            $crate::run_test($file_path)
        }
    };
    ($extension:literal, $file_path:literal, $ident:ident) => {};
}

macro_rules! generate_test_modules {
    (
        $(
            $mod_name:ident: [
                $(
                    $folder:literal
                ),*
            ],
        )*
    ) => {
        $(
            mod $mod_name {
                use fajt_macros::for_each_file;

                $(
                    for_each_file!($folder, generate_test_cases);
                )*
            }
        )*
    }
}

generate_test_modules!(
    expr: ["tests/cases/expr", "parser/tests/cases/expr"],
    stmt: ["tests/cases/stmt", "parser/tests/cases/stmt"],
    decl: ["tests/cases/decl"],
    semicolon: ["parser/tests/cases/semicolon"],
    strict_mode: ["parser/tests/cases/strict-mode"],
    source_module: ["tests/cases/source-module"],
    source_script: ["parser/tests/cases/source-script"],
);

#[test]
fn dummy() {
    // This is just so IDE recognize this is a runnable file.
}
