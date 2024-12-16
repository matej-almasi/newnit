//! Units of current.
//!
//! This module contains predefined newtypes for units of current as defined in
//! the International System of Units (SI). The base unit is the [`Ampere`].
//!
//! [`Ampere`]: metric::Ampere

pub mod metric {
    use super::Current;
    use crate::unit;
    use crate::Unit;

    unit!(QuettaAmpere, 1E+30, Current);
    unit!(RonnaAmpere, 1E+27, Current);
    unit!(YottaAmpere, 1E+24, Current);
    unit!(ZettaAmpere, 1E+21, Current);
    unit!(ExaAmpere, 1E+18, Current);
    unit!(PetaAmpere, 1E+15, Current);
    unit!(TeraAmpere, 1E+12, Current);
    unit!(GigaAmpere, 1E+9, Current);
    unit!(MegaAmpere, 1E+6, Current);
    unit!(KiloAmpere, 1E+3, Current);
    unit!(Ampere, 1.0, Current);
    unit!(DeciAmpere, 1E-1, Current);
    unit!(CentiAmpere, 1E-2, Current);
    unit!(MilliAmpere, 1E-3, Current);
    unit!(MicroAmpere, 1E-6, Current);
    unit!(NanoAmpere, 1E-9, Current);
    unit!(PicoAmpere, 1E-12, Current);
    unit!(FemtoAmpere, 1E-15, Current);
    unit!(AttoAmpere, 1E-18, Current);
    unit!(ZeptoAmpere, 1E-21, Current);
    unit!(YoctoAmpere, 1E-24, Current);
    unit!(RontoAmpere, 1E-27, Current);
    unit!(QuectoAmpere, 1E-30, Current);
}

pub trait Current {}
