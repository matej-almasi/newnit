use proc_macro::TokenStream;

mod unit;

#[proc_macro_derive(Unit, attributes(unit))]
pub fn unit_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    unit::derive(&ast)
}
