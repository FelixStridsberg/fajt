use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, DataEnum, Variant};

fn get_from_string_attribute(variant: &Variant) -> Option<&Attribute> {
    variant
        .attrs
        .iter()
        .find(|a| a.path.is_ident("from_string"))
}

fn attribute_string(variant: &Variant) -> Option<String> {
    let attribute = get_from_string_attribute(variant);
    attribute.map(|a| {
        let string_literal: syn::LitStr = a
            .parse_args()
            .expect("Could not parse #[from_string(..)] attribute.");
        string_literal.value()
    })
}

fn map_variants<'a, I: Iterator<Item = &'a Variant>>(
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

pub fn enum_from_string(ident: Ident, input: DataEnum) -> TokenStream {
    let variants = map_variants(&ident, input.variants.iter());

    quote! {
        impl std::str::FromStr for #ident {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#variants,)*
                    _ => Err("No matching enum found."),
                }
            }
        }
    }
}
