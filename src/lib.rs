//! A library for working with units of measurement based on the [newtype]
//! pattern.
//!
//! [newtype]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html

pub mod mass;
pub mod unit;

pub use unit::Unit;
