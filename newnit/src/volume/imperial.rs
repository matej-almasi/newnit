//! British Imperial units of volume
//!
//! This module contains predefined newtypes for units of Volume as defined in
//! the British Imperial system. The base unit is the cubic meter.

use super::Volume;
use crate::unit;
use crate::Unit;

// Trivial cubes of units of length
unit!(CubicInch, 0.0254 * 0.0254 * 0.0254, 0.0, Volume);
unit!(CubicFoot, 0.3048 * 0.3048 * 0.3048, 0.0, Volume);
unit!(CubicYard, 0.9144 * 0.9144 * 0.9144, 0.0, Volume);
unit!(CubicChain, 20.1168 * 20.1168 * 20.1168, 0.0, Volume);
unit!(CubicFurlong, 201.168 * 201.168 * 201.168, 0.0, Volume);
unit!(CubicMile, 1_609.344 * 1_609.344 * 1_609.344, 0.0, Volume);
unit!(CubicLeague, 4_828.032 * 4_828.032 * 4_828.032, 0.0, Volume);

// Non-trivial units

// fl oz
unit!(FluidOunce, 28.413_062_5e-6, 0.0, Volume);

// gi
unit!(Gill, 142.065_312_5e-6, 0.0, Volume);

// pt
unit!(Pint, 568.261_25e-6, 0.0, Volume);

// qt
unit!(Quart, 1_136.522_5e-6, 0.0, Volume);

// gal
unit!(Gallon, 4_546.09e-6, 0.0, Volume);
