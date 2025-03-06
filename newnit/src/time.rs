//! Units of time.
//!
//! This module contains predefined newtypes for units of time as defined in
//! the International System of Units (SI). The base unit is the [`Second`].
//!
//! [`Second`]: metric::Second

use crate::Unit;
use crate::length::metric::Meter;
use crate::velocity::Velocity;

pub trait Time: Unit {
    /// Multiply a unit of time with a unit of velocity.
    fn multiply_velocity(&self, rhs: &dyn Velocity) -> Meter {
        Meter(self.to_base() * rhs.to_base())
    }
}

pub mod metric {
    use super::Time;
    use crate::{Unit, make_unit};
    use newnit_derive::{Time, Unit};

    make_unit!(QuettaSecond, 1E+30, Time);
    make_unit!(RonnaSecond, 1E+27, Time);
    make_unit!(YottaSecond, 1E+24, Time);
    make_unit!(ZettaSecond, 1E+21, Time);
    make_unit!(ExaSecond, 1E+18, Time);
    make_unit!(PetaSecond, 1E+15, Time);
    make_unit!(TeraSecond, 1E+12, Time);
    make_unit!(GigaSecond, 1E+9, Time);
    make_unit!(MegaSecond, 1E+6, Time);
    make_unit!(KiloSecond, 1E+3, Time);
    make_unit!(Second, 1.0, Time);
    make_unit!(DeciSecond, 1E-1, Time);
    make_unit!(CentiSecond, 1E-2, Time);
    make_unit!(MilliSecond, 1E-3, Time);
    make_unit!(MicroSecond, 1E-6, Time);
    make_unit!(NanoSecond, 1E-9, Time);
    make_unit!(PicoSecond, 1E-12, Time);
    make_unit!(FemtoSecond, 1E-15, Time);
    make_unit!(AttoSecond, 1E-18, Time);
    make_unit!(ZeptoSecond, 1E-21, Time);
    make_unit!(YoctoSecond, 1E-24, Time);
    make_unit!(RontoSecond, 1E-27, Time);
    make_unit!(QuectoSecond, 1E-30, Time);

    make_unit!(Minute, 60.0, Time);
    make_unit!(Hour, 3600.0, Time);
    make_unit!(Day, 86_400.0, Time);
    make_unit!(Week, 604_800.0, Time);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiply_velocity() {
        let time = metric::Hour(2.0);
        let velocity = crate::velocity::imperial::MilePerHour(3.0);

        let length = time.multiply_velocity(&velocity);
        assert!((length.to_value() - 9656.064).abs() < 1e-5);
    }
}
