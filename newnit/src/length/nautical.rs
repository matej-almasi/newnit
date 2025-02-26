//! Nautical units of length.
//!
//! This module contains predefined newtypes for units of length used in
//! marine, air and space navigation. The base unit is the meter.

use super::Length;
use crate::{Unit, make_unit};
use newnit_derive::{Length, Unit};

make_unit!(Fathom, 1.828_8, Length);
make_unit!(Cable, 219.456, Length);
make_unit!(Mile, 1852.0, Length);
