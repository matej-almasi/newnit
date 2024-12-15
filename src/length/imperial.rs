//! British Imperial units of length
//!
//! This module contains predefined newtypes for units of length as defined in
//! the British Imperial system. The base unit is the metric meter.

use super::Length;
use crate::unit;
use crate::Unit;

unit!(Inch, 0.0254, Length);
unit!(Foot, 0.3048, Length);
unit!(Yard, 0.9144, Length);
unit!(Chain, 20.1168, Length);
unit!(Furlong, 201.168, Length);
unit!(Mile, 1_609.344, Length);
unit!(League, 4_828.032, Length);
