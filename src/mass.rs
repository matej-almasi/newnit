//! Units of mass.
//!
//! This module contains predefined newtypes for units of mass as defined in the
//! following systems:
//! - [`customary`] - US customary units
//! - [`imperial`] - British imperial units
//! - [`metric`] - International System of Units (SI)
//!
//! The base used for all systems is the (metric) kilogram.
//!
//! # Examples
//! ```
//! use newnit::mass::{self, Mass};
//! use newnit::Unit;
//!
//! // Create a new mass value in pounds
//! let mass = mass::imperial::Pound(2.0);
//!
//! // Convert the mass to grams
//! let grams = mass::metric::Gram::from(&mass);
//!
//! assert_eq!(grams.as_value(), 907.18474);
//! ```

pub mod customary;
pub mod imperial;
pub mod metric;

/// Marker trait for mass units.
pub trait Mass {}
