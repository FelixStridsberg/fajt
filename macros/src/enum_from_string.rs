use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, DataEnum, DeriveInput, Variant};

// TODO dry up a bit

fn attribute_string(variant: &Variant) -> Option<String> {
    let attribute = variant
        .attrs
        .iter()
        .find(|a| a.path.is_ident("from_string"));
    attribute.map(|a| {
        let string_literal: syn::LitStr = a
            .parse_args()
            .expect("Could not parse #[from_string(..)] attribute.");
        string_literal.value()
    })
}

fn get_macro_name(attrs: &Vec<Attribute>) -> Option<String> {
    let macro_attribute = attrs.iter().find(|a| a.path.is_ident("from_string_macro"));
    macro_attribute.map(|a| {
        let string_literal: syn::LitStr = a
            .parse_args()
            .expect("Could not parse arg of from_string_macro attribute.");
        string_literal.value()
    })
}

fn gen_match_branches<'a, I: Iterator<Item = &'a Variant>>(
    ident: &Ident,
    variants: I,
) -> Vec<TokenStream> {
    variants
        .map(|v| {
            let variant_ident = &v.ident;
            let variant_string =
                attribute_string(v).unwrap_or_else(|| v.ident.to_string().to_lowercase());
            quote! {
                #variant_string => Ok(#ident::#variant_ident)
            }
        })
        .collect()
}

fn gen_macro_branches<'a, I: Iterator<Item = &'a Variant>>(
    ident: &Ident,
    variants: I,
) -> Vec<TokenStream> {
    variants
        .map(|v| {
            let variant_ident = &v.ident;
            let variant_string =
                attribute_string(v).unwrap_or_else(|| v.ident.to_string().to_lowercase());
            quote! {
                (#variant_string) => {#ident::#variant_ident}
            }
        })
        .collect()
}

pub fn enum_from_string(input: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
    let match_branches = gen_match_branches(&input.ident, enum_data.variants.iter());

    let ident = &input.ident;
    let macro_name = get_macro_name(&input.attrs);

    let gen_macro = macro_name.map(|name| {
        let macro_name = syn::Ident::new(&name, Span::call_site());
        let macro_branches = gen_macro_branches(ident, enum_data.variants.iter());
        quote! {
            macro_rules! #macro_name {
                #(#macro_branches;)*
            }
        }
    });

    quote! {
        #gen_macro

        impl std::str::FromStr for #ident {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#match_branches,)*
                    _ => Err("No matching enum found."),
                }
            }
        }
    }
}
