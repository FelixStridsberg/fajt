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

extern crate core;
extern crate fajt_macros;
extern crate fajt_testing;

use fajt_ast::traverse::Traverse;
use fajt_ast::{Expr, Program, SourceType, Stmt};
use fajt_codegen::{generate_code, GeneratorContext};
use fajt_parser::error::emitter::ErrorEmitter;
use fajt_parser::error::Result;
use fajt_parser::{parse, Parse};
use fajt_testing::markdown::{Markdown, MarkdownBlock};
use fajt_testing::{read_string, write_string};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::io::Cursor;

const SOURCE_SECTION: &str = "Source";
const MINIFIED_SECTION: &str = "Output: minified";
const AST_SECTION: &str = "Output: ast";
const ERROR_SECTION: &str = "Output: error";

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
    let mut regenerate_error = None;

    let mut test = test;
    let source_block = test.get_block(SOURCE_SECTION).unwrap();
    let source = source_block.contents;
    let mut result = parse::<T>(source, source_type);

    if let Some(ast_section) = test.get_section(AST_SECTION) {
        if let Some(ast) = ast_section.get_code() {
            assert_ast(&result, ast);
        } else {
            regenerate_ast = true;
        }
    }

    if let Some(error_section) = test.get_section(ERROR_SECTION) {
        let error_msg = error_to_message(&result, source);

        if let Some(expected_error_msg) = error_section.get_code() {
            assert_eq!(error_msg, expected_error_msg, "Error message don't match.");
        } else {
            regenerate_error = Some(error_msg);
        }
    }

    if result.is_ok() {
        let source_format = get_attribute(source_block.language, "check-format:");
        if source_format != Some("no") {
            assert_source_format(source_block, source, &mut result);
        }

        if let Some(minified_section) = test.get_section(MINIFIED_SECTION) {
            let mut ctx = GeneratorContext::new();
            ctx.minified = true;

            let output_min = generate_code(result.as_mut().unwrap(), ctx);
            if let Some(expected_minified) = minified_section.get_code() {
                assert_minified_output(&output_min, expected_minified);
            } else {
                regenerate_min = Some(output_min);
            }
        }
    }

    if let Some(error_message) = regenerate_error.as_ref() {
        test.set_block_content(ERROR_SECTION, "txt", error_message);
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

    if regenerate_ast || regenerate_min.is_some() || regenerate_error.is_some() {
        write_string(path.as_ref(), &test.to_string());
        panic!("Output generated. Verify and rerun test.");
    }
}

fn assert_ast<T>(result: &Result<T>, ast_json: &str)
where
    T: Parse + Serialize + DeserializeOwned + PartialEq + Debug,
{
    if let Ok(result) = result {
        let expected_expr: T = serde_json::from_str(ast_json).unwrap();
        assert_eq!(result, &expected_expr)
    } else {
        panic!("Tried to compare AST but got error result.");
    }
}

fn assert_source_format<T>(source_block: &MarkdownBlock, source: &str, result: &mut Result<T>)
where
    T: Parse + Serialize + DeserializeOwned + PartialEq + Debug + Traverse,
{
    let parse_type = get_attribute(source_block.language, "parse:").unwrap_or("program");
    let code_output = generate_code(result.as_mut().unwrap(), GeneratorContext::new());

    if parse_type == "expr" {
        assert_eq!(
            code_output.trim(),
            source.trim(),
            "Source do not match formatted output."
        );
    } else {
        assert_eq!(code_output, source, "Source do not match formatted output.");
    }
}

fn assert_minified_output(output_min: &str, expected_minified: &str) {
    assert_eq!(
        output_min.trim(),
        expected_minified.trim(),
        "Minified output mismatch."
    );
}

fn error_to_message<T>(result: &Result<T>, source: &str) -> String
where
    T: Parse + Serialize + DeserializeOwned + PartialEq + Debug + Traverse,
{
    if result.is_ok() {
        panic!("Expected error but got {:?}", result);
    }

    let error = result.as_ref().unwrap_err();

    let mut cursor = Cursor::new(Vec::new());
    let mut emitter = ErrorEmitter::new("test.js", source, &mut cursor);
    emitter.emit_error(error).unwrap();

    String::from_utf8(cursor.into_inner()).unwrap()
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
    serde_json::to_string_pretty(result.as_ref().unwrap()).unwrap()
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
    expr: ["tests/cases/expr"],
    stmt: ["tests/cases/stmt"],
    decl: ["tests/cases/decl"],
    semicolon: ["tests/cases/semicolon"],
    strict_mode: ["tests/cases/strict-mode"],
    source_module: ["tests/cases/source-module"],
    source_script: ["tests/cases/source-script"],
    comment: ["tests/cases/comment"],
);

#[test]
fn dummy() {
    // This is just so IDE recognize this is a runnable file.
}
