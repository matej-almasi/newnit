//! Unit trait and a macro to define units of measurement.

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
    fn value(&self) -> f64;
}

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
/// assert!((ounces.value() - 35.2740) < 1e-4);
/// ```
#[macro_export]
macro_rules! unit {
    ($name:ident, $factor: expr, $offset: expr, $quantity_trait:ident) => {
        #[derive(Debug, Clone, Copy, Default)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name(pub f64);

        impl $name {
            /// Create a representation of a quantity expressed in this unit from its
            /// representation expressed in base units.
            fn from_base(base: f64) -> Self {
                Self((base - $offset) / $factor)
            }
        }

        impl Unit for $name {
            fn to_base(&self) -> f64 {
                self.value() * $factor + $offset
            }

            fn value(&self) -> f64 {
                self.value()
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

        impl<T> std::cmp::PartialEq<T> for $name
        where
            T: Unit + $quantity_trait,
        {
            fn eq(&self, other: &T) -> bool {
                self.to_base() == other.to_base()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value())
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

        impl<T> std::ops::AddAssign<&T> for $name
        where
            T: Unit + $quantity_trait,
        {
            fn add_assign(&mut self, other: &T) {
                self.0 = Self::from_base(self.to_base() + other.to_base()).value();
            }
        }

        impl<T> std::ops::Div<&T> for $name
        where
            T: Unit + $quantity_trait,
        {
            type Output = f64;

            fn div(self, other: &T) -> Self::Output {
                self.to_base() / other.to_base()
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.value())
            }
        }

        impl<T> std::ops::Sub<&T> for $name
        where
            T: Unit + $quantity_trait,
        {
            type Output = Self;

            fn sub(self, other: &T) -> Self::Output {
                Self::from_base(self.to_base() - other.to_base())
            }
        }

        impl<T> std::ops::SubAssign<&T> for $name
        where
            T: Unit + $quantity_trait,
        {
            fn sub_assign(&mut self, other: &T) {
                self.0 = Self::from_base(self.to_base() - other.to_base()).value();
            }
        }
    };
}
