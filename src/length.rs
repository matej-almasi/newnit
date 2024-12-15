//! Units of length.
//!
//! This module contains predefined newtypes for units of length as defined in
//! the following systems:
//! - [`astronomical`] - units commonly used in Astronomy
//! - [`imperial`] - British Imperial units
//! - [`metric`] - International System of Units (SI)
//! - [`nautical`] - International nautical units

pub mod astronomical;
pub mod imperial;
pub mod metric;
pub mod nautical;

pub trait Length {}
