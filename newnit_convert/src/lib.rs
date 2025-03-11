use newnit::{
    area::Area, current::Current, length::Length, luminous_intensity::LuminousIntensity,
    mass::Mass, substance_amount::SubstanceAmount, temperature::Temperature, time::Time,
    velocity::Velocity, volume::Volume,
};

use clap::ValueEnum;

mod parse_unit;
pub use parse_unit::parse_unit;

mod convert;
pub use convert::convert;

pub enum Quantity {
    Area(Box<dyn Area>),
    Current(Box<dyn Current>),
    Length(Box<dyn Length>),
    LuminousIntensity(Box<dyn LuminousIntensity>),
    Mass(Box<dyn Mass>),
    SubstanceAmount(Box<dyn SubstanceAmount>),
    Temperature(Box<dyn Temperature>),
    Time(Box<dyn Time>),
    Velocity(Box<dyn Velocity>),
    Volume(Box<dyn Volume>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Unit {
    // Area
    SquareMeter,
    SquareInch,
    SquareFoot,
    SquareYard,
    Acre,
    Hectare,
    SquareMile,
    // Mass
    Gram,
    Kilogram,
    Tonne,
    TroyOunce,
    Pound,
    ShortTon,
    // Temperature
    Celsius,
    Fahrenheit,
    Kelvin,
    // Time
    Second,
    Minute,
    Hour,
    Day,
    Week,
    // Velocity
    MeterPerSecond,
    KilometerPerHour,
    MilePerHour,
    Knot,
    // Volume
    CubicMeter,
    CubicInch,
    CubicFoot,
    Liter,
    HectoLiter,
    TeaSpoon,
    TableSpoon,
    FluidOunce,
    Cup,
    Gallon,
    Barrel,
    // Length
    Meter,
    Inch,
    Foot,
    Yard,
    Mile,
    NauticalMile,
    Parsec,
    LightYear,
    AstronomicalUnit,
}
