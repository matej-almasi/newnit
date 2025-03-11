use newnit::{
    area::{
        imperial::{Acre, SquareFoot, SquareInch, SquareMile, SquareYard},
        metric::{HectAre, SquareMeter},
    },
    length::{
        astronomical::{AstronomicalUnit, LightYear, Parsec},
        imperial::{self, Foot, Inch, Yard},
        metric::Meter,
        nautical,
    },
    mass::{
        customary::ShortTon,
        imperial::{Pound, TroyOunce},
        metric::{Gram, KiloGram, Tonne},
    },
    temperature::{Celsius, Fahrenheit, Kelvin},
    time::metric::{Day, Hour, Minute, Second, Week},
    velocity::{
        imperial::MilePerHour,
        metric::{KiloMeterPerHour, MeterPerSecond},
        nautical::Knot,
    },
    volume::{
        customary::{Barrel, Cup, FluidOunce, Gallon, TableSpoon, TeaSpoon},
        imperial::{CubicFoot, CubicInch},
        metric::{CubicMeter, HectoLiter, Liter},
    },
};

use super::{Quantity, Unit};

pub fn parse_unit(unit: &Unit) -> Quantity {
    match unit {
        // Area
        Unit::SquareMeter => Quantity::Area(Box::<SquareMeter>::default()),
        Unit::SquareInch => Quantity::Area(Box::<SquareInch>::default()),
        Unit::SquareFoot => Quantity::Area(Box::<SquareFoot>::default()),
        Unit::SquareYard => Quantity::Area(Box::<SquareYard>::default()),
        Unit::Acre => Quantity::Area(Box::<Acre>::default()),
        Unit::Hectare => Quantity::Area(Box::<HectAre>::default()),
        Unit::SquareMile => Quantity::Area(Box::<SquareMile>::default()),
        // Mass
        Unit::Gram => Quantity::Mass(Box::<Gram>::default()),
        Unit::Kilogram => Quantity::Mass(Box::<KiloGram>::default()),
        Unit::Tonne => Quantity::Mass(Box::<Tonne>::default()),
        Unit::TroyOunce => Quantity::Mass(Box::<TroyOunce>::default()),
        Unit::Pound => Quantity::Mass(Box::<Pound>::default()),
        Unit::ShortTon => Quantity::Mass(Box::<ShortTon>::default()),
        // Temperature
        Unit::Celsius => Quantity::Temperature(Box::<Celsius>::default()),
        Unit::Fahrenheit => Quantity::Temperature(Box::<Fahrenheit>::default()),
        Unit::Kelvin => Quantity::Temperature(Box::<Kelvin>::default()),
        // Time
        Unit::Second => Quantity::Time(Box::<Second>::default()),
        Unit::Minute => Quantity::Time(Box::<Minute>::default()),
        Unit::Hour => Quantity::Time(Box::<Hour>::default()),
        Unit::Day => Quantity::Time(Box::<Day>::default()),
        Unit::Week => Quantity::Time(Box::<Week>::default()),
        // Velocity
        Unit::MeterPerSecond => Quantity::Velocity(Box::<MeterPerSecond>::default()),
        Unit::KilometerPerHour => Quantity::Velocity(Box::<KiloMeterPerHour>::default()),
        Unit::MilePerHour => Quantity::Velocity(Box::<MilePerHour>::default()),
        Unit::Knot => Quantity::Velocity(Box::<Knot>::default()),
        // Volume
        Unit::CubicMeter => Quantity::Volume(Box::<CubicMeter>::default()),
        Unit::CubicInch => Quantity::Volume(Box::<CubicInch>::default()),
        Unit::CubicFoot => Quantity::Volume(Box::<CubicFoot>::default()),
        Unit::Liter => Quantity::Volume(Box::<Liter>::default()),
        Unit::HectoLiter => Quantity::Volume(Box::<HectoLiter>::default()),
        Unit::TeaSpoon => Quantity::Volume(Box::<TeaSpoon>::default()),
        Unit::TableSpoon => Quantity::Volume(Box::<TableSpoon>::default()),
        Unit::FluidOunce => Quantity::Volume(Box::<FluidOunce>::default()),
        Unit::Cup => Quantity::Volume(Box::<Cup>::default()),
        Unit::Gallon => Quantity::Volume(Box::<Gallon>::default()),
        Unit::Barrel => Quantity::Volume(Box::<Barrel>::default()),
        // Length
        Unit::Meter => Quantity::Length(Box::<Meter>::default()),
        Unit::Inch => Quantity::Length(Box::<Inch>::default()),
        Unit::Foot => Quantity::Length(Box::<Foot>::default()),
        Unit::Yard => Quantity::Length(Box::<Yard>::default()),
        Unit::Mile => Quantity::Length(Box::<imperial::Mile>::default()),
        Unit::NauticalMile => Quantity::Length(Box::<nautical::Mile>::default()),
        Unit::Parsec => Quantity::Length(Box::<Parsec>::default()),
        Unit::LightYear => Quantity::Length(Box::<LightYear>::default()),
        Unit::AstronomicalUnit => Quantity::Length(Box::<AstronomicalUnit>::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_volume_units() {
        assert!(matches!(parse_unit(&Unit::CubicMeter), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::CubicFoot), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::CubicInch), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::Liter), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::HectoLiter), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::TeaSpoon), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::TableSpoon), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::FluidOunce), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::Cup), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::Gallon), Quantity::Volume(_)));
        assert!(matches!(parse_unit(&Unit::Barrel), Quantity::Volume(_)));
    }

    #[test]
    fn test_parse_length_units() {
        assert!(matches!(parse_unit(&Unit::Meter), Quantity::Length(_)));
        assert!(matches!(parse_unit(&Unit::Inch), Quantity::Length(_)));
        assert!(matches!(parse_unit(&Unit::Foot), Quantity::Length(_)));
        assert!(matches!(parse_unit(&Unit::Yard), Quantity::Length(_)));
        assert!(matches!(parse_unit(&Unit::Mile), Quantity::Length(_)));
        assert!(matches!(
            parse_unit(&Unit::NauticalMile),
            Quantity::Length(_)
        ));
        assert!(matches!(parse_unit(&Unit::Parsec), Quantity::Length(_)));
        assert!(matches!(parse_unit(&Unit::LightYear), Quantity::Length(_)));
        assert!(matches!(
            parse_unit(&Unit::AstronomicalUnit),
            Quantity::Length(_)
        ));
    }

    #[test]
    fn test_parse_mass_units() {
        assert!(matches!(parse_unit(&Unit::Gram), Quantity::Mass(_)));
        assert!(matches!(parse_unit(&Unit::Kilogram), Quantity::Mass(_)));
        assert!(matches!(parse_unit(&Unit::Tonne), Quantity::Mass(_)));
        assert!(matches!(parse_unit(&Unit::TroyOunce), Quantity::Mass(_)));
        assert!(matches!(parse_unit(&Unit::Pound), Quantity::Mass(_)));
        assert!(matches!(parse_unit(&Unit::ShortTon), Quantity::Mass(_)));
    }

    #[test]
    fn test_parse_temperature_units() {
        assert!(matches!(
            parse_unit(&Unit::Celsius),
            Quantity::Temperature(_)
        ));
        assert!(matches!(
            parse_unit(&Unit::Fahrenheit),
            Quantity::Temperature(_)
        ));
        assert!(matches!(
            parse_unit(&Unit::Kelvin),
            Quantity::Temperature(_)
        ));
    }

    #[test]
    fn test_parse_time_units() {
        assert!(matches!(parse_unit(&Unit::Second), Quantity::Time(_)));
        assert!(matches!(parse_unit(&Unit::Minute), Quantity::Time(_)));
        assert!(matches!(parse_unit(&Unit::Hour), Quantity::Time(_)));
        assert!(matches!(parse_unit(&Unit::Day), Quantity::Time(_)));
        assert!(matches!(parse_unit(&Unit::Week), Quantity::Time(_)));
    }

    #[test]
    fn test_parse_velocity_units() {
        assert!(matches!(
            parse_unit(&Unit::MeterPerSecond),
            Quantity::Velocity(_)
        ));
        assert!(matches!(
            parse_unit(&Unit::KilometerPerHour),
            Quantity::Velocity(_)
        ));
        assert!(matches!(
            parse_unit(&Unit::MilePerHour),
            Quantity::Velocity(_)
        ));
        assert!(matches!(parse_unit(&Unit::Knot), Quantity::Velocity(_)));
    }

    #[test]
    fn test_parse_area_units() {
        assert!(matches!(parse_unit(&Unit::SquareMeter), Quantity::Area(_)));
        assert!(matches!(parse_unit(&Unit::SquareInch), Quantity::Area(_)));
        assert!(matches!(parse_unit(&Unit::SquareFoot), Quantity::Area(_)));
        assert!(matches!(parse_unit(&Unit::SquareYard), Quantity::Area(_)));
        assert!(matches!(parse_unit(&Unit::Acre), Quantity::Area(_)));
        assert!(matches!(parse_unit(&Unit::Hectare), Quantity::Area(_)));
        assert!(matches!(parse_unit(&Unit::SquareMile), Quantity::Area(_)));
    }
}
