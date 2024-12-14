pub trait Unit: Sized + From<f64> {
    const FACTOR: f64;

    fn as_value(&self) -> f64;

    fn to_base(&self) -> f64 {
        self.as_value() * Self::FACTOR
    }

    fn from_base(base: f64) -> Self {
        Self::from(base / Self::FACTOR)
    }
}

#[macro_export]
macro_rules! unit {
    ($name:ident, $factor: expr, $trait:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name(pub f64);

        impl From<f64> for $name {
            fn from(value: f64) -> Self {
                Self(value)
            }
        }

        impl Unit for $name {
            const FACTOR: f64 = $factor;

            fn as_value(&self) -> f64 {
                self.0
            }
        }

        impl $trait for $name {}
    };
}
