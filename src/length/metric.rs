//! Metric units of length
//!
//! This module contains predefined newtypes for units of length as defined in
//! the International System of Units (SI). The base unit is the meter.
//!
//! Additional definition provided for the Ångström (Å) = 1E-10 m.

use super::Length;
use crate::unit;
use crate::Unit;

unit!(QuettaMeter, 1E+30, Length);
unit!(RonnaMeter, 1E+27, Length);
unit!(YottaMeter, 1E+24, Length);
unit!(ZettaMeter, 1E+21, Length);
unit!(ExaMeter, 1E+18, Length);
unit!(PetaMeter, 1E+15, Length);
unit!(TeraMeter, 1E+12, Length);
unit!(GigaMeter, 1E+9, Length);
unit!(MegaMeter, 1E+6, Length);
unit!(KiloMeter, 1E+3, Length);
unit!(Meter, 1.0, Length);
unit!(DeciMeter, 1E-1, Length);
unit!(CentiMeter, 1E-2, Length);
unit!(MilliMeter, 1E-3, Length);
unit!(MicroMeter, 1E-6, Length);
unit!(NanoMeter, 1E-9, Length);
unit!(PicoMeter, 1E-12, Length);
unit!(FemtoMeter, 1E-15, Length);
unit!(AttoMeter, 1E-18, Length);
unit!(ZeptoMeter, 1E-21, Length);
unit!(YoctoMeter, 1E-24, Length);
unit!(RontoMeter, 1E-27, Length);
unit!(QuectoMeter, 1E-30, Length);

unit!(Angstrom, 1E-10, Length);
