//! US Customary units of mass.
//!
//! This module contains predefined newtypes for units of mass as defined in the
//! US Customary system. The base unit is the metric kilogram.
//!
//! Since the US Customary system is the same as the British Imperial system for
//! mass up to the pound (included), reexports from the `imperial` module are
//! used, where possible.

use super::Mass;
use crate::unit;
use crate::Unit;

// Troy units (precious metals)
pub use super::imperial::Grain;
pub use super::imperial::Pennyweight;
pub use super::imperial::TroyOunce;
pub use super::imperial::TroyPound;

// Avoirdupois units

// long ton
pub type LongTon = super::imperial::LongTon;

// short ton
unit!(ShortTon, 907.184_74, 0.0, Mass);

// long cwt
pub type LongHundredweight = super::imperial::Hundredweight;

// US cwt
unit!(ShortHundredweight, 45.359_237, 0.0, Mass);

pub use super::imperial::Ounce;
pub use super::imperial::Pound;

// dr
pub type Dram = super::imperial::Drachm;
