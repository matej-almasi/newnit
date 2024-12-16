//! Unit trait and a macro to define units of measurement.

/// A unit of measurement.
///
/// This trait is used to define units of measurement. It provides a simple
/// conversion mechanism to convert from/ to the 'base' unit, which is arbitrary
/// and up to the implementor (although choosing a SI base is a good idea).
///
/// From the point of view of the trait, a unit is defined by its
/// name, and a conversion [`FACTOR`] to a 'base' unit. It is up to the
/// implementor to additionally mark the implementing type with a trait defining
/// its associated quantity (e.g. `Mass`, `Length`, etc.).
///
/// [`FACTOR`]: Unit::FACTOR
pub trait Unit: Sized + From<f64> {
    /// Conversion factor to the base unit.
    const FACTOR: f64;
    const OFFSET: f64;

    /// Get the [`f64`] value stored in the unit.
    fn as_value(&self) -> f64;

    /// Convert this unit to the base unit, such that:  
    /// {base} =
    ///   {value} * {[`FACTOR`](Unit::FACTOR)} + {[`OFFSET`](Unit::OFFSET)}
    fn to_base(&self) -> f64 {
        self.as_value() * Self::FACTOR + Self::OFFSET
    }

    /// Convert a base unit to this unit, such that:  
    /// {value} =
    ///   ({base} - {[`OFFSET`](Unit::OFFSET)}) / {[`FACTOR`](Unit::FACTOR)}
    fn from_base(base: f64) -> Self {
        Self::from((base - Self::OFFSET) / Self::FACTOR)
    }
}

/// Define a new unit of measurement.
///
/// Defines a newtype struct with the given `name`, implementing the [`Unit`]
/// trait (using the provided `factor`) and the `quantity_trait` marker,
/// signifying the quantity measured by the unit. `$factor` and `$offset` will
/// be used for trait constants [`Unit::FACTOR`] and [`Unit::OFFSET`] defining
/// the conversion from/ to the base unit (see [`Unit::to_base()`] and
/// [`Unit::from_base()`]).
///
/// Other standard library traits are implemented or derived, most notably:
/// - [`From`] - enabling conversions between units of the same quantity
/// - [`Add`] - overloading the addition `+` operator
///
/// [`Add`]: std::ops::Add
///
/// # Examples
///
/// Define and convert between different units of mass:
/// ```
/// use newnit::{unit, Unit};
/// pub trait Mass {}
///
/// // Define two units of mass...
/// unit!(Kilogram, 1.0, 0.0, Mass);
/// unit!(Ounce, 0.028_349_523_1, 0.0, Mass);
///
/// // ... and convert between them
/// let kilograms = Kilogram(1.0);
/// let ounces = Ounce::from(&kilograms);
/// assert!((ounces.as_value() - 35.2740) < 1e-4);
/// ```
#[macro_export]
macro_rules! unit {
    ($name:ident, $factor: expr, $offset: expr, $quantity_trait:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name(pub f64);

        impl From<f64> for $name {
            fn from(value: f64) -> Self {
                Self(value)
            }
        }

        impl Unit for $name {
            const FACTOR: f64 = $factor;
            const OFFSET: f64 = $offset;

            fn as_value(&self) -> f64 {
                self.0
            }
        }

        impl $quantity_trait for $name {}

        impl<T> From<&T> for $name
        where
            T: Unit + $quantity_trait,
        {
            fn from(other: &T) -> Self {
                Self::from_base(other.to_base())
            }
        }

        impl<T> std::ops::Add<&T> for $name
        where
            T: Unit + $quantity_trait,
        {
            type Output = Self;

            fn add(self, other: &T) -> Self::Output {
                Self::from_base(self.to_base() + other.to_base())
            }
        }
    };
}
