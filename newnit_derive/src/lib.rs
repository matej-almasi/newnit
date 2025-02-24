use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(unit))]
struct UnitArgs {
    factor: f64,

    #[darling(default)] // Default to 0.0 if missing
    offset: f64,
}

#[proc_macro_derive(Unit, attributes(unit))]
pub fn unit_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    impl_unit(&ast)
}

fn impl_unit(ast: &syn::DeriveInput) -> TokenStream {
    let args = UnitArgs::from_derive_input(ast).expect("Missing factor for unit conversion.");

    let name = &ast.ident;
    let factor = args.factor;
    let offset = args.offset;

    let generated = quote! {
        impl Unit for #name {
            fn to_base(&self) -> f64 {
                self.value() * #factor + #offset
            }

            fn value(&self) -> f64 {
                self.0
            }

        }

    };
    generated.into()
}

// fn from_base(base: f64) -> Self {
//     Self((base - #offset) / #factor)
// }
