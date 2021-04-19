use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, Variant};

fn map_variants<'a, I: Iterator<Item = &'a Variant>>(
    ident: &Ident,
    variants: I,
) -> Vec<TokenStream> {
    variants
        .map(|v| {
            let variant_ident = &v.ident;
            let variant_string = v.ident.to_string().to_lowercase();
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
