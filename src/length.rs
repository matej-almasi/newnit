//! Units of length.
//!
//! This module contains predefined newtypes for units of length as defined in
//! the following systems:
//! - [`astronomical`] - units commonly used in Astronomy
//! - [`imperial`] - British Imperial units
//! - [`metric`] - International System of Units (SI)
//! - [`nautical`] - International nautical units

use crate::area::metric::SquareMeter;
use crate::Unit;

pub mod astronomical;
pub mod imperial;
pub mod metric;
pub mod nautical;

/// Types that are units of length.
///
/// Provides various multiplication methods, resulting in other units of
/// quantity, based on the right hand side (`rhs`) in the multiplication.
///
/// # Examples
/// ```
/// use newnit::area::metric::SquareMeter;
/// use newnit::length::{imperial::Foot, metric::Meter, Length};
/// use newnit::Unit;
///
/// let length1 = Meter(4.0);
/// let length2 = Foot(2.0);
///
/// let area = length1.multiply(&length2);
/// assert!((area.value() - 2.4384).abs() < 1e-5);
/// ```
pub trait Length: Unit {
    /// Multiply two units of length.
    fn multiply(&self, rhs: &dyn Length) -> SquareMeter {
        SquareMeter(self.to_base() * rhs.to_base())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiply_with_length() {
        let length1 = metric::Meter(2.0);
        let length2 = imperial::Foot(3.0);

        let area = length1.multiply(&length2);
        assert!((area.value() - 1.8288).abs() < 1e-5);
    }
}
