use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use crate::impl_quantity::{QuantityArgs, impl_quantity};

pub fn derive(ast: &syn::DeriveInput) -> TokenStream {
    let args = QuantityArgs::from_derive_input(ast).expect("Failed parsing derive arguments.");

    let unit = &ast.ident;

    let quantity = Ident::new("Velocity", Span::call_site());

    impl_quantity(unit, &quantity, &args)
}
