use super::*;

macro_rules! impls {
    ($($size:ty as $bits:literal)*) => {
        paste::paste!(
            $(
                impl Num for $size {
                    const BITS: u8 = $bits;
                }
            )*
        );
    };
}

pub trait Num 
where
    Self: Copy,
    Self: PartialEq,
    Self: PartialOrd,
    Self: core::ops::Add<Output = Self>,
    Self: core::ops::Sub<Output = Self>,
    Self: core::ops::Mul<Output = Self>,
    Self: core::ops::Div<Output = Self>,
    Self: core::ops::Rem<Output = Self>,
    Self: ops::Add,
    Self: ops::Sub,
    Self: ops::Mul,
    Self: ops::Div,
    Self: ops::Rem,
    Self: ops::Pow,
    Self: ops::Shl,
    Self: ops::Shr {
    const BITS: u8;
}

impls!(
    f32 as 32
    f64 as 64
    i8 as 8
    i16 as 16
    i32 as 32
    i64 as 64
    i128 as 128
    u8 as 8
    u16 as 16
    u32 as 32
    u64 as 64
    u128 as 128
);