//! British Imperial units of area
//!
//! This module contains predefined newtypes for units of area as defined in
//! the British Imperial system. The base unit is the square meter.

use super::Area;
use crate::{Unit, make_unit};
use newnit_derive::{Area, Unit};

// Trivial squares of units of length
make_unit!(SquareInch, 0.000_645_16, 0.0, Area);
make_unit!(SquareFoot, 0.092_903_04, 0.0, Area);
make_unit!(SquareYard, 0.836_127_36, 0.0, Area);
make_unit!(SquareChain, 404.685_642_24, 0.0, Area);
make_unit!(SquareFurlong, 40_468.564_224, 0.0, Area);
make_unit!(SquareMile, 2_589_988.110_34, 0.0, Area);
make_unit!(SquareLeague, 23_309_892.993_0, 0.0, Area);

// Non-trivial units
make_unit!(Acre, 4_046.856_422_4, 0.0, Area);
