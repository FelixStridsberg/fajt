extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

mod enum_from_string;

use enum_from_string::enum_from_string;

#[proc_macro_derive(FromString, attributes(from_string, from_string_macro))]
pub fn enum_from_string_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = if let Data::Enum(enum_data) = &input.data {
        enum_from_string(&input, &enum_data)
    } else {
        panic!("FromString is only applicable for enums.");
    };

    TokenStream::from(tokens)
}
