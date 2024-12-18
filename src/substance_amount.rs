//! Units of substance amount.
//!
//! This module contains predefined newtypes for units of substance as defined
//! in the International System of Units (SI). The base unit is the [`Mole`].
//!
//! [`Mole`]: metric::Mole

pub trait SubstanceAmount {}

pub mod metric {
    use super::SubstanceAmount;
    use crate::unit;
    use crate::Unit;

    unit!(QuettaMole, 1E+30, 0.0, SubstanceAmount);
    unit!(RonnaMole, 1E+27, 0.0, SubstanceAmount);
    unit!(YottaMole, 1E+24, 0.0, SubstanceAmount);
    unit!(ZettaMole, 1E+21, 0.0, SubstanceAmount);
    unit!(ExaMole, 1E+18, 0.0, SubstanceAmount);
    unit!(PetaMole, 1E+15, 0.0, SubstanceAmount);
    unit!(TeraMole, 1E+12, 0.0, SubstanceAmount);
    unit!(GigaMole, 1E+9, 0.0, SubstanceAmount);
    unit!(MegaMole, 1E+6, 0.0, SubstanceAmount);
    unit!(KiloMole, 1E+3, 0.0, SubstanceAmount);
    unit!(Mole, 1.0, 0.0, SubstanceAmount);
    unit!(DeciMole, 1E-1, 0.0, SubstanceAmount);
    unit!(CentiMole, 1E-2, 0.0, SubstanceAmount);
    unit!(MilliMole, 1E-3, 0.0, SubstanceAmount);
    unit!(MicroMole, 1E-6, 0.0, SubstanceAmount);
    unit!(NanoMole, 1E-9, 0.0, SubstanceAmount);
    unit!(PicoMole, 1E-12, 0.0, SubstanceAmount);
    unit!(FemtoMole, 1E-15, 0.0, SubstanceAmount);
    unit!(AttoMole, 1E-18, 0.0, SubstanceAmount);
    unit!(ZeptoMole, 1E-21, 0.0, SubstanceAmount);
    unit!(YoctoMole, 1E-24, 0.0, SubstanceAmount);
    unit!(RontoMole, 1E-27, 0.0, SubstanceAmount);
    unit!(QuectoMole, 1E-30, 0.0, SubstanceAmount);
}
