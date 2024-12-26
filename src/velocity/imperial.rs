//! British Imperial units of velocity
//!
//! This module contains predefined newtypes for units of Velocity based on
//! units of length defined in the British Imperial system. The base unit is
//! the meter per second.

use super::Velocity;
use crate::unit;
use crate::Unit;

unit!(InchPerSecond, 0.0254, 0.0, Velocity);
unit!(FootPerSecond, 0.3048, 0.0, Velocity);
unit!(YardPerSecond, 0.9144, 0.0, Velocity);
unit!(ChainPerSecond, 20.1168, 0.0, Velocity);
unit!(FurlongPerSecond, 201.168, 0.0, Velocity);
unit!(MilePerSecond, 1_609.344, 0.0, Velocity);
unit!(LeaguePerSecond, 4_828.032, 0.0, Velocity);

unit!(MilePerHour, 0.44704, 0.0, Velocity);
