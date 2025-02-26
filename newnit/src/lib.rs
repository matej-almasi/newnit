//! A library for working with units of measurement based on the [newtype]
//! pattern.
//!
//! [newtype]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html

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
