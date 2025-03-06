//! Units of area.
//!
//! This module contains predefined newtypes for units of area as defined in
//! the following systems:
//! - [`imperial`] - British Imperial units
//! - [`metric`] - International System of Units (SI)

use crate::Unit;
use crate::length::Length;
use crate::length::metric::Meter;
use crate::volume::metric::CubicMeter;

pub mod imperial;
pub mod metric;

pub trait Area: Unit {
    /// Multiply a unit of area with a unit of length
    fn multiply_length(&self, rhs: &dyn Length) -> CubicMeter {
        CubicMeter(self.to_base() * rhs.to_base())
    }

    /// Divide a unit of area by a unit of length.
    fn divide_length(&self, rhs: &dyn Length) -> Meter {
        Meter(self.to_base() / rhs.to_base())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiply_with_length() {
        let area = metric::SquareMeter(2.0);
        let length = crate::length::imperial::Foot(3.0);

        let volume = area.multiply_length(&length);
        assert!((volume.to_value() - 1.8288).abs() < 1e-5);
    }

    #[test]
    fn divide_by_length() {
        let area = metric::SquareMeter(2.0);
        let length = crate::length::metric::Meter(4.0);

        let result_length = area.divide_length(&length);
        assert!((result_length.to_value() - 0.5).abs() < 1e-5);
    }
}
