//! British Imperial units of area
//!
//! This module contains predefined newtypes for units of area as defined in
//! the British Imperial system. The base unit is the square meter.

use super::Area;
use crate::unit;
use crate::Unit;

// Trivial squares of units of length
unit!(SquareInch, 0.0254 * 0.0254, Area);
unit!(SquareFoot, 0.3048 * 0.3048, Area);
unit!(SquareYard, 0.9144 * 0.9144, Area);
unit!(SquareChain, 20.1168 * 20.1168, Area);
unit!(SquareFurlong, 201.168 * 201.168, Area);
unit!(SquareMile, 1_609.344 * 1_609.344, Area);
unit!(SquareLeague, 4_828.032 * 4_828.032, Area);

// Non-trivial units
unit!(Acre, 4_046.856_422_4, Area);
