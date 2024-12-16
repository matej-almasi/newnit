//! Nautical units of length.
//!
//! This module contains predefined newtypes for units of length used in
//! marine, air and space navigation. The base unit is the meter.

use super::Length;
use crate::unit;
use crate::Unit;

unit!(Fathom, 1.828_8, 0.0, Length);
unit!(Cable, 219.456, 0.0, Length);
unit!(Mile, 1852.0, 0.0, Length);
