//! British Imperial units of mass
//!
//! This module contains predefined newtypes for units of mass as defined in the
//! British Imperial system. The base unit is the metric kilogram.
//!
//! Additional aliases provided for commonly used alternative unit names.

use super::Mass;
use crate::unit;
use crate::Unit;

// Troy units (precious metals)
// lb t
unit!(TroyPound, 0.373_241_721_6, 0.0, Mass);

// oz t
unit!(TroyOunce, 0.031_103_476_8, 0.0, Mass);

// dwt
unit!(Pennyweight, 0.001_555_173_84, 0.0, Mass);

// gr
unit!(Grain, 0.000_064_798_91, 0.0, Mass);

// Avoirdupois units

// imperial long ton
unit!(LongTon, 1_016.046_908_8, 0.0, Mass);
pub type ImperialTon = LongTon;

// cwt
unit!(Hundredweight, 50.802_345_44, 0.0, Mass);

// qr/ qrt
unit!(Quarter, 12.700_586_36, 0.0, Mass);

// st
unit!(Stone, 6.350_293_18, 0.0, Mass);

// lb
unit!(Pound, 0.453_592_37, 0.0, Mass);

// oz
unit!(Ounce, 0.028_349_523_1, 0.0, Mass);

// dr
unit!(Drachm, 0.001_771_845_195_312_5, 0.0, Mass);

// Other

// slug
unit!(Slug, 14.593_902_94, 0.0, Mass);
