//! Metric units of volume
//!
//! This module contains predefined newtypes for units of Volume as defined in
//! the International System of Units (SI). The base unit is the cubic meter.
//!
//! Additional definition provided for:
//! - the cubic Ångström (Å^3) = 1E-30 m^3
//!
//! Additional aliases provided for commonly used alternative unit names.

use super::Volume;
use crate::unit;
use crate::Unit;

// Trivial cubes of units of length
unit!(CubicQuettaMeter, 1E+90, 0.0, Volume);
unit!(CubicRonnaMeter, 1E+81, 0.0, Volume);
unit!(CubicYottaMeter, 1E+72, 0.0, Volume);
unit!(CubicZettaMeter, 1E+63, 0.0, Volume);
unit!(CubicExaMeter, 1E+54, 0.0, Volume);
unit!(CubicPetaMeter, 1E+45, 0.0, Volume);
unit!(CubicTeraMeter, 1E+36, 0.0, Volume);
unit!(CubicGigaMeter, 1E+27, 0.0, Volume);
unit!(CubicMegaMeter, 1E+18, 0.0, Volume);
unit!(CubicKiloMeter, 1E+9, 0.0, Volume);
unit!(CubicMeter, 1.0, 0.0, Volume);
unit!(CubicDeciMeter, 1E-3, 0.0, Volume);
unit!(CubicCentiMeter, 1E-6, 0.0, Volume);
unit!(CubicMilliMeter, 1E-9, 0.0, Volume);
unit!(CubicMicroMeter, 1E-18, 0.0, Volume);
unit!(CubicNanoMeter, 1E-27, 0.0, Volume);
unit!(CubicPicoMeter, 1E-36, 0.0, Volume);
unit!(CubicFemtoMeter, 1E-45, 0.0, Volume);
unit!(CubicAttoMeter, 1E-54, 0.0, Volume);
unit!(CubicZeptoMeter, 1E-63, 0.0, Volume);
unit!(CubicYoctoMeter, 1E-72, 0.0, Volume);
unit!(CubicRontoMeter, 1E-81, 0.0, Volume);
unit!(CubicQuectoMeter, 1E-90, 0.0, Volume);

unit!(CubicAngstrom, 1E-30, 0.0, Volume);

// Non-trivial units
unit!(HectoLiter, 1e-1, 0.0, Volume);
pub type Liter = CubicDeciMeter;
unit!(DeciLiter, 1e-4, 0.0, Volume);
unit!(CentiLiter, 1e-5, 0.0, Volume);
pub type MilliLiter = CubicCentiMeter;
pub type MicroLiter = CubicMilliMeter;
