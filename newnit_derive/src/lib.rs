use proc_macro::TokenStream;

mod impl_quantity;

mod area;
mod current;
mod length;
mod luminous_intensity;
mod mass;
mod substance_amount;
mod temperature;
mod time;
mod velocity;
mod volume;

mod unit;

#[proc_macro_derive(Unit, attributes(unit))]
pub fn unit_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    unit::derive(&ast)
}

#[proc_macro_derive(Length, attributes(quantity))]
pub fn length_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    length::derive(&ast)
}

#[proc_macro_derive(Area, attributes(quantity))]
pub fn area_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    area::derive(&ast)
}

#[proc_macro_derive(Volume, attributes(quantity))]
pub fn volume_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    volume::derive(&ast)
}

#[proc_macro_derive(Mass, attributes(quantity))]
pub fn mass_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    mass::derive(&ast)
}

#[proc_macro_derive(Velocity, attributes(quantity))]
pub fn velocity_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    velocity::derive(&ast)
}

#[proc_macro_derive(Time, attributes(quantity))]
pub fn time_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    time::derive(&ast)
}

#[proc_macro_derive(Current, attributes(quantity))]
pub fn current_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    current::derive(&ast)
}

#[proc_macro_derive(LuminousIntensity, attributes(quantity))]
pub fn luminous_intensity_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    luminous_intensity::derive(&ast)
}

#[proc_macro_derive(SubstanceAmount, attributes(quantity))]
pub fn substance_amount_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    substance_amount::derive(&ast)
}

#[proc_macro_derive(Temperature, attributes(quantity))]
pub fn temperature_derive(input: TokenStream) -> TokenStream {
    // TODO: better error message
    let ast = syn::parse(input).expect("Failed to parse input code.");
    temperature::derive(&ast)
}
