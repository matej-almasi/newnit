//! Units of velocity.
//!
//! This module contains predefined newtypes for units of velocity, derived by
//! dividing units of length with units of time (suitable units are used).
//!
//! The units of length come from the following systems, which determine the
//! derived systems of velocity:
//! - [`astronomical`] - units commonly used in Astronomy
//! - [`imperial`] - British Imperial units
//! - [`metric`] - International System of Units (SI)
//! - [`nautical`] - International nautical units

use crate::Unit;
use crate::length::metric::Meter;
use crate::time::Time;

pub mod astronomical;
pub mod imperial;
pub mod metric;
pub mod nautical;

/// Types that are units of velocity.
///
/// Provides various multiplication/ division methods, resulting in other units
/// of quantity, based on the right hand side (`rhs`) in the multiplication.
pub trait Velocity: Unit {
    /// Multiply a unit of velocity with a unit of time.
    fn multiply_time(&self, rhs: &dyn Time) -> Meter {
        Meter(self.to_base() * rhs.to_base())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiply_with_time() {
        let velocity = metric::MeterPerSecond(2.0);
        let time = crate::time::metric::Hour(3.0);

        let length = velocity.multiply_time(&time);
        assert!((length.value() - 21600.0).abs() < 1e-5);
    }
}
