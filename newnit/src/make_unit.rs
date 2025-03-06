/// Define a new unit of measurement.
///
/// Defines a newtype struct with the given `name`, implementing the [`Unit`]
/// trait and the `quantity_trait`, signifying the quantity measured by
/// the unit.
///
/// `$factor` and `$offset` are used for conversions between the new unit and
/// the base unit of the same quantity, defined as follows:
///   value_in_base_unit = value_in_this_unit * `$factor` + `$offset`
///
/// [`Unit`]: crate::Unit
#[macro_export]
macro_rules! make_unit {
    ($name:ident, $factor: expr, $offset: expr, $quantity_trait:ident) => {
        #[derive(Unit, Copy, Clone, PartialEq, PartialOrd, Debug, Default, $quantity_trait)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[unit(factor = $factor, offset = $offset, display)]
        #[quantity(from, ops)]
        pub struct $name(pub f64);
    };

    ($name:ident, $factor: expr, $quantity_trait:ident) => {
        #[derive(Unit, Copy, Clone, PartialEq, PartialOrd, Debug, Default, $quantity_trait)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[unit(factor = $factor, display)]
        #[quantity(from, ops)]
        pub struct $name(pub f64);
    };
}
