//! Units of volume.
//!
//! This module contains predefined newtypes for units of area as defined in
//! the following systems:
//! - [`customary`] - US Customary units
//! - [`imperial`] - British Imperial units
//! - [`metric`] - International System of Units (SI)

pub mod customary;
pub mod imperial;
pub mod metric;

pub trait Volume {}
