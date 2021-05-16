use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::{Attribute, DataEnum, DeriveInput, Variant};

pub fn enum_from_string(input: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
    let from_str_impl_tokens = generate_from_str_impl(input, enum_data);
    let to_str_impl_tokens = generate_to_str_impl(input, enum_data);
    let macro_tokens = generate_macro(input, enum_data);

    quote! {
        #macro_tokens
        #from_str_impl_tokens
        #to_str_impl_tokens
    }
}

fn generate_from_str_impl(input: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
    let ident = &input.ident;
    let match_branches = map_variants(enum_data, |v| {
        let variant_ident = &v.ident;
        let variant_string = variant_string(v);
        quote! {
            #variant_string => Ok(#ident::#variant_ident)
        }
    });

    quote! {
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

fn generate_to_str_impl(input: &DeriveInput, enum_data: &DataEnum) -> TokenStream {
    let ident = &input.ident;
    let match_branches = map_variants(enum_data, |v| {
        let variant_ident = &v.ident;
        let variant_string = variant_string(v);
        quote! {
            #ident::#variant_ident => #variant_string.to_owned()
        }
    });

    quote! {
        impl std::string::ToString for #ident {
            fn to_string(&self) -> String {
                match self {
                    #(#match_branches,)*
                }
            }
        }
    }
}

fn generate_macro(input: &DeriveInput, enum_data: &DataEnum) -> Option<TokenStream> {
    let macro_name = get_macro_name(&input.attrs);
    let extra_rules = get_macro_rules(&input.attrs).unwrap_or_else(|| {
        quote! {
            ($a:ident) => { $a };
        }
    });

    macro_name.map(|name| {
        let macro_name = syn::Ident::new(&name, Span::call_site());
        let macro_rules = map_variants(enum_data, |v| {
            let variant_ident = &v.ident;
            let variant_string = variant_string(v);
            quote! {
                (#variant_string) => { #macro_name!(#variant_ident) }
            }
        });

        quote! {
            #[macro_export]
            macro_rules! #macro_name {
                #extra_rules
                #(#macro_rules;)*
            }
        }
    })
}

fn get_macro_name(attrs: &[Attribute]) -> Option<String> {
    let macro_attribute = attrs.iter().find(|a| a.path.is_ident("from_string_macro"));
    macro_attribute.map(|a| {
        let string_literal: syn::LitStr = a
            .parse_args()
            .expect("Could not parse arg of from_string_macro attribute.");
        string_literal.value()
    })
}

fn get_macro_rules(attrs: &[Attribute]) -> Option<TokenStream> {
    let macro_attribute = attrs
        .iter()
        .find(|a| a.path.is_ident("from_string_macro_rules"));
    macro_attribute.map(|a| {
        // #[from_string_macro_rules( ($item:ident) => { $ident } )]
        //      We get all of this  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
        //      but we only want the contents of the parenthesis.
        get_group_content(&a.tokens)
    })
}

fn get_group_content(tokens: &TokenStream) -> TokenStream {
    tokens
        .clone()
        .into_iter()
        .map(|t| match t {
            TokenTree::Group(g) => g.stream(),
            _ => panic!("Expected a TokenTree::Group"),
        })
        .collect()
}

fn variant_string(variant: &Variant) -> String {
    let attribute = variant
        .attrs
        .iter()
        .find(|a| a.path.is_ident("from_string"));
    attribute
        .map(|a| {
            let string_literal: syn::LitStr = a
                .parse_args()
                .expect("Could not parse #[from_string(..)] attribute.");
            string_literal.value()
        })
        .unwrap_or_else(|| variant.ident.to_string().to_lowercase())
}

fn map_variants<F: Fn(&Variant) -> proc_macro2::TokenStream>(
    enum_data: &DataEnum,
    map: F,
) -> Vec<proc_macro2::TokenStream> {
    enum_data.variants.iter().map(map).collect()
}
