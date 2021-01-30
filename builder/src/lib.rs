use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();

    let name = &ast.ident;
    let bident = Ident::new(&format!("{}Builder", name), name.span());

    let tokens = quote! {

        pub struct #bident {
        }

        impl Command {
            pub fn builder() -> #bident {
                #bident {
                }
            }
        }

    };

    TokenStream::from(tokens)
}
