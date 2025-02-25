//! Metric units of length
//!
//! This module contains predefined newtypes for units of length as defined in
//! the International System of Units (SI). The base unit is the meter.
//!
//! Additional definition provided for the Ångström (Å) = 1E-10 m.

use super::Length;
use crate::{Unit, make_unit};
use newnit_derive::{Length, Unit};

make_unit!(QuettaMeter, 1E+30, 0.0, Length);
make_unit!(RonnaMeter, 1E+27, 0.0, Length);
make_unit!(YottaMeter, 1E+24, 0.0, Length);
make_unit!(ZettaMeter, 1E+21, 0.0, Length);
make_unit!(ExaMeter, 1E+18, 0.0, Length);
make_unit!(PetaMeter, 1E+15, 0.0, Length);
make_unit!(TeraMeter, 1E+12, 0.0, Length);
make_unit!(GigaMeter, 1E+9, 0.0, Length);
make_unit!(MegaMeter, 1E+6, 0.0, Length);
make_unit!(KiloMeter, 1E+3, 0.0, Length);
make_unit!(Meter, 1.0, 0.0, Length);
make_unit!(DeciMeter, 1E-1, 0.0, Length);
make_unit!(CentiMeter, 1E-2, 0.0, Length);
make_unit!(MilliMeter, 1E-3, 0.0, Length);
make_unit!(MicroMeter, 1E-6, 0.0, Length);
make_unit!(NanoMeter, 1E-9, 0.0, Length);
make_unit!(PicoMeter, 1E-12, 0.0, Length);
make_unit!(FemtoMeter, 1E-15, 0.0, Length);
make_unit!(AttoMeter, 1E-18, 0.0, Length);
make_unit!(ZeptoMeter, 1E-21, 0.0, Length);
make_unit!(YoctoMeter, 1E-24, 0.0, Length);
make_unit!(RontoMeter, 1E-27, 0.0, Length);
make_unit!(QuectoMeter, 1E-30, 0.0, Length);

make_unit!(Angstrom, 1E-10, 0.0, Length);
