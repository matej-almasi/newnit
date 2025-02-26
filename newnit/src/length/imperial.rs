//! British Imperial units of length
//!
//! This module contains predefined newtypes for units of length as defined in
//! the British Imperial system. The base unit is the meter.

use super::Length;
use crate::{Unit, make_unit};
use newnit_derive::{Length, Unit};

make_unit!(Inch, 0.0254, Length);
make_unit!(Foot, 0.3048, Length);
make_unit!(Yard, 0.9144, Length);
make_unit!(Chain, 20.1168, Length);
make_unit!(Furlong, 201.168, Length);
make_unit!(Mile, 1_609.344, Length);
make_unit!(League, 4_828.032, Length);
