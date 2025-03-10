use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(unit))]
struct UnitArgs {
    factor: f64,

    #[darling(default)] // Default to 0.0 if missing
    offset: f64,

    #[darling(default)]
    display: bool,
}

pub fn derive(ast: &syn::DeriveInput) -> TokenStream {
    let args = UnitArgs::from_derive_input(ast).expect("Missing factor for unit conversion.");

    let name = &ast.ident;
    let factor = args.factor;
    let offset = args.offset;

    let impl_display = args.display.then(|| {
        quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} {}", self.to_value(), stringify!(#name))
                }
            }

        }
    });

    let generated = quote! {
        impl Unit for #name {
            fn to_base(&self) -> f64 {
                self.to_value() * #factor + #offset
            }

            fn to_value(&self) -> f64 {
                self.0
            }

            fn set_value(&mut self, value: f64) {
                self.0 = value;
            }

            fn set_from_base(&mut self, base: f64) {
                self.0 = (base - #offset) / #factor;
            }

            fn from_base(base: f64) -> Self {
                Self((base - #offset) / #factor)
            }
        }

        #impl_display

    };
    generated.into()
}
