//! A derive macro package for deriving the `Unit` trait and quantity traits
//! defined in the [`newnit`] library.

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
mod unit;
mod velocity;
mod volume;

/// Derive Macro for the `Unit` trait.
///
/// Use this macro with any tuple `struct` that has a [`f64`] as its first
/// field.
///
/// This macro declares additional parameters in its `#[unit()]` macro
/// attribute:
/// - factor: [`f64`] the factor for conversion to base unit
/// - offset: [`f64`] (optional) the offset for conversion to base unit
/// - display: [`bool`] whether to additionally derive the
///   [`Display`](std::fmt::Display) trait.
///
/// The equation for conversion to base unit is defined as follows:
///   value_in_base_unit = value_in_this_unit * `factor` + `offset`
///
/// # Examples:
/// ```
/// use newnit::Unit;
/// use newnit_derive::Unit;
///
/// #[derive(Unit)]
/// #[unit(factor = 0.0254)] // 1 inch is 0.0254 meters
/// struct Inch(f64);
///
/// let length = Inch(42.0);
/// assert_eq!(length.to_base(), 42.0 * 0.0254);
/// ```
#[proc_macro_derive(Unit, attributes(unit))]
pub fn unit_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    unit::derive(&ast)
}

/// Derive macro for the `Length` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `Length` and [`f64`],
///   where such operations make sense:
///   - [`std::ops::Add`] with another `Length`
///   - [`std::ops::AddAssign`] with another `Length`
///   - [`std::ops::Div`] with another `Length`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `Length`
///   - [`std::ops::SubAssign`] with another `Length`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{Unit, length::Length};
/// use newnit_derive::{Length, Unit};
///
/// #[derive(Unit, Length)]
/// #[unit(factor = 0.0254)] // 1 inch is 0.0254 meters
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct Inch(f64);
///
/// let length = Inch(42.0);
/// assert_eq!(length.to_base(), 42.0 * 0.0254);
/// ```
#[proc_macro_derive(Length, attributes(quantity))]
pub fn length_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    length::derive(&ast)
}

/// Derive macro for the `Area` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `Area` and [`f64`],
///   where such operations make sense:
///   - [`std::ops::Add`] with another `Area`
///   - [`std::ops::AddAssign`] with another `Area`
///   - [`std::ops::Div`] with another `Area`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `Area`
///   - [`std::ops::SubAssign`] with another `Area`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{area::Area, Unit};
/// use newnit_derive::{Area, Unit};
///
/// #[derive(Unit, Area)]
/// #[unit(factor = 0.000_645_16)] // 1 square inch is 0.000_645_16 square meters
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct SquareInch(f64);
///
/// let area = SquareInch(42.0);
/// assert_eq!(area.to_base(), 42.0 * 0.000_645_16);
/// ```
#[proc_macro_derive(Area, attributes(quantity))]
pub fn area_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    area::derive(&ast)
}

/// Derive macro for the `Volume` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `Volume` and [`f64`],
///   where such operations make sense:
///   - [`std::ops::Add`] with another `Volume`
///   - [`std::ops::AddAssign`] with another `Volume`
///   - [`std::ops::Div`] with another `Volume`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `Volume`
///   - [`std::ops::SubAssign`] with another `Volume`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{volume::Volume, Unit};
/// use newnit_derive::{Volume, Unit};
///
/// #[derive(Unit, Volume)]
/// #[unit(factor = 0.16387064E-4)] // 1 cubic inch is 0.16387064E-4 cubic meters
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct CubicInch(f64);
///
/// let volume = CubicInch(42.0);
/// assert_eq!(volume.to_base(), 42.0 * 0.16387064E-4);
/// ```
#[proc_macro_derive(Volume, attributes(quantity))]
pub fn volume_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    volume::derive(&ast)
}

/// Derive macro for the `Mass` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `Mass` and [`f64`],
///   where such operations make sense:
///   - [`std::ops::Add`] with another `Mass`
///   - [`std::ops::AddAssign`] with another `Mass`
///   - [`std::ops::Div`] with another `Mass`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `Mass`
///   - [`std::ops::SubAssign`] with another `Mass`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{mass::Mass, Unit};
/// use newnit_derive::{Mass, Unit};
///
/// #[derive(Unit, Mass)]
/// #[unit(factor = 0.453_592_37)] // 1 pound is 0.453_592_37 kilograms
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct Pound(f64);
///
/// let mass = Pound(42.0);
/// assert_eq!(mass.to_base(), 42.0 * 0.453_592_37);
/// ```
#[proc_macro_derive(Mass, attributes(quantity))]
pub fn mass_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    mass::derive(&ast)
}

/// Derive macro for the `Velocity` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `Velocity` and [`f64`],
///   where such operations make sense:
///   - [`std::ops::Add`] with another `Velocity`
///   - [`std::ops::AddAssign`] with another `Velocity`
///   - [`std::ops::Div`] with another `Velocity`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `Velocity`
///   - [`std::ops::SubAssign`] with another `Velocity`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{velocity::Velocity, Unit};
/// use newnit_derive::{Velocity, Unit};
///
/// #[derive(Unit, Velocity)]
/// #[unit(factor = 0.44704)] // 1 MPH is 0.44704 MPS
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct MilePerHour(f64);
///
/// let velocity = MilePerHour(42.0);
/// assert_eq!(velocity.to_base(), 42.0 * 0.44704);
/// ```
#[proc_macro_derive(Velocity, attributes(quantity))]
pub fn velocity_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    velocity::derive(&ast)
}

