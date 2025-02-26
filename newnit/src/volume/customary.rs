//! US Customary units of volume
//!
//! This module contains predefined newtypes for units of Volume as defined in
//! the US Customary system. The base unit is the cubic meter.
//!
//! Since the units of length are shared with the British Imperial system,
//! refer to [`volume::imperial`] for units that are cubes of length
//! (e.g. CubicInch, CubicFoot etc.).
//!
//! [`volume::imperial`]: super::imperial

use super::Volume;
use crate::{Unit, make_unit};
use newnit_derive::{Unit, Volume};

// acre ft
make_unit!(AcreFoot, 1_233.482, Volume);

// tsp
make_unit!(TeaSpoon, 4.928_921_593_75e-6, Volume);

// tbsp
make_unit!(TableSpoon, 14.786_764_781_25e-6, Volume);

// US fl oz
make_unit!(FluidOunce, 29.573_529_562_5e-6, Volume);

// US gi
make_unit!(Gill, 118.294_118_25e-6, Volume);

// c
make_unit!(Cup, 236.588_236_5e-6, Volume);

// US pt
make_unit!(Pint, 0.473176473e-3, Volume);

// US qt
make_unit!(Quarter, 0.946_352_946e-3, Volume);

// US gal
make_unit!(Gallon, 3.785_411_784e-3, Volume);

// bbl
make_unit!(Barrel, 158.987_294_928e-3, Volume);
