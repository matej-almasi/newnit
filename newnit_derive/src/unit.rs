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

pub fn derive(ast: &syn::DeriveInput) -> TokenStream {
    // TODO: better error message
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

            fn from_base(base: f64) -> Box<Self> {
                Box::new(Self((base - #offset) / #factor))
            }
        }

    };
    generated.into()
}
