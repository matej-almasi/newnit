//! Nautical units of velocity.
//!
//! This module contains predefined newtypes for units of velocity used in
//! marine, air and space navigation. The base unit is the meter per second.

use super::Velocity;
use crate::{Unit, unit};

make_unit!(FathomPerSecond, 1.828_8, 0.0, Velocity);
make_unit!(CablePerSecond, 219.456, 0.0, Velocity);
make_unit!(MilePerSecond, 1852.0, 0.0, Velocity);

make_unit!(MilePerHour, 1852.0 / 3_600.0, 0.0, Velocity);

pub type Knot = MilePerHour;
