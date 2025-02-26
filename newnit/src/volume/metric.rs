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
use crate::{Unit, make_unit};
use newnit_derive::{Unit, Volume};

// Trivial cubes of units of length
make_unit!(CubicQuettaMeter, 1E+90, 0.0, Volume);
make_unit!(CubicRonnaMeter, 1E+81, 0.0, Volume);
make_unit!(CubicYottaMeter, 1E+72, 0.0, Volume);
make_unit!(CubicZettaMeter, 1E+63, 0.0, Volume);
make_unit!(CubicExaMeter, 1E+54, 0.0, Volume);
make_unit!(CubicPetaMeter, 1E+45, 0.0, Volume);
make_unit!(CubicTeraMeter, 1E+36, 0.0, Volume);
make_unit!(CubicGigaMeter, 1E+27, 0.0, Volume);
make_unit!(CubicMegaMeter, 1E+18, 0.0, Volume);
make_unit!(CubicKiloMeter, 1E+9, 0.0, Volume);
make_unit!(CubicMeter, 1.0, 0.0, Volume);
make_unit!(CubicDeciMeter, 1E-3, 0.0, Volume);
make_unit!(CubicCentiMeter, 1E-6, 0.0, Volume);
make_unit!(CubicMilliMeter, 1E-9, 0.0, Volume);
make_unit!(CubicMicroMeter, 1E-18, 0.0, Volume);
make_unit!(CubicNanoMeter, 1E-27, 0.0, Volume);
make_unit!(CubicPicoMeter, 1E-36, 0.0, Volume);
make_unit!(CubicFemtoMeter, 1E-45, 0.0, Volume);
make_unit!(CubicAttoMeter, 1E-54, 0.0, Volume);
make_unit!(CubicZeptoMeter, 1E-63, 0.0, Volume);
make_unit!(CubicYoctoMeter, 1E-72, 0.0, Volume);
make_unit!(CubicRontoMeter, 1E-81, 0.0, Volume);
make_unit!(CubicQuectoMeter, 1E-90, 0.0, Volume);

make_unit!(CubicAngstrom, 1E-30, 0.0, Volume);

// Non-trivial units
make_unit!(HectoLiter, 1e-1, 0.0, Volume);
pub type Liter = CubicDeciMeter;
make_unit!(DeciLiter, 1e-4, 0.0, Volume);
make_unit!(CentiLiter, 1e-5, 0.0, Volume);
pub type MilliLiter = CubicCentiMeter;
pub type MicroLiter = CubicMilliMeter;
