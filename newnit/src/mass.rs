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
//! use newnit::Unit;
//! use newnit::mass::{self, Mass};
//!
//! // Create a new mass value in pounds
//! let mass = mass::imperial::Pound(2.0);
//!
//! // Convert the mass to grams
//! let grams = mass::metric::Gram::from(&mass);
//!
//! assert!((grams.to_value() - 907.1847).abs() < 1e-4);
//! ```

use crate::Unit;

pub mod customary;
pub mod imperial;
pub mod metric;

/// Marker trait for mass units.
pub trait Mass: Unit {}
