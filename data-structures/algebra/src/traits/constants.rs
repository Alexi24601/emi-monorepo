//! Submodule providing traits for several common numerical constants.

/// Trait for the number zero.
pub trait Zero {
    /// The zero value.
    const ZERO: Self;
}
/// Trait for the number one.
pub trait One {
    /// The one value.
    const ONE: Self;
}

/// Trait for the number ten.
pub trait Ten {
    /// The ten value.
    const TEN: Self;
}

/// Trait representing a bounded value.
pub trait Bounded {
    /// The minimum value.
    const MIN: Self;
    /// The maximum value.
    const MAX: Self;
}

/// Macro implementing the [`Bounded`], [`Zero`], [`One`], and [`Ten`] traits
/// for numeric types.
macro_rules! impl_numeric_traits {
    ($($t:ty),+) => {
        $(
            impl Zero for $t {
                const ZERO: Self = 0;
            }
            impl One for $t {
                const ONE: Self = 1;
            }
            impl Ten for $t {
                const TEN: Self = 10;
            }
            impl Bounded for $t {
                const MIN: Self = <$t>::MIN;
                const MAX: Self = <$t>::MAX;
            }
        )+
    };
}

/// Macro implementing the [`Bounded`], [`Zero`], [`One`], and [`Ten`] traits
/// for floating-point types.
macro_rules! impl_float_traits {
    ($($t:ty),+) => {
        $(
            impl Zero for $t {
                const ZERO: Self = 0.0;
            }
            impl One for $t {
                const ONE: Self = 1.0;
            }
            impl Ten for $t {
                const TEN: Self = 10.0;
            }
            impl Bounded for $t {
                const MIN: Self = <$t>::MIN;
                const MAX: Self = <$t>::MAX;
            }
        )+
    };
}

impl_numeric_traits!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_float_traits!(f32, f64);
