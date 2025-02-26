/// Define a new unit of measurement.
///
/// Defines a newtype struct with the given `name`, implementing the [`Unit`]
/// trait and the `quantity_trait` marker, signifying the quantity measured by
/// the unit.
///
/// `$factor` and `$offset` are used for conversions between the new unit and
/// the base unit of the same quantity, defined as follows:
///   value_in_base_unit = value_in_this_unit * `$factor` + `$offset`
///
/// # Examples
///
/// Define and convert between different units of mass:
/// ```
/// use newnit::{Unit, unit};
/// pub trait Mass {}
///
/// // Define two units of mass...
/// make_unit!(Kilogram, 1.0, 0.0, Mass);
/// make_unit!(Ounce, 0.028_349_523_1, 0.0, Mass);
///
/// // ... and convert between them
/// let kilograms = Kilogram(1.0);
/// let ounces = Ounce::from(&kilograms);
/// assert!((ounces.value() - 35.2740) < 1e-5);
/// ```
#[macro_export]
macro_rules! make_unit {
    ($name:ident, $factor: expr, $offset: expr, $quantity_trait:ident) => {
        #[derive(Unit, Debug, Clone, Copy, Default, $quantity_trait)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[unit(factor = $factor, offset = $offset, display)]
        #[quantity(from, partial_eq, ops)]
        pub struct $name(pub f64);
    };
}
