//! Units of substance amount.
//!
//! This module contains predefined newtypes for units of substance amount as
//! defined in the International System of Units (SI). The base unit is the
//! [`Mole`].
//!
//! [`Mole`]: metric::Mole

pub trait SubstanceAmount {}

pub mod metric {
    use super::SubstanceAmount;
    use crate::{Unit, unit};

    make_unit!(QuettaMole, 1E+30, 0.0, SubstanceAmount);
    make_unit!(RonnaMole, 1E+27, 0.0, SubstanceAmount);
    make_unit!(YottaMole, 1E+24, 0.0, SubstanceAmount);
    make_unit!(ZettaMole, 1E+21, 0.0, SubstanceAmount);
    make_unit!(ExaMole, 1E+18, 0.0, SubstanceAmount);
    make_unit!(PetaMole, 1E+15, 0.0, SubstanceAmount);
    make_unit!(TeraMole, 1E+12, 0.0, SubstanceAmount);
    make_unit!(GigaMole, 1E+9, 0.0, SubstanceAmount);
    make_unit!(MegaMole, 1E+6, 0.0, SubstanceAmount);
    make_unit!(KiloMole, 1E+3, 0.0, SubstanceAmount);
    make_unit!(Mole, 1.0, 0.0, SubstanceAmount);
    make_unit!(DeciMole, 1E-1, 0.0, SubstanceAmount);
    make_unit!(CentiMole, 1E-2, 0.0, SubstanceAmount);
    make_unit!(MilliMole, 1E-3, 0.0, SubstanceAmount);
    make_unit!(MicroMole, 1E-6, 0.0, SubstanceAmount);
    make_unit!(NanoMole, 1E-9, 0.0, SubstanceAmount);
    make_unit!(PicoMole, 1E-12, 0.0, SubstanceAmount);
    make_unit!(FemtoMole, 1E-15, 0.0, SubstanceAmount);
    make_unit!(AttoMole, 1E-18, 0.0, SubstanceAmount);
    make_unit!(ZeptoMole, 1E-21, 0.0, SubstanceAmount);
    make_unit!(YoctoMole, 1E-24, 0.0, SubstanceAmount);
    make_unit!(RontoMole, 1E-27, 0.0, SubstanceAmount);
    make_unit!(QuectoMole, 1E-30, 0.0, SubstanceAmount);
}
