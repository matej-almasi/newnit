use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(length))]
struct LengthArgs {
    #[darling(default)]
    from: bool,
    #[darling(default)]
    partial_eq: bool,
    #[darling(default)]
    ops: bool,
}

pub fn derive(ast: &syn::DeriveInput) -> TokenStream {
    // TODO: better error message
    let args = LengthArgs::from_derive_input(ast).expect("Failed parsing derive arguments.");

    let name = &ast.ident;

    let impl_from = args.from.then(|| {
        quote! {
            impl<T: Length> From<&T> for #name {
                fn from(other: &T) -> Self {
                    *Self::from_base(other.to_base())
                }
            }
        }
    });

    let impl_partial_eq = args.partial_eq.then(|| {
        quote! {
            impl<T: Length> std::cmp::PartialEq<T> for #name {
                fn eq(&self, other: &T) -> bool {
                    self.to_base() == other.to_base()
                }
            }
        }
    });

    let impl_ops = args.ops.then(|| {
        quote! {
            impl<T: Length> std::ops::Add<&T> for #name {
                type Output = Self;

                fn add(self, other: &T) -> Self::Output {
                    *Self::from_base(self.to_base() + other.to_base())
                }
            }

            impl<T: Length> std::ops::AddAssign<&T> for #name {
                fn add_assign(&mut self, other: &T) {
                    self.0 = Self::from_base(self.to_base() + other.to_base()).value();
                }
            }

            impl<T: Length> std::ops::Div<&T> for #name {
                type Output = f64;

                fn div(self, other: &T) -> Self::Output {
                    self.to_base() / other.to_base()
                }
            }

            impl std::ops::Div<f64> for #name {
                type Output = Self;

                fn div(self, rhs: f64) -> Self::Output {
                    Self(self.value() / rhs)
                }
            }

            impl std::ops::DivAssign<f64> for #name {
                fn div_assign(&mut self, rhs: f64) {
                    self.0 /= rhs;
                }
            }

            impl std::ops::Mul<f64> for #name {
                type Output = Self;

                fn mul(self, rhs: f64) -> Self::Output {
                    Self(self.value() * rhs)
                }
            }

            impl std::ops::Mul<#name> for f64 {
                type Output = #name;

                fn mul(self, rhs: #name) -> Self::Output {
                    #name(self * rhs.value())
                }
            }

            impl std::ops::MulAssign<f64> for #name {
                fn mul_assign(&mut self, rhs: f64) {
                    self.0 *= rhs;
                }
            }

            impl std::ops::Neg for #name {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self(-self.value())
                }
            }

            impl<T: Length> std::ops::Sub<&T> for #name {
                type Output = Self;

                fn sub(self, other: &T) -> Self::Output {
                    *Self::from_base(self.to_base() - other.to_base())
                }
            }

            impl<T> std::ops::SubAssign<&T> for #name {
                fn sub_assign(&mut self, other: &T) {
                    self.0 = Self::from_base(self.to_base() - other.to_base()).value();
                }
            }

        }
    });

    let generated = quote! {
        impl Length for #name {}

        #impl_from

        #impl_partial_eq

        #impl_ops

    };
    generated.into()
}
