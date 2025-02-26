//! Metric units of velocity
//!
//! This module contains predefined newtypes for units of velocity as defined in
//! the International System of Units (SI). The base unit is the meter per
//! second.
//!
//! Additional definition provided for the Ångström per second (Å/s) = 1E-10
//! m/s.

use super::Velocity;
use crate::{Unit, make_unit};
use newnit_derive::{Unit, Velocity};

make_unit!(QuettaMeterPerSecond, 1E+30, Velocity);
make_unit!(RonnaMeterPerSecond, 1E+27, Velocity);
make_unit!(YottaMeterPerSecond, 1E+24, Velocity);
make_unit!(ZettaMeterPerSecond, 1E+21, Velocity);
make_unit!(ExaMeterPerSecond, 1E+18, Velocity);
make_unit!(PetaMeterPerSecond, 1E+15, Velocity);
make_unit!(TeraMeterPerSecond, 1E+12, Velocity);
make_unit!(GigaMeterPerSecond, 1E+9, Velocity);
make_unit!(MegaMeterPerSecond, 1E+6, Velocity);
make_unit!(KiloMeterPerSecond, 1E+3, Velocity);
make_unit!(MeterPerSecond, 1.0, Velocity);
make_unit!(DeciMeterPerSecond, 1E-1, Velocity);
make_unit!(CentiMeterPerSecond, 1E-2, Velocity);
make_unit!(MilliMeterPerSecond, 1E-3, Velocity);
make_unit!(MicroMeterPerSecond, 1E-6, Velocity);
make_unit!(NanoMeterPerSecond, 1E-9, Velocity);
make_unit!(PicoMeterPerSecond, 1E-12, Velocity);
make_unit!(FemtoMeterPerSecond, 1E-15, Velocity);
make_unit!(AttoMeterPerSecond, 1E-18, Velocity);
make_unit!(ZeptoMeterPerSecond, 1E-21, Velocity);
make_unit!(YoctoMeterPerSecond, 1E-24, Velocity);
make_unit!(RontoMeterPerSecond, 1E-27, Velocity);
make_unit!(QuectoMeterPerSecond, 1E-30, Velocity);

make_unit!(AngstromPerSecond, 1E-10, Velocity);

make_unit!(KiloMeterPerHour, 2.777_777_777_777_777_8E-1, Velocity);
