//! Metric units of area
//!
//! This module contains predefined newtypes for units of area as defined in
//! the International System of Units (SI). The base unit is the square meter.
//!
//! Additional definition provided for:
//! - the squared Ångström (Å^2) = 1E-20 m^2
//! - the Are and the HectAre

use super::Area;
use crate::{Unit, make_unit};
use newnit_derive::{Area, Unit};

// Trivial squares of units of length
make_unit!(SquareQuettaMeter, 1E+60, Area);
make_unit!(SquareRonnaMeter, 1E+54, Area);
make_unit!(SquareYottaMeter, 1E+48, Area);
make_unit!(SquareZettaMeter, 1E+42, Area);
make_unit!(SquareExaMeter, 1E+36, Area);
make_unit!(SquarePetaMeter, 1E+30, Area);
make_unit!(SquareTeraMeter, 1E+24, Area);
make_unit!(SquareGigaMeter, 1E+18, Area);
make_unit!(SquareMegaMeter, 1E+12, Area);
make_unit!(SquareKiloMeter, 1E+6, Area);
make_unit!(SquareMeter, 1.0, Area);
make_unit!(SquareDeciMeter, 1E-2, Area);
make_unit!(SquareCentiMeter, 1E-4, Area);
make_unit!(SquareMilliMeter, 1E-6, Area);
make_unit!(SquareMicroMeter, 1E-12, Area);
make_unit!(SquareNanoMeter, 1E-18, Area);
make_unit!(SquarePicoMeter, 1E-24, Area);
make_unit!(SquareFemtoMeter, 1E-30, Area);
make_unit!(SquareAttoMeter, 1E-36, Area);
make_unit!(SquareZeptoMeter, 1E-42, Area);
make_unit!(SquareYoctoMeter, 1E-48, Area);
make_unit!(SquareRontoMeter, 1E-54, Area);
make_unit!(SquareQuectoMeter, 1E-60, Area);

make_unit!(SquareAngstrom, 1E-20, Area);

// Non-trivial units
make_unit!(Are, 1E+2, Area);
make_unit!(HectAre, 1E+4, Area);
