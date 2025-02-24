//! Units of luminous intensity.
//!
//! This module contains predefined newtypes for units of luminous intensity as
//! defined in the International System of Units (SI). The base unit is the
//! [`Candela`].
//!
//! [`Candela`]: metric::Candela

pub trait LuminousIntensity {}

pub mod metric {
    use super::LuminousIntensity;
    use crate::unit;
    use crate::Unit;

    unit!(QuettaCandela, 1E+30, 0.0, LuminousIntensity);
    unit!(RonnaCandela, 1E+27, 0.0, LuminousIntensity);
    unit!(YottaCandela, 1E+24, 0.0, LuminousIntensity);
    unit!(ZettaCandela, 1E+21, 0.0, LuminousIntensity);
    unit!(ExaCandela, 1E+18, 0.0, LuminousIntensity);
    unit!(PetaCandela, 1E+15, 0.0, LuminousIntensity);
    unit!(TeraCandela, 1E+12, 0.0, LuminousIntensity);
    unit!(GigaCandela, 1E+9, 0.0, LuminousIntensity);
    unit!(MegaCandela, 1E+6, 0.0, LuminousIntensity);
    unit!(KiloCandela, 1E+3, 0.0, LuminousIntensity);
    unit!(Candela, 1.0, 0.0, LuminousIntensity);
    unit!(DeciCandela, 1E-1, 0.0, LuminousIntensity);
    unit!(CentiCandela, 1E-2, 0.0, LuminousIntensity);
    unit!(MilliCandela, 1E-3, 0.0, LuminousIntensity);
    unit!(MicroCandela, 1E-6, 0.0, LuminousIntensity);
    unit!(NanoCandela, 1E-9, 0.0, LuminousIntensity);
    unit!(PicoCandela, 1E-12, 0.0, LuminousIntensity);
    unit!(FemtoCandela, 1E-15, 0.0, LuminousIntensity);
    unit!(AttoCandela, 1E-18, 0.0, LuminousIntensity);
    unit!(ZeptoCandela, 1E-21, 0.0, LuminousIntensity);
    unit!(YoctoCandela, 1E-24, 0.0, LuminousIntensity);
    unit!(RontoCandela, 1E-27, 0.0, LuminousIntensity);
    unit!(QuectoCandela, 1E-30, 0.0, LuminousIntensity);
}
