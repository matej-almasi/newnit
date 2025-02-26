//! British Imperial units of velocity
//!
//! This module contains predefined newtypes for units of Velocity based on
//! units of length defined in the British Imperial system. The base unit is
//! the meter per second.

use super::Velocity;
use crate::{Unit, make_unit};
use newnit_derive::{Unit, Velocity};

make_unit!(InchPerSecond, 0.0254, 0.0, Velocity);
make_unit!(FootPerSecond, 0.3048, 0.0, Velocity);
make_unit!(YardPerSecond, 0.9144, 0.0, Velocity);
make_unit!(ChainPerSecond, 20.1168, 0.0, Velocity);
make_unit!(FurlongPerSecond, 201.168, 0.0, Velocity);
make_unit!(MilePerSecond, 1_609.344, 0.0, Velocity);
make_unit!(LeaguePerSecond, 4_828.032, 0.0, Velocity);

make_unit!(MilePerHour, 0.44704, 0.0, Velocity);
