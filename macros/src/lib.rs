extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

/// Not used atm but could add boilerplate functionality
#[proc_macro_derive(MMFTInterface)]
pub fn impl_mmft_interface(s: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&s.to_string()).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl MMFTInterface for #name {
            fn schema() -> String {
                serde_json::to_string_pretty(&schema_for!(#name)).unwrap()
            }

            fn from_json(str: &str) -> Self {
                serde_json::from_str(str).unwrap()
            }

            fn to_json(&self) -> String {
                serde_json::to_string(self).unwrap()
            }
        }
    };
    gen.parse().unwrap()
}
