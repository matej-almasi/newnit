//! Metric units of velocity
//!
//! This module contains predefined newtypes for units of velocity as defined in
//! the International System of Units (SI). The base unit is the meter per
//! second.
//!
//! Additional definition provided for the Ångström per second (Å/s) = 1E-10
//! m/s.

use super::Velocity;
use crate::unit;
use crate::Unit;

unit!(QuettaMeterPerSecond, 1E+30, 0.0, Velocity);
unit!(RonnaMeterPerSecond, 1E+27, 0.0, Velocity);
unit!(YottaMeterPerSecond, 1E+24, 0.0, Velocity);
unit!(ZettaMeterPerSecond, 1E+21, 0.0, Velocity);
unit!(ExaMeterPerSecond, 1E+18, 0.0, Velocity);
unit!(PetaMeterPerSecond, 1E+15, 0.0, Velocity);
unit!(TeraMeterPerSecond, 1E+12, 0.0, Velocity);
unit!(GigaMeterPerSecond, 1E+9, 0.0, Velocity);
unit!(MegaMeterPerSecond, 1E+6, 0.0, Velocity);
unit!(KiloMeterPerSecond, 1E+3, 0.0, Velocity);
unit!(MeterPerSecond, 1.0, 0.0, Velocity);
unit!(DeciMeterPerSecond, 1E-1, 0.0, Velocity);
unit!(CentiMeterPerSecond, 1E-2, 0.0, Velocity);
unit!(MilliMeterPerSecond, 1E-3, 0.0, Velocity);
unit!(MicroMeterPerSecond, 1E-6, 0.0, Velocity);
unit!(NanoMeterPerSecond, 1E-9, 0.0, Velocity);
unit!(PicoMeterPerSecond, 1E-12, 0.0, Velocity);
unit!(FemtoMeterPerSecond, 1E-15, 0.0, Velocity);
unit!(AttoMeterPerSecond, 1E-18, 0.0, Velocity);
unit!(ZeptoMeterPerSecond, 1E-21, 0.0, Velocity);
unit!(YoctoMeterPerSecond, 1E-24, 0.0, Velocity);
unit!(RontoMeterPerSecond, 1E-27, 0.0, Velocity);
unit!(QuectoMeterPerSecond, 1E-30, 0.0, Velocity);

unit!(AngstromPerSecond, 1E-10, 0.0, Velocity);

unit!(KiloMeterPerHour, 1E+3 / 3_600.0, 0.0, Velocity);
