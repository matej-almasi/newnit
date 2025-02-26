use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use crate::impl_quantity::{OtherImpls, impl_quantity};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(current))]
struct CurrentArgs {
    #[darling(default)]
    from: bool,
    #[darling(default)]
    partial_eq: bool,
    #[darling(default)]
    ops: bool,
}

pub fn derive(ast: &syn::DeriveInput) -> TokenStream {
    // TODO: better error message
    let args = CurrentArgs::from_derive_input(ast).expect("Failed parsing derive arguments.");

    let unit = &ast.ident;

    let quantity = Ident::new("Current", Span::call_site());

    let other_impls = OtherImpls {
        from: args.from,
        partial_eq: args.partial_eq,
        ops: args.ops,
    };

    impl_quantity(unit, &quantity, &other_impls)
}
