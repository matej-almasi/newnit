//! Metric units of mass
//!
//! This module contains predefined newtypes for units of mass as defined in the
//! International System of Units (SI). The base unit is the kilogram.
//!
//! Additional aliases provided for commonly used alternative unit names.

use super::Mass;
use crate::{Unit, make_unit};
use newnit_derive::{Mass, Unit};

make_unit!(QuettaGram, 1E+27, 0.0, Mass);
make_unit!(RonnaGram, 1E+24, 0.0, Mass);
make_unit!(YottaGram, 1E+21, 0.0, Mass);
make_unit!(ZettaGram, 1E+18, 0.0, Mass);
make_unit!(ExaGram, 1E+15, 0.0, Mass);
make_unit!(PetaGram, 1E+12, 0.0, Mass);
make_unit!(TeraGram, 1E+9, 0.0, Mass);
make_unit!(GigaGram, 1E+6, 0.0, Mass);
make_unit!(MegaGram, 1E+3, 0.0, Mass);
make_unit!(KiloGram, 1E+0, 0.0, Mass);
make_unit!(Gram, 1E-3, 0.0, Mass);
make_unit!(MilliGram, 1E-6, 0.0, Mass);
make_unit!(MicroGram, 1E-9, 0.0, Mass);
make_unit!(NanoGram, 1E-12, 0.0, Mass);
make_unit!(PicoGram, 1E-15, 0.0, Mass);
make_unit!(FemtoGram, 1E-18, 0.0, Mass);
make_unit!(AttoGram, 1E-21, 0.0, Mass);
make_unit!(ZeptoGram, 1E-24, 0.0, Mass);
make_unit!(YoctoGram, 1E-27, 0.0, Mass);
make_unit!(RontoGram, 1E-30, 0.0, Mass);
make_unit!(QuectoGram, 1E-33, 0.0, Mass);

/// Metric Tonne
pub type Tonne = MegaGram;
pub type KiloTonne = GigaGram;
pub type MegaTonne = TeraGram;
pub type GigaTonne = PetaGram;
pub type TeraTonne = ExaGram;
