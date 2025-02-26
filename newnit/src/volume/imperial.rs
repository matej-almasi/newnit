//! British Imperial units of volume
//!
//! This module contains predefined newtypes for units of Volume as defined in
//! the British Imperial system. The base unit is the cubic meter.

use super::Volume;
use crate::{Unit, make_unit};
use newnit_derive::{Unit, Volume};

// Trivial cubes of units of length
make_unit!(CubicInch, 64_121.997_435_587, Volume);
make_unit!(CubicFoot, 0.028_316_846_592, Volume);
make_unit!(CubicYard, 0.764_554_857_984, Volume);
make_unit!(CubicChain, 8_140.980_127_813_6, Volume);
make_unit!(CubicFurlong, 8_140_980.127_813_6, Volume);
make_unit!(CubicMile, 4_168_181_825.440_6, Volume);
make_unit!(CubicLeague, 112_540_909_286.9, Volume);

// Non-trivial units

// fl oz
make_unit!(FluidOunce, 28.413_062_5e-6, Volume);

// gi
make_unit!(Gill, 142.065_312_5e-6, Volume);

// pt
make_unit!(Pint, 568.261_25e-6, Volume);

// qt
make_unit!(Quart, 1_136.522_5e-6, Volume);

// gal
make_unit!(Gallon, 4_546.09e-6, Volume);
