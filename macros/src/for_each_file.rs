use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use syn::parse::{Parse, ParseStream};

const INPUT_ERROR: &str =
    "Unexpected input, expected: for_each_file!(\"string/path\", macro_identifier);";

struct File {
    path: String,
    relative_path: String,
    extension: String,
}

struct MacroInput {
    path: syn::LitStr,
    macro_ident: syn::Ident,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: syn::LitStr = input.parse().expect(INPUT_ERROR);
        input.parse::<syn::Token![,]>().expect(INPUT_ERROR);
        let macro_ident: syn::Ident = input.parse().expect(INPUT_ERROR);
        Ok(MacroInput { path, macro_ident })
    }
}

pub fn for_each_file(input: TokenStream) -> TokenStream {
    let MacroInput { path, macro_ident } = syn::parse_macro_input!(input as MacroInput);

    // Cargo manifest of this crate, and up one level to the root crate.
    // NOTE: This must probably be ported if using this macro outside this project.
    let mut directory_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory_root.pop();
    directory_root.push(path.value());

    let files = find_files(&directory_root, "");
    let macro_invocations: Vec<TokenStream2> = files
        .into_iter()
        .map(|file| {
            let path = &file.path;
            let extension = &file.extension;
            let identifier = create_identifier(&file.relative_path, extension);
            quote! {
                #macro_ident!(#extension, #path, #identifier);
            }
        })
        .collect();

    TokenStream::from(quote! {
        #(#macro_invocations)*
    })
}

fn find_files(path: &Path, directory_path: &str) -> Vec<File> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            files.push(to_file(&entry, directory_path))
        }

        if file_type.is_dir() {
            let mut relative_path = entry.file_name().into_string().unwrap();
            relative_path.push('/');
            let dir_prefix = if directory_path.is_empty() {
                relative_path
            } else {
                format!("{directory_path}{relative_path}")
            };

            let mut nested_files = find_files(&entry.path(), &dir_prefix);
            files.append(&mut nested_files);
        }
    }

    files
}

fn to_file(entry: &DirEntry, relative_path: &str) -> File {
    let name = entry.file_name().into_string().unwrap();
    let path = entry.path().into_os_string().into_string().unwrap();
    let extension = entry
        .path()
        .extension()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    File {
        relative_path: format!("{relative_path}{name}"),
        path,
        extension,
    }
}

fn create_identifier(name: &str, extension: &str) -> syn::Ident {
    let mut name = name[0..name.len() - (extension.len() + 1)]
        .to_owned()
        .replace(['-', '.'], "_")
        .replace('/', "__");
    name.retain(|c| c.is_ascii_alphanumeric());
    format_ident!("{}", name)
}
