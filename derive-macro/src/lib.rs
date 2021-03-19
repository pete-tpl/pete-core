extern crate proc_macro;

use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HasBaseNode)]
/// Adds methods "get_base_node" and "get_base_node_mut" to nodes
pub fn derive_has_base_node(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        use crate::nodes::HasBaseNodeTrait;
        impl HasBaseNodeTrait for #ident {
            fn get_base_node(&self) -> &BaseNode {
                return &self.base_node;
            }

            fn get_base_node_mut(&mut self) -> &mut BaseNode {
                return &mut self.base_node;
            }
        }
    };

    output.into()
}