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

    /// Get the [`f64`] value stored in the unit.
    fn as_value(&self) -> f64;

    /// Convert this unit to the base unit, such that:  
    /// {base} = {value} * {[`FACTOR`](Unit::FACTOR)}
    fn to_base(&self) -> f64 {
        self.as_value() * Self::FACTOR
    }

    /// Convert a base unit to this unit, such that:  
    /// {value} = {base} / {[`FACTOR`](Unit::FACTOR)}
    fn from_base(base: f64) -> Self {
        Self::from(base / Self::FACTOR)
    }
}

/// Define a new unit of measurement.
///
/// Defines a newtype struct with the given `name`, implementing the [`Unit`]
/// trait (using the provided `factor`), and the `quantity_trait`, signifying
/// the quantity measured by the unit.
///
/// # Example
/// ```
/// pub trait Mass {}
///
/// unit!(Kilogram, 1.0, Mass);
/// unit!(Ounce, 0.028_349_523_1, Mass);
/// ```
#[macro_export]
macro_rules! unit {
    ($name:ident, $factor: expr, $quantity_trait:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name(pub f64);

        impl From<f64> for $name {
            fn from(value: f64) -> Self {
                Self(value)
            }
        }

        impl Unit for $name {
            const FACTOR: f64 = $factor;

            fn as_value(&self) -> f64 {
                self.0
            }
        }

        impl $quantity_trait for $name {}
    };
}
