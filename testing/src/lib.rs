use quote::quote;
use quote::format_ident;
use std::path::PathBuf;
use syn::{bracketed, parse_macro_input};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenSteam2};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::snapshot::TestSuite;

mod snapshot;

#[derive(Debug)]
struct SnapshotTestsInput {
    dirs: Vec<String>,
    test_endings: Vec<String>,
    result_endings: Vec<String>,
    runner: syn::Ident,
}

impl Parse for SnapshotTestsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut dirs = None;
        let mut test_endings = None;
        let mut result_endings = None;
        let mut runner = None;

        loop {
            if input.is_empty() {
                break;
            }

            let label = parse_label(&input)?;
            match label.as_str() {
                "dirs" => {
                    dirs = Some(parse_vector(&input)?);
                }
                "test_endings" => {
                    test_endings = Some(parse_vector(&input)?);
                }
                "result_endings" => {
                    result_endings = Some(parse_vector(&input)?);
                }
                "runner" => {
                    runner = Some(input.parse::<syn::Ident>()?);
                    input.parse::<syn::Token![,]>()?;
                }
                l => panic!("Unknown label '{}:'", l)
            }
        }

        let dirs = dirs.expect("Missing `dir:` attribute");
        let test_endings = test_endings.expect("Missing `test_endings:` attribute");
        let result_endings = result_endings.expect("Missing `result_endings:` attribute");
        let runner = runner.expect("Missing `runner:` attribute");

        Ok(SnapshotTestsInput {
            dirs,
            test_endings,
            result_endings,
            runner
        })
    }
}

fn parse_label(input: &ParseStream) -> syn::Result<String> {
    let name: syn::Ident = input.parse()?;
    input.parse::<syn::Token![:]>()?;
    Ok(name.to_string())
}

fn parse_vector(input: &ParseStream) -> syn::Result<Vec<String>> {
    let content;
    bracketed!(content in input);

    let lit_list: Punctuated<syn::LitStr, syn::Token![,]> = content.parse_terminated(|input| {
        input.parse::<syn::LitStr>()
    })?;

    input.parse::<syn::Token![,]>()?;

    Ok(lit_list.into_iter().map(|a| a.value()).collect())
}

#[proc_macro]
pub fn snapshot_tests(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SnapshotTestsInput);

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut loader = TestSuite::loader(&input.test_endings, &input.result_endings);
    for dir in &input.dirs {
        let mut path = project_root.clone();
        path.pop();
        path.push(dir);
        loader.load_dir(path);
    }

    let suite = loader.build();

    let case: Vec<TokenSteam2> = suite.test_cases.iter().map(|case| {
        let test_file = &case.test_file;
        let result_files: Vec<TokenSteam2> = case.result_files.iter().map(|(key, value)| {
            quote! {
            result_files.insert(#key, #value);
        }
        }).collect();

        let runner = &input.runner;
        let test_name = create_identifier(&case.name);

        quote! {
            #[test]
            fn #test_name() {
                let test_file = #test_file;
                let mut result_files = HashMap::new();
                #(#result_files)*

                #runner(test_file, result_files);
            }
        }
    }).collect();

    TokenStream::from(quote! {
        #(#case)*
    })
}

fn create_identifier(name: &str) -> syn::Ident {
    let mut name = name.to_owned().replace('-', "_");
    name.retain(|c| ('a'..'z').contains(&c) || ('A'..'Z').contains(&c) || ('0'..'9').contains(&c) || c == '_');
    format_ident!("{}", name)
}