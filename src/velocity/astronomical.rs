//! Astronomical units of Velocity.
//!
//! This module contains predefined newtypes for units of Velocity used in
//! astronomy as defined by the International Astronomical Union ([IAU]).
//! The base unit is the meter per second.
//!
//! [IAU]: https://www.iau.org/

use super::Velocity;
use crate::unit;
use crate::Unit;

unit!(MegaParsecPerSecond, 3.085_677_581_491_37E+22, 0.0, Velocity);
unit!(KiloParsecPerSecond, 3.085_677_581_491_37E+19, 0.0, Velocity);
unit!(ParsecPerSecond, 3.085_677_581_491_37E+16, 0.0, Velocity);
unit!(LightYearPerSecond, 9.460_730_472_580_8E+15, 0.0, Velocity);
unit!(AstronomicalUnitPerSecond, 1.495_978_707E+11, 0.0, Velocity);
