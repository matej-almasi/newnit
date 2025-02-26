use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(quantity))]
pub(crate) struct QuantityArgs {
    #[darling(default)]
    from: bool,
    #[darling(default)]
    partial_eq: bool,
    #[darling(default)]
    ops: bool,
}

pub(crate) fn impl_quantity(unit: &Ident, quantity: &Ident, args: &QuantityArgs) -> TokenStream {
    let impl_from = args.from.then(|| {
        quote! {
            impl<T: #quantity + Unit> From<&T> for #unit {
                fn from(other: &T) -> Self {
                    Self::from_base(other.to_base())
                }
            }
        }
    });

    let impl_partial_eq = args.partial_eq.then(|| {
        quote! {
            impl<T: #quantity + Unit> std::cmp::PartialEq<T> for #unit {
                fn eq(&self, other: &T) -> bool {
                    self.to_base() == other.to_base()
                }
            }
        }
    });

    let impl_ops = args.ops.then(|| {
        quote! {
            impl<T: #quantity + Unit> std::ops::Add<&T> for #unit {
                type Output = Self;

                fn add(self, other: &T) -> Self::Output {
                    Self::from_base(self.to_base() + other.to_base())
                }
            }

            impl<T: #quantity + Unit> std::ops::AddAssign<&T> for #unit {
                fn add_assign(&mut self, other: &T) {
                    self.0 = Self::from_base(self.to_base() + other.to_base()).value();
                }
            }

            impl<T: #quantity + Unit> std::ops::Div<&T> for #unit {
                type Output = f64;

                fn div(self, other: &T) -> Self::Output {
                    self.to_base() / other.to_base()
                }
            }

            impl std::ops::Div<f64> for #unit {
                type Output = Self;

                fn div(self, rhs: f64) -> Self::Output {
                    Self(self.value() / rhs)
                }
            }

            impl std::ops::DivAssign<f64> for #unit {
                fn div_assign(&mut self, rhs: f64) {
                    self.0 /= rhs;
                }
            }

            impl std::ops::Mul<f64> for #unit {
                type Output = Self;

                fn mul(self, rhs: f64) -> Self::Output {
                    Self(self.value() * rhs)
                }
            }

            impl std::ops::Mul<#unit> for f64 {
                type Output = #unit;

                fn mul(self, rhs: #unit) -> Self::Output {
                    #unit(self * rhs.value())
                }
            }

            impl std::ops::MulAssign<f64> for #unit {
                fn mul_assign(&mut self, rhs: f64) {
                    self.0 *= rhs;
                }
            }

            impl std::ops::Neg for #unit {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self(-self.value())
                }
            }

            impl<T: #quantity + Unit> std::ops::Sub<&T> for #unit {
                type Output = Self;

                fn sub(self, other: &T) -> Self::Output {
                    Self::from_base(self.to_base() - other.to_base())
                }
            }

            impl<T: #quantity + Unit> std::ops::SubAssign<&T> for #unit {
                fn sub_assign(&mut self, other: &T) {
                    self.0 = Self::from_base(self.to_base() - other.to_base()).value();
                }
            }

        }
    });

    let generated = quote! {
        impl #quantity for #unit {}

        #impl_from

        #impl_partial_eq

        #impl_ops

    };

    generated.into()
}
