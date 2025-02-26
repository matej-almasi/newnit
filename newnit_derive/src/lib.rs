use proc_macro::TokenStream;

mod impl_quantity;

mod area;
mod length;
mod mass;
mod volume;

mod unit;

#[proc_macro_derive(Unit, attributes(unit))]
pub fn unit_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    unit::derive(&ast)
}

#[proc_macro_derive(Length, attributes(length))]
pub fn length_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    length::derive(&ast)
}

#[proc_macro_derive(Area, attributes(area))]
pub fn area_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    area::derive(&ast)
}

#[proc_macro_derive(Volume, attributes(volume))]
pub fn volume_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    volume::derive(&ast)
}

#[proc_macro_derive(Mass, attributes(mass))]
pub fn mass_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    mass::derive(&ast)
}
