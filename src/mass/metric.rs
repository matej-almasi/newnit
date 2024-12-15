//! Metric units of mass
//!
//! This module contains predefined newtypes for units of mass as defined in the
//! International System of Units (SI). The base unit is the kilogram.
//!
//! Additional aliases provided for commonly used alternative unit names.

use super::Mass;
use crate::unit;
use crate::Unit;

unit!(QuettaGram, 1E+27, Mass);
unit!(RonnaGram, 1E+24, Mass);
unit!(YottaGram, 1E+21, Mass);
unit!(ZettaGram, 1E+18, Mass);
unit!(ExaGram, 1E+15, Mass);
unit!(PetaGram, 1E+12, Mass);
unit!(TeraGram, 1E+9, Mass);
unit!(GigaGram, 1E+6, Mass);
unit!(MegaGram, 1E+3, Mass);
unit!(KiloGram, 1E+0, Mass);
unit!(Gram, 1E-3, Mass);
unit!(MilliGram, 1E-6, Mass);
unit!(MicroGram, 1E-9, Mass);
unit!(NanoGram, 1E-12, Mass);
unit!(PicoGram, 1E-15, Mass);
unit!(FemtoGram, 1E-18, Mass);
unit!(AttoGram, 1E-21, Mass);
unit!(ZeptoGram, 1E-24, Mass);
unit!(YoctoGram, 1E-27, Mass);
unit!(RontoGram, 1E-30, Mass);
unit!(QuectoGram, 1E-33, Mass);

/// Metric Tonne
pub type Tonne = MegaGram;
pub type KiloTonne = GigaGram;
pub type MegaTonne = TeraGram;
pub type GigaTonne = PetaGram;
pub type TeraTonne = ExaGram;
