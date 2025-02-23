//! Units of volume.
//!
//! This module contains predefined newtypes for units of area as defined in
//! the following systems:
//! - [`customary`] - US Customary units
//! - [`imperial`] - British Imperial units
//! - [`metric`] - International System of Units (SI)

use crate::Unit;
use crate::area::Area;
use crate::area::metric::SquareMeter;
use crate::length::Length;
use crate::length::metric::Meter;

pub mod customary;
pub mod imperial;
pub mod metric;

pub trait Volume: Unit {
    /// Divide a unit of volume by a unit of length
    fn divide_length(&self, rhs: &dyn Length) -> SquareMeter {
        SquareMeter(self.to_base() / rhs.to_base())
    }

    /// Divide a unit of volume by a unit of area
    fn divide_area(&self, rhs: &dyn Area) -> Meter {
        Meter(self.to_base() / rhs.to_base())
    }
}
