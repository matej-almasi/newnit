//! Astronomical units of length.
//!
//! This module contains predefined newtypes for units of length used in
//! astronomy as defined by the International Astronomical Union ([IAU]).
//! The base unit is the meter.
//!
//! [IAU]: https://www.iau.org/

use super::Length;
use crate::unit;
use crate::Unit;

make_unit!(MegaParsec, 3.085_677_581_491_37E+22, 0.0, Length);
make_unit!(KiloParsec, 3.085_677_581_491_37E+19, 0.0, Length);
make_unit!(Parsec, 3.085_677_581_491_37E+16, 0.0, Length);
make_unit!(LightYear, 9.460_730_472_580_8E+15, 0.0, Length);
make_unit!(AstronomicalUnit, 1.495_978_707E+11, 0.0, Length);
