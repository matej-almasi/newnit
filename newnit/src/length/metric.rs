//! Metric units of length
//!
//! This module contains predefined newtypes for units of length as defined in
//! the International System of Units (SI). The base unit is the meter.
//!
//! Additional definition provided for the Ångström (Å) = 1E-10 m.

use super::Length;
use crate::unit;
use crate::Unit;

unit!(QuettaMeter, 1E+30, 0.0, Length);
unit!(RonnaMeter, 1E+27, 0.0, Length);
unit!(YottaMeter, 1E+24, 0.0, Length);
unit!(ZettaMeter, 1E+21, 0.0, Length);
unit!(ExaMeter, 1E+18, 0.0, Length);
unit!(PetaMeter, 1E+15, 0.0, Length);
unit!(TeraMeter, 1E+12, 0.0, Length);
unit!(GigaMeter, 1E+9, 0.0, Length);
unit!(MegaMeter, 1E+6, 0.0, Length);
unit!(KiloMeter, 1E+3, 0.0, Length);
unit!(Meter, 1.0, 0.0, Length);
unit!(DeciMeter, 1E-1, 0.0, Length);
unit!(CentiMeter, 1E-2, 0.0, Length);
unit!(MilliMeter, 1E-3, 0.0, Length);
unit!(MicroMeter, 1E-6, 0.0, Length);
unit!(NanoMeter, 1E-9, 0.0, Length);
unit!(PicoMeter, 1E-12, 0.0, Length);
unit!(FemtoMeter, 1E-15, 0.0, Length);
unit!(AttoMeter, 1E-18, 0.0, Length);
unit!(ZeptoMeter, 1E-21, 0.0, Length);
unit!(YoctoMeter, 1E-24, 0.0, Length);
unit!(RontoMeter, 1E-27, 0.0, Length);
unit!(QuectoMeter, 1E-30, 0.0, Length);

unit!(Angstrom, 1E-10, 0.0, Length);
