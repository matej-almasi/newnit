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
