//! Units of current.
//!
//! This module contains predefined newtypes for units of current as defined in
//! the International System of Units (SI). The base unit is the [`Ampere`].
//!
//! [`Ampere`]: metric::Ampere

use crate::Unit;

pub trait Current: Unit {}

pub mod metric {
    use super::Current;
    use crate::{Unit, make_unit};
    use newnit_derive::{Current, Unit};

    make_unit!(QuettaAmpere, 1E+30, Current);
    make_unit!(RonnaAmpere, 1E+27, Current);
    make_unit!(YottaAmpere, 1E+24, Current);
    make_unit!(ZettaAmpere, 1E+21, Current);
    make_unit!(ExaAmpere, 1E+18, Current);
    make_unit!(PetaAmpere, 1E+15, Current);
    make_unit!(TeraAmpere, 1E+12, Current);
    make_unit!(GigaAmpere, 1E+9, Current);
    make_unit!(MegaAmpere, 1E+6, Current);
    make_unit!(KiloAmpere, 1E+3, Current);
    make_unit!(Ampere, 1.0, Current);
    make_unit!(DeciAmpere, 1E-1, Current);
    make_unit!(CentiAmpere, 1E-2, Current);
    make_unit!(MilliAmpere, 1E-3, Current);
    make_unit!(MicroAmpere, 1E-6, Current);
    make_unit!(NanoAmpere, 1E-9, Current);
    make_unit!(PicoAmpere, 1E-12, Current);
    make_unit!(FemtoAmpere, 1E-15, Current);
    make_unit!(AttoAmpere, 1E-18, Current);
    make_unit!(ZeptoAmpere, 1E-21, Current);
    make_unit!(YoctoAmpere, 1E-24, Current);
    make_unit!(RontoAmpere, 1E-27, Current);
    make_unit!(QuectoAmpere, 1E-30, Current);
}
