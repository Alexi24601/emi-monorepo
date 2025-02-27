//! Trait defining a number.

use common_traits::prelude::Serde;

use super::{Bounded, One, Zero};

/// Trait defining a number.
pub trait Number:
    Copy
    + Clone
    + core::fmt::Display
    + core::fmt::Debug
    + PartialEq
    + PartialOrd
    + One
    + Zero
    + Bounded
    + Serde
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Rem<Output = Self>
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
    + core::ops::DivAssign
    + core::ops::RemAssign
{
}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for isize {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for usize {}
impl Number for f32 {}
impl Number for f64 {}
