//! A library for working with units of measurement based on the [newtype]
//! pattern.
//!
//! [newtype]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html

pub mod area;
pub mod length;
pub mod mass;
pub mod unit;
pub mod volume;

pub use unit::Unit;
