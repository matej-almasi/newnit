use super::{unit, Unit};

pub trait Mass: Unit {
    fn from_mass(other: &impl Mass) -> Self {
        Unit::from_base(other.to_base())
    }
}

// Metric units
unit!(Zettagram, 1E+18, Mass);
unit!(Exagram, 1E+15, Mass);
unit!(Petagram, 1E+12, Mass);
unit!(Teragram, 1E+9, Mass);
unit!(Gigagram, 1E+6, Mass);
unit!(Megagram, 1E+3, Mass);
unit!(Kilogram, 1E+0, Mass);
unit!(Gram, 1E-3, Mass);
unit!(Milligram, 1E-6, Mass);
unit!(Microgram, 1E-9, Mass);
unit!(Nanogram, 1E-12, Mass);
unit!(Picogram, 1E-15, Mass);
unit!(Femtogram, 1E-18, Mass);
unit!(Attogram, 1E-21, Mass);

pub type Tonne = Megagram;
pub type Kilotonne = Gigagram;
pub type Megatonne = Teragram;
pub type Gigatonne = Petagram;
pub type Teratonne = Exagram;

// Imperial units
