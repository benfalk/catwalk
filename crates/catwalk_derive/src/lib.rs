#![allow(dead_code, unused_imports)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Catwalk, attributes(catwalk))]
pub fn my_macro(input: TokenStream) -> TokenStream {
    println!("{input:#?}");
    let _input = parse_macro_input!(input as DeriveInput);

    TokenStream::from(quote! {
        impl catwalk::Model for Person {
            fn tablename(&self) -> &str {
                "people"
            }
        }
    })
}
