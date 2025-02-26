//! British Imperial units of mass
//!
//! This module contains predefined newtypes for units of mass as defined in the
//! British Imperial system. The base unit is the metric kilogram.
//!
//! Additional aliases provided for commonly used alternative unit names.

use super::Mass;
use crate::{Unit, make_unit};
use newnit_derive::{Mass, Unit};

// Troy units (precious metals)
// lb t
make_unit!(TroyPound, 0.373_241_721_6, Mass);

// oz t
make_unit!(TroyOunce, 0.031_103_476_8, Mass);

// dwt
make_unit!(Pennyweight, 0.001_555_173_84, Mass);

// gr
make_unit!(Grain, 0.000_064_798_91, Mass);

// Avoirdupois units

// imperial long ton
make_unit!(LongTon, 1_016.046_908_8, Mass);
pub type ImperialTon = LongTon;

// cwt
make_unit!(Hundredweight, 50.802_345_44, Mass);

// qr/ qrt
make_unit!(Quarter, 12.700_586_36, Mass);

// st
make_unit!(Stone, 6.350_293_18, Mass);

// lb
make_unit!(Pound, 0.453_592_37, Mass);

// oz
make_unit!(Ounce, 0.028_349_523_1, Mass);

// dr
make_unit!(Drachm, 0.001_771_845_195_312_5, Mass);

// Other

// slug
make_unit!(Slug, 14.593_902_94, Mass);
