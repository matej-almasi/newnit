//! Units of length.
//!
//! This module contains predefined newtypes for units of length as defined in
//! the following systems:
//! - [`astronomical`] - units commonly used in Astronomy
//! - [`imperial`] - British Imperial units
//! - [`metric`] - International System of Units (SI)
//! - [`nautical`] - International nautical units

use crate::Unit;
use crate::area::Area;
use crate::area::metric::SquareMeter;
use crate::time::Time;
use crate::time::metric::Second;
use crate::velocity::Velocity;
use crate::velocity::metric::MeterPerSecond;
use crate::volume::metric::CubicMeter;

pub mod astronomical;
pub mod imperial;
pub mod metric;
pub mod nautical;

/// Types that are units of length.
///
/// Provides various multiplication and division methods, resulting in other
/// units of quantity, based on the right hand side (`rhs`) in the
/// multiplication/ division.
///
/// # Examples
/// ```
/// use newnit::Unit;
/// use newnit::area::metric::SquareMeter;
/// use newnit::length::Length;
/// use newnit::length::imperial::Foot;
/// use newnit::length::metric::Meter;
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

    /// Multiply a unit of length with a unit of area.
    fn multiply_area(&self, rhs: &dyn Area) -> CubicMeter {
        CubicMeter(self.to_base() * rhs.to_base())
    }

    /// Divide a unit of length by a unit of time.
    fn divide_time(&self, rhs: &dyn Time) -> MeterPerSecond {
        MeterPerSecond(self.to_base() / rhs.to_base())
    }

    /// Divide a unit of length by a unit of velocity.
    fn divide_velocity(&self, rhs: &dyn Velocity) -> Second {
        Second(self.to_base() / rhs.to_base())
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

    #[test]
    fn multiply_with_area() {
        let length = metric::Meter(2.0);
        let area = crate::area::imperial::SquareInch(2.0);

        let volume = length.multiply_area(&area);
        assert!((volume.value() - 2.58064e-3).abs() < 1e-9);
    }

    #[test]
    fn divide_by_time() {
        let length = metric::Meter(2.0);
        let time = crate::time::metric::Second(3.0);

        let velocity = length.divide_time(&time);
        assert!((velocity.value() - 0.66667).abs() < 1e-5);
    }

    #[test]
    fn divide_by_velocity() {
        let length = metric::Meter(2.0);
        let velocity = crate::velocity::metric::MeterPerSecond(3.0);

        let time = length.divide_velocity(&velocity);
        assert!((time.value() - 0.66667).abs() < 1e-5);
    }
}
