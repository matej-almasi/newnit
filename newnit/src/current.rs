//! Units of current.
//!
//! This module contains predefined newtypes for units of current as defined in
//! the International System of Units (SI). The base unit is the [`Ampere`].
//!
//! [`Ampere`]: metric::Ampere

pub trait Current {}

pub mod metric {
    use super::Current;
    use crate::unit;
    use crate::Unit;

    unit!(QuettaAmpere, 1E+30, 0.0, Current);
    unit!(RonnaAmpere, 1E+27, 0.0, Current);
    unit!(YottaAmpere, 1E+24, 0.0, Current);
    unit!(ZettaAmpere, 1E+21, 0.0, Current);
    unit!(ExaAmpere, 1E+18, 0.0, Current);
    unit!(PetaAmpere, 1E+15, 0.0, Current);
    unit!(TeraAmpere, 1E+12, 0.0, Current);
    unit!(GigaAmpere, 1E+9, 0.0, Current);
    unit!(MegaAmpere, 1E+6, 0.0, Current);
    unit!(KiloAmpere, 1E+3, 0.0, Current);
    unit!(Ampere, 1.0, 0.0, Current);
    unit!(DeciAmpere, 1E-1, 0.0, Current);
    unit!(CentiAmpere, 1E-2, 0.0, Current);
    unit!(MilliAmpere, 1E-3, 0.0, Current);
    unit!(MicroAmpere, 1E-6, 0.0, Current);
    unit!(NanoAmpere, 1E-9, 0.0, Current);
    unit!(PicoAmpere, 1E-12, 0.0, Current);
    unit!(FemtoAmpere, 1E-15, 0.0, Current);
    unit!(AttoAmpere, 1E-18, 0.0, Current);
    unit!(ZeptoAmpere, 1E-21, 0.0, Current);
    unit!(YoctoAmpere, 1E-24, 0.0, Current);
    unit!(RontoAmpere, 1E-27, 0.0, Current);
    unit!(QuectoAmpere, 1E-30, 0.0, Current);
}
