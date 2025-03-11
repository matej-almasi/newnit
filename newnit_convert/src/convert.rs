use std::error::Error;

use crate::Quantity;

pub fn convert(amount: f64, from: Quantity, to: Quantity) -> Result<f64, Box<dyn Error>> {
    match from {
        Quantity::Area(mut unit_from) => {
            let Quantity::Area(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
        Quantity::Current(mut unit_from) => {
            let Quantity::Current(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
        Quantity::Length(mut unit_from) => {
            let Quantity::Length(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
        Quantity::LuminousIntensity(mut unit_from) => {
            let Quantity::LuminousIntensity(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
        Quantity::Mass(mut unit_from) => {
            let Quantity::Mass(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
        Quantity::SubstanceAmount(mut unit_from) => {
            let Quantity::SubstanceAmount(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
        Quantity::Temperature(mut unit_from) => {
            let Quantity::Temperature(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
        Quantity::Time(mut unit_from) => {
            let Quantity::Time(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
        Quantity::Velocity(mut unit_from) => {
            let Quantity::Velocity(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
        Quantity::Volume(mut unit_from) => {
            let Quantity::Volume(mut unit_to) = to else {
                return Err("Units don't match!".into());
            };
            unit_from.set_value(amount);
            unit_to.set_from_base(unit_from.to_base());
            Ok(unit_to.to_value())
        }
    }
}