/// Derive macro for the `Time` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `Time` and [`f64`],
///   where such operations make sense:
///   - [`std::ops::Add`] with another `Time`
///   - [`std::ops::AddAssign`] with another `Time`
///   - [`std::ops::Div`] with another `Time`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `Time`
///   - [`std::ops::SubAssign`] with another `Time`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{time::Time, Unit};
/// use newnit_derive::{Time, Unit};
///
/// #[derive(Unit, Time)]
/// #[unit(factor = 3600.0)] // 1 hour is 3600.0 seconds
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct Hour(f64);
///
/// let time = Hour(42.0);
/// assert_eq!(time.to_base(), 42.0 * 3600.0);
/// ```
#[proc_macro_derive(Time, attributes(quantity))]
pub fn time_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    time::derive(&ast)
}

/// Derive macro for the `Current` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `Current` and [`f64`],
///   where such operations make sense:
///   - [`std::ops::Add`] with another `Current`
///   - [`std::ops::AddAssign`] with another `Current`
///   - [`std::ops::Div`] with another `Current`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `Current`
///   - [`std::ops::SubAssign`] with another `Current`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{current::Current, Unit};
/// use newnit_derive::{Current, Unit};
///
/// #[derive(Unit, Current)]
/// #[unit(factor = 1E+3)] // 1 kA is 1000 Amperes
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct KiloAmpere(f64);
///
/// let current = KiloAmpere(42.0);
/// assert_eq!(current.to_base(), 42.0 * 1E+3);
/// ```
#[proc_macro_derive(Current, attributes(quantity))]
pub fn current_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    current::derive(&ast)
}

/// Derive macro for the `LuminousIntensity` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `LuminousIntensity` and
///   [`f64`], where such operations make sense:
///   - [`std::ops::Add`] with another `LuminousIntensity`
///   - [`std::ops::AddAssign`] with another `LuminousIntensity`
///   - [`std::ops::Div`] with another `LuminousIntensity`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `LuminousIntensity`
///   - [`std::ops::SubAssign`] with another `LuminousIntensity`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{luminous_intensity::LuminousIntensity, Unit};
/// use newnit_derive::{LuminousIntensity, Unit};
///
/// #[derive(Unit, LuminousIntensity)]
/// #[unit(factor = 1E-3)] // 1 mcd is 0.001 cd
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct MilliCandela(f64);
///
/// let luminous_intensity = MilliCandela(42.0);
/// assert_eq!(luminous_intensity.to_base(), 42.0 * 1E-3);
/// ```
#[proc_macro_derive(LuminousIntensity, attributes(quantity))]
pub fn luminous_intensity_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    luminous_intensity::derive(&ast)
}

/// Derive macro for the `SubstanceAmount` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `SubstanceAmount` and
///   [`f64`], where such operations make sense:
///   - [`std::ops::Add`] with another `SubstanceAmount`
///   - [`std::ops::AddAssign`] with another `SubstanceAmount`
///   - [`std::ops::Div`] with another `SubstanceAmount`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `SubstanceAmount`
///   - [`std::ops::SubAssign`] with another `SubstanceAmount`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{substance_amount::SubstanceAmount, Unit};
/// use newnit_derive::{SubstanceAmount, Unit};
///
/// #[derive(Unit, SubstanceAmount)]
/// #[unit(factor = 1E+18)] // 1 Emol is 3600.0 mols
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct ExaMole(f64);
///
/// let substance_amount = ExaMole(42.0);
/// assert_eq!(substance_amount.to_base(), 42.0 * 1E+18);
/// ```
#[proc_macro_derive(SubstanceAmount, attributes(quantity))]
pub fn substance_amount_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    substance_amount::derive(&ast)
}

/// Derive macro for the `Temperature` trait.
///
/// This trait requires `Unit` as a supertrait.
///
/// You can opt in for additional implementations of one or more of:
/// - [`From`]
/// - [`PartialEq`]
/// - select [`std::ops`] operations with other units of `Temperature` and
///   [`f64`], where such operations make sense:
///   - [`std::ops::Add`] with another `Temperature`
///   - [`std::ops::AddAssign`] with another `Temperature`
///   - [`std::ops::Div`] with another `Temperature`
///   - [`std::ops::Div`] with [`f64`]
///   - [`std::ops::DivAssign`] with [`f64`]
///   - [`std::ops::Mul`] with [`f64`]
///   - [`std::ops::MulAssign`] with [`f64`]
///   - [`std::ops::Neg`]
///   - [`std::ops::Sub`] with another `Temperature`
///   - [`std::ops::SubAssign`] with another `Temperature`
///
/// by specifying them in an (optional) `#[quantity()]` macro attribute.
/// /// # Examples:
/// ```
/// use newnit::{temperature::Temperature, Unit};
/// use newnit_derive::{Temperature, Unit};
///
/// #[derive(Unit, Temperature)]
/// #[unit(factor = 0.556, offset = 255.372)] // 1˚F is around 0.556 + 255.372 K
/// #[quantity(ops)]  // opt in to provided Ops implementations, but not to the From implementation
/// struct Fahrenheit(f64);
///
/// let temperature = Fahrenheit(42.0);
/// assert_eq!(temperature.to_base(), 42.0 * 0.556 + 255.372);
/// ```
#[proc_macro_derive(Temperature, attributes(quantity))]
pub fn temperature_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Failed to parse input code.");
    temperature::derive(&ast)
}
