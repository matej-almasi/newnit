//! Units of area.
//!
//! This module contains predefined newtypes for units of area as defined in
//! the following systems:
//! - [`imperial`] - British Imperial units
//! - [`metric`] - International System of Units (SI)

pub mod imperial;
pub mod metric;

pub trait Area {}
