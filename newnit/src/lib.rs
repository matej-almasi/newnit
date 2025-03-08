//! A library providing type safety and ergonomics for working with units of
//! measurement, based on the [newtype] pattern.
//!
//! At the core of this library is the [`Unit`] trait defining the behavior
//! of a unit of measurement, based on the principle that an agreed base unit
//! exists with which from/ to conversions can be made to obtain any given unit.
//!
//! Along with the [`Unit`] trait, multiple other traits are provided which
//! represent individual measurable quantities (e.g. [`length::Length`],
//! [`mass::Mass`]). These traits provide additional semantics that allow
//! correct ant automatic inter-usability of units of the same quantity and
//! protect against inadvertent use of incorrect units at compile time. In
//! addition, these quantity traits provide useful and valid mathematical
//! operations between the units.
//!
//! For each provided quantity trait the library provides their common units
//! (e.g. [`mass::metric::Gram`], [`mass::imperial::Pound`]) and, for the metric
//! units, their multiples (e.g. [`length::metric::KiloMeter`],
//! [`length::metric::MilliMeter`]).
//!
//! In case you didn't find a unit that you need, you can implement the [`Unit`]
//! and the desired quantity trait yourself, or you can use the derive macros
//! from the [`derive`] module (see examples below).
//!
//! If you believe a unit or a quantity is missing, feel free to open an issue
//! or a PR!
//!
//! # Examples
//!
//! Use the predefined units:
//!
//! ```
//! use newnit::length::imperial::Foot;
//! use newnit::substance_amount::metric::KiloMole;
//!
//! let length = Foot(32.0);
//! let oxygen_amount = KiloMole(4.0);
//! ```
//!
//! Get the base representation of a unit:
//!
//! ```
//! use newnit::Unit;
//! use newnit::mass::Mass;
//! use newnit::mass::imperial::Ounce;
//!
//! fn display_weight(weight: &dyn Mass) {
//!     println!("Weight in kilograms is: {}", weight.to_base());
//! }
//!
//! fn main() {
//!     let weight_in_ounces = Ounce(4.0);
//!     display_weight(&weight_in_ounces);
//! }
//! ```
//!
//! Seamlessly convert between (compatible) units...
//!
//! ```
//! use newnit::Unit;
//! use newnit::area::imperial::SquareFoot;
//! use newnit::area::metric::SquareMeter;
//!
//! let bathroom_area = SquareMeter(6.0);
//! let bathroom_area_in_sq_feet = SquareFoot::from(&bathroom_area);
//! ```
//!
//! ... and get compile time protection against inadvertent use:
//!
//! ```compile_fail
//! use newnit::Unit;
//! use newnit::area::metric::SquareMeter;
//! use newnit::mass::metric::KiloGram;
//!
//! let bathroom_area = SquareMeter(6.0);
//!
//! // This is a compile error!
//! let bad_conversion = KiloGram::from(&bathroom_area);
//! ```
//!
//! Use the units in common mathematical operations:
//!
//! ```
//! use newnit::length::imperial::Foot;
//! use newnit::length::metric::Meter;
//!
//! let meters = Meter(4.0);
//! let feet = Foot(2.0);
//!
//! let sum = meters + &feet;
//! ```
//!
//! ```
//! use newnit::length::imperial::Foot;
//! use newnit::length::metric::Meter;
//!
//! let meters = Meter(4.0);
//!
//! let times_three = 3.0 * meters;
//!
//! let divided_by_negative_five = meters / -5.0;
//! ```
//!
//! ```
//! use newnit::length::Length;
//! use newnit::length::metric::Meter;
//! use newnit::time::metric::Second;
//! use newnit::velocity::metric::MeterPerSecond;
//!
//! let meters = Meter(4.0);
//! let seconds = Second(3.1);
//!
//! let speed: MeterPerSecond = meters.divide_time(&seconds);
//! ```
//!
//! Define your own unit with a derive macro:
//!
//! ```
//! use newnit::Unit;
//! use newnit::derive::{Mass, Unit};
//! use newnit::mass::Mass;
//!
//! #[derive(Unit, Mass)]
//! #[unit(factor = 6000.0)]
//! #[quantity(ops)] // opt in to provided mathematical Ops implementations
//! struct Elephants(f64);
//! ```
//!
//!  [newtype]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html

pub mod area;
pub mod current;
pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod substance_amount;
pub mod temperature;
pub mod time;
pub mod unit;
pub mod velocity;
pub mod volume;

pub use newnit_derive as derive;
pub use unit::Unit;

mod make_unit;
