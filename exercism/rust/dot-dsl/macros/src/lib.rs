extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Attrs)]
pub fn with_attrs_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl Attrs for #name {
            fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                for (k, v) in attrs {
                    self.attrs.insert(k.to_string(), v.to_string());
                }
                self
            }

            fn get_attr(&self, key: &str) -> Option<&str> {
                if let Some(v) = self.attrs.get(key) {
                    return Some(v.as_str());
                }
                None
            }
        }
    };
    gen.into()
}
