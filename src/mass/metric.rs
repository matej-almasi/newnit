//! Metric units of mass
//!
//! This module contains predefined newtypes for units of mass as defined in the
//! International System of Units (SI). The base unit is the kilogram.
//!
//! Additional aliases provided for commonly used alternative unit names.

use super::Mass;
use crate::unit;
use crate::Unit;

unit!(Zettagram, 1E+18, Mass);
unit!(Exagram, 1E+15, Mass);
unit!(Petagram, 1E+12, Mass);
unit!(Teragram, 1E+9, Mass);
unit!(Gigagram, 1E+6, Mass);
unit!(Megagram, 1E+3, Mass);
unit!(Kilogram, 1E+0, Mass);
unit!(Gram, 1E-3, Mass);
unit!(Milligram, 1E-6, Mass);
unit!(Microgram, 1E-9, Mass);
unit!(Nanogram, 1E-12, Mass);
unit!(Picogram, 1E-15, Mass);
unit!(Femtogram, 1E-18, Mass);
unit!(Attogram, 1E-21, Mass);

/// Metric Tonne
pub type Tonne = Megagram;
pub type Kilotonne = Gigagram;
pub type Megatonne = Teragram;
pub type Gigatonne = Petagram;
pub type Teratonne = Exagram;
