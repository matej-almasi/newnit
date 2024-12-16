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
unit!(SquareQuettaMeter, 1E+60, Area);
unit!(SquareRonnaMeter, 1E+54, Area);
unit!(SquareYottaMeter, 1E+48, Area);
unit!(SquareZettaMeter, 1E+42, Area);
unit!(SquareExaMeter, 1E+36, Area);
unit!(SquarePetaMeter, 1E+30, Area);
unit!(SquareTeraMeter, 1E+24, Area);
unit!(SquareGigaMeter, 1E+18, Area);
unit!(SquareMegaMeter, 1E+12, Area);
unit!(SquareKiloMeter, 1E+6, Area);
unit!(SquareMeter, 1.0, Area);
unit!(SquareDeciMeter, 1E-2, Area);
unit!(SquareCentiMeter, 1E-4, Area);
unit!(SquareMilliMeter, 1E-6, Area);
unit!(SquareMicroMeter, 1E-12, Area);
unit!(SquareNanoMeter, 1E-18, Area);
unit!(SquarePicoMeter, 1E-24, Area);
unit!(SquareFemtoMeter, 1E-30, Area);
unit!(SquareAttoMeter, 1E-36, Area);
unit!(SquareZeptoMeter, 1E-42, Area);
unit!(SquareYoctoMeter, 1E-48, Area);
unit!(SquareRontoMeter, 1E-54, Area);
unit!(SquareQuectoMeter, 1E-60, Area);

unit!(SquareAngstrom, 1E-20, Area);

// Non-trivial units
unit!(Are, 1E+2, Area);
unit!(HectAre, 1E+4, Area);
