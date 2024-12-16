//! Units of time.
//!
//! This module contains predefined newtypes for units of time as defined in
//! the International System of Units (SI).

pub mod metric {
    use super::Time;
    use crate::unit;
    use crate::Unit;

    unit!(QuettaSecond, 1E+30, Time);
    unit!(RonnaSecond, 1E+27, Time);
    unit!(YottaSecond, 1E+24, Time);
    unit!(ZettaSecond, 1E+21, Time);
    unit!(ExaSecond, 1E+18, Time);
    unit!(PetaSecond, 1E+15, Time);
    unit!(TeraSecond, 1E+12, Time);
    unit!(GigaSecond, 1E+9, Time);
    unit!(MegaSecond, 1E+6, Time);
    unit!(KiloSecond, 1E+3, Time);
    unit!(Second, 1.0, Time);
    unit!(DeciSecond, 1E-1, Time);
    unit!(CentiSecond, 1E-2, Time);
    unit!(MilliSecond, 1E-3, Time);
    unit!(MicroSecond, 1E-6, Time);
    unit!(NanoSecond, 1E-9, Time);
    unit!(PicoSecond, 1E-12, Time);
    unit!(FemtoSecond, 1E-15, Time);
    unit!(AttoSecond, 1E-18, Time);
    unit!(ZeptoSecond, 1E-21, Time);
    unit!(YoctoSecond, 1E-24, Time);
    unit!(RontoSecond, 1E-27, Time);
    unit!(QuectoSecond, 1E-30, Time);

    unit!(Minute, 60.0, Time);
    unit!(Hour, 3600.0, Time);
    unit!(Day, 86_400.0, Time);
    unit!(Week, 604_800.0, Time);
}

pub trait Time {}
