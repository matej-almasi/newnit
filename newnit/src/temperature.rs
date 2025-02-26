//! Units of temperature.
//!
//! This module contains predefined newtypes for units of temperature as defined
//! by the following scales: [`Celsius`], [`Kelvin`] (thermodynamic) and
//! [`Fahrenheit`].
//!
//! The base unit is the [`Kelvin`].

pub trait Temperature {}

use crate::{Unit, make_unit};
use newnit_derive::{Temperature, Unit};

make_unit!(Kelvin, 1.0, Temperature);
make_unit!(Celsius, 1.0, 273.15, Temperature);
make_unit!(
    Fahrenheit,
    5.555_555_555_555_555_6E-1,
    255.372_222_222_222_22,
    Temperature
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_celsius_to_kelvin() {
        let celsius = Celsius(12.4);
        let kelvin = Kelvin::from(&celsius);
        assert!((kelvin.value() - 285.55).abs() < 1e-5);
    }

    #[test]
    fn from_celsius_to_fahrenheit() {
        let celsius = Celsius(42.24);
        let fahrenheit = Fahrenheit::from(&celsius);
        assert!((fahrenheit.value() - 108.032).abs() < 1e-5);
    }
}
