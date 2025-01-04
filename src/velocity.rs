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

use crate::{length::metric::Meter, time::Time, Unit};

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
