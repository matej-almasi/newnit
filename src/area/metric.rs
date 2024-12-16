//! Metric units of area
//!
//! This module contains predefined newtypes for units of area as defined in
//! the International System of Units (SI). The base unit is the square meter.
//!
//! Additional definition provided for:
//! - the squared Ångström (Å^2) = 1E-20 m^2
//! - the Are and the HectAre

use super::Area;
use crate::unit;
use crate::Unit;

// Trivial squares of units of length
unit!(SquareQuettaMeter, 1E+60, 0.0, Area);
unit!(SquareRonnaMeter, 1E+54, 0.0, Area);
unit!(SquareYottaMeter, 1E+48, 0.0, Area);
unit!(SquareZettaMeter, 1E+42, 0.0, Area);
unit!(SquareExaMeter, 1E+36, 0.0, Area);
unit!(SquarePetaMeter, 1E+30, 0.0, Area);
unit!(SquareTeraMeter, 1E+24, 0.0, Area);
unit!(SquareGigaMeter, 1E+18, 0.0, Area);
unit!(SquareMegaMeter, 1E+12, 0.0, Area);
unit!(SquareKiloMeter, 1E+6, 0.0, Area);
unit!(SquareMeter, 1.0, 0.0, Area);
unit!(SquareDeciMeter, 1E-2, 0.0, Area);
unit!(SquareCentiMeter, 1E-4, 0.0, Area);
unit!(SquareMilliMeter, 1E-6, 0.0, Area);
unit!(SquareMicroMeter, 1E-12, 0.0, Area);
unit!(SquareNanoMeter, 1E-18, 0.0, Area);
unit!(SquarePicoMeter, 1E-24, 0.0, Area);
unit!(SquareFemtoMeter, 1E-30, 0.0, Area);
unit!(SquareAttoMeter, 1E-36, 0.0, Area);
unit!(SquareZeptoMeter, 1E-42, 0.0, Area);
unit!(SquareYoctoMeter, 1E-48, 0.0, Area);
unit!(SquareRontoMeter, 1E-54, 0.0, Area);
unit!(SquareQuectoMeter, 1E-60, 0.0, Area);

unit!(SquareAngstrom, 1E-20, 0.0, Area);

// Non-trivial units
unit!(Are, 1E+2, 0.0, Area);
unit!(HectAre, 1E+4, 0.0, Area);
