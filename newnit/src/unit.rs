//! Unit trait to define units of measurement.

/// A unit of measurement.
///
/// For the purpose of this crate, a unit of measurement is defined purely by
/// its conversion to a base unit of the same quantity.
///
/// The choice of the base unit is arbitrary and up to the implementor (although
/// choosing a SI base is a good idea).
///
/// It is up to the implementor to additionally mark the implementing type with
/// a trait defining its associated quantity (e.g. `Mass`, `Length`, etc.).
pub trait Unit {
    /// Converts the quantity value represented in this unit to its equivalent
    /// value in the base unit.
    fn to_base(&self) -> f64;

    /// Returns the wrapped [`f64`] value.
    fn to_value(&self) -> f64;

    /// Create a representation of a quantity expressed in this unit from its
    /// value in base units.
    fn from_base(base: f64) -> Self
    where
        Self: Sized;
}
