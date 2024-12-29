#![allow(clippy::expect_used, clippy::panicking_unwrap)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(TinyError, attributes(error, from))]
pub fn my_error_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let (display_impl, from_impls) = match &input.data {
        Data::Enum(data_enum) => {
            let mut display_variants = vec![];
            let mut from_variants = vec![];

            for variant in &data_enum.variants {
                let variant_name = &variant.ident;
                let message = variant
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("error"))
                    .and_then(|attr| attr.parse_args::<syn::LitStr>().ok())
                    .expect("Each variant must have an #[error(\"...\")] attribute");

                // Add Display implementation for this variant
                display_variants.push(quote! {
                    #name::#variant_name => write!(f, #message),
                });

                // Check for `from` attribute to generate a From implementation
                if let Some(from_attr) = variant
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("from"))
                {
                    let ty = from_attr
                        .parse_args::<syn::Type>()
                        .expect("Invalid type in #[from]");

                    from_variants.push(quote! {
                        impl From<#ty> for #name {
                            fn from(value: #ty) -> Self {
                                #name::#variant_name
                            }
                        }
                    });
                }
            }

            (display_variants, from_variants)
        }
        _ => panic!("TinyError derive macro only supports enums"),
    };

    let output = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_impl)*
                }
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }

        impl std::error::Error for #name {}

        #(#from_impls)*
    };

    TokenStream::from(output)
}
