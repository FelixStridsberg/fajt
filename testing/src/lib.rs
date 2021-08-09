use quote::quote;
use quote::format_ident;
use std::path::PathBuf;
use syn::{bracketed, parse_macro_input};
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use std::fs::read_dir;
use syn::__private::TokenStream2;

mod snapshot;

#[derive(Debug)]
struct SnapshotTestsInput {
    dirs: Vec<String>,
    test_endings: Vec<String>,
    runner: syn::Ident,
}

impl Parse for SnapshotTestsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut dirs = None;
        let mut test_endings = None;
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
                "runner" => {
                    runner = Some(input.parse::<syn::Ident>()?);
                    input.parse::<syn::Token![,]>()?;
                }
                l => panic!("Unknown label '{}:'", l)
            }
        }

        let dirs = dirs.expect("Missing `dir:` attribute");
        let test_endings = test_endings.expect("Missing `test_endings:` attribute");
        let runner = runner.expect("Missing `runner:` attribute");

        Ok(SnapshotTestsInput {
            dirs,
            test_endings,
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

fn find_files(path: &PathBuf) -> Vec<(String, String, String)> {
    let mut paths = Vec::new();
    let files = read_dir(path).unwrap();
    for file in files {
        let file = file.unwrap();
        let file_type = file.file_type().unwrap();
        if file_type.is_file() {
            let path = file.path().into_os_string().into_string().unwrap();
            let extension = file.path().extension().unwrap().to_os_string().into_string().unwrap();
            paths.push((path, file.file_name().into_string().unwrap(), extension))
        }

        if file_type.is_dir() {
            let mut nested_files = find_files(&file.path());
            paths.append(&mut nested_files);
        }
    }

    paths
}

#[proc_macro]
pub fn snapshot_tests(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SnapshotTestsInput);

    let mut project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Up one level to the root crate.
    project_root.pop();

    project_root.push(&input.dirs[0]);

    let files = find_files(&project_root);
    let runner = input.runner;

    let tokens: Vec<TokenStream2> = files.iter().map(|(path, name, extension)| {
        let identifier = create_identifier(name);
        quote! {
            #runner!(#path, #identifier, #extension);
        }
    }).collect();

    TokenStream::from(quote! {
        #(#tokens)*
    })
}

fn create_identifier(name: &str) -> syn::Ident {
    let mut name = name.to_owned().replace('-', "_").replace('.', "_");
    name.retain(|c| ('a'..'z').contains(&c) || ('A'..'Z').contains(&c) || ('0'..'9').contains(&c) || c == '_');
    format_ident!("{}", name)
}