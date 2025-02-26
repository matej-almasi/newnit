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
    use crate::{Unit, make_unit};
    use newnit_derive::{LuminousIntensity, Unit};

    make_unit!(QuettaCandela, 1E+30, 0.0, LuminousIntensity);
    make_unit!(RonnaCandela, 1E+27, 0.0, LuminousIntensity);
    make_unit!(YottaCandela, 1E+24, 0.0, LuminousIntensity);
    make_unit!(ZettaCandela, 1E+21, 0.0, LuminousIntensity);
    make_unit!(ExaCandela, 1E+18, 0.0, LuminousIntensity);
    make_unit!(PetaCandela, 1E+15, 0.0, LuminousIntensity);
    make_unit!(TeraCandela, 1E+12, 0.0, LuminousIntensity);
    make_unit!(GigaCandela, 1E+9, 0.0, LuminousIntensity);
    make_unit!(MegaCandela, 1E+6, 0.0, LuminousIntensity);
    make_unit!(KiloCandela, 1E+3, 0.0, LuminousIntensity);
    make_unit!(Candela, 1.0, 0.0, LuminousIntensity);
    make_unit!(DeciCandela, 1E-1, 0.0, LuminousIntensity);
    make_unit!(CentiCandela, 1E-2, 0.0, LuminousIntensity);
    make_unit!(MilliCandela, 1E-3, 0.0, LuminousIntensity);
    make_unit!(MicroCandela, 1E-6, 0.0, LuminousIntensity);
    make_unit!(NanoCandela, 1E-9, 0.0, LuminousIntensity);
    make_unit!(PicoCandela, 1E-12, 0.0, LuminousIntensity);
    make_unit!(FemtoCandela, 1E-15, 0.0, LuminousIntensity);
    make_unit!(AttoCandela, 1E-18, 0.0, LuminousIntensity);
    make_unit!(ZeptoCandela, 1E-21, 0.0, LuminousIntensity);
    make_unit!(YoctoCandela, 1E-24, 0.0, LuminousIntensity);
    make_unit!(RontoCandela, 1E-27, 0.0, LuminousIntensity);
    make_unit!(QuectoCandela, 1E-30, 0.0, LuminousIntensity);
}
