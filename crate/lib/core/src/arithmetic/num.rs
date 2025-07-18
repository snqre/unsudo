use super::*;

macro_rules! const_as {
    ($($n:literal)*) => {
        paste::paste!(
            $(
                const [< AS_ $n >]: Self;
            )*
        );
    };
}

macro_rules! const_signed_metadata {
    ($size:literal) => {
        paste::paste!(
            const IS_SIGNED: bool = true;
            const BITS: Self = $size;
            const BITS_U128: u128 = $size;
            const BITS_I128: i128 = $size;
            const MAX: Self = [< i $size >]::MAX;
            const MIN: Self = [< i $size >]::MIN;
            const MAX_U128: u128 = [< i $size >]::MAX as u128;
            const MIN_U128: u128 = 0;
            const MAX_I128: i128 = [< i $size >]::MAX as i128;
            const MIN_I128: i128 = [< i $size >]::MIN as i128;
        );
    };
}

macro_rules! const_unsigned_metadata {
    ($size:literal) => {
        paste::paste!(
            const IS_SIGNED: bool = false;
            const BITS: Self = $size;
            const BITS_U128: u128 = $size;
            const BITS_I128: i128 = $size;
            const MAX: Self = [< u $size >]::MAX;
            const MIN: Self = [< u $size >]::MIN;
            const MAX_U128: u128 = [< u $size >]::MAX as u128;
            const MIN_U128: u128 = 0;
            const MAX_I128: i128 = [< u $size >]::MAX as i128;
            const MIN_I128: i128 = 0;
        );
    };
}

macro_rules! const_as_impl {
    ($($n:literal)*) => {
        paste::paste!(
            $(
                const [< AS_ $n >]: Self = $n;
            )*
        );
    };
}

ops!(
    add rhs Self => Self
    sub rhs Self => Self
    mul rhs Self => Self
    div rhs Self => Self
    shl rhs Self => Self
    shr rhs Self => Self
    rem rhs Self => Self
    pow exp u32 => Self
    saturating_add rhs Self => Self
    saturating_sub rhs Self => Self
    saturating_mul rhs Self => Self
    saturating_div rhs Self => Self
    saturating_pow exp u32 => Self
    wrapping_add rhs Self => Self
    wrapping_sub rhs Self => Self
    wrapping_mul rhs Self => Self
    wrapping_div rhs Self => Self
    wrapping_rem rhs Self => Self
    wrapping_pow exp u32 => Self
    checked_add rhs Self => Option<Self>
    checked_sub rhs Self => Option<Self>
    checked_mul rhs Self => Option<Self>
    checked_div rhs Self => Option<Self>
    checked_shl rhs u32 => Option<Self>
    checked_shr rhs u32 => Option<Self>
    checked_pow rhs u32 => Option<Self>
    checked_ilog base Self => Option<u32>
    overflowing_add rhs Self => (Self, bool)
    overflowing_sub rhs Self => (Self, bool)
    overflowing_mul rhs Self => (Self, bool)
    overflowing_div rhs Self => (Self, bool)
    overflowing_rem rhs Self => (Self, bool)
    overflowing_shl rhs u32 => (Self, bool)
    overflowing_shr rhs u32 => (Self, bool)
    overflowing_pow exp u32 => (Self, bool)
    unbound_shl rhs u32 => Self
    unbound_shr rhs u32 => Self
);

ops!(
    wrapping_neg => Self 
    overflowing_neg => (Self, bool)
);

macro_rules! common_impls {
    ($sign:ident $size:literal) => {
        paste::paste!(
            impl ops::Add for [< $sign $size >] {
                add!();
            }

            impl ops::Sub for [< $sign $size >] {
                sub!();
            }

            impl ops::Mul for [< $sign $size >] {
                mul!();
            }

            impl ops::Div for [< $sign $size >] {
                div!();
            }

            impl ops::Shl for [< $sign $size >] {
                shl!();
            }

            impl ops::Shr for [< $sign $size >] {
                shr!();
            }

            impl ops::Rem for [< $sign $size >] {
                rem!();
            }

            impl ops::Pow for [< $sign $size >] {
                pow!();
            }

            impl ops::SaturatingAdd for [< $sign $size >] {
                saturating_add!();
            }

            impl ops::SaturatingSub for [< $sign $size >] {
                saturating_sub!();
            }

            impl ops::SaturatingMul for [< $sign $size >] {
                saturating_mul!();
            }

            impl ops::SaturatingDiv for [< $sign $size >] {
                saturating_div!();
            }

            impl ops::SaturatingPow for [< $sign $size >] {
                saturating_pow!();
            }

            impl ops::WrappingAdd for [< $sign $size >] {
                wrapping_add!();
            }

            impl ops::WrappingSub for [< $sign $size >] {
                wrapping_sub!();
            }

            impl ops::WrappingMul for [< $sign $size >] {
                wrapping_mul!();
            }

            impl ops::WrappingDiv for [< $sign $size >] {
                wrapping_div!();
            }

            impl ops::WrappingRem for [< $sign $size >] {
                wrapping_rem!();
            }

            impl ops::WrappingPow for [< $sign $size >] {
                wrapping_pow!();
            }

            impl ops::WrappingNeg for [< $sign $size >] {
                wrapping_neg!();
            }

            impl ops::CheckedAdd for [< $sign $size >] {
                checked_add!();
            }

            impl ops::CheckedSub for [< $sign $size >] {
                checked_sub!();
            }

            impl ops::CheckedMul for [< $sign $size >] {
                checked_mul!();
            }

            impl ops::CheckedDiv for [< $sign $size >] {
                checked_div!();
            }

            impl ops::CheckedShl for [< $sign $size >] {
                checked_shl!();
            }

            impl ops::CheckedShr for [< $sign $size >] {
                checked_shr!();
            }

            impl ops::CheckedPow for [< $sign $size >] {
                checked_pow!();
            }

            impl ops::CheckedIlog for [< $sign $size >] {
                checked_ilog!();
            }

            impl ops::OverflowingAdd for [< $sign $size >] {
                overflowing_add!();
            }

            impl ops::OverflowingSub for [< $sign $size >] {
                overflowing_sub!();
            }

            impl ops::OverflowingMul for [< $sign $size >] {
                overflowing_mul!();
            }

            impl ops::OverflowingDiv for [< $sign $size >] {
                overflowing_div!();
            }

            impl ops::OverflowingRem for [< $sign $size >] {
                overflowing_rem!();
            }

            impl ops::OverflowingShl for [< $sign $size >] {
                overflowing_shl!();
            }

            impl ops::OverflowingShr for [< $sign $size >] {
                overflowing_shr!();
            }

            impl ops::OverflowingPow for [< $sign $size >] {
                overflowing_pow!();
            }

            impl ops::OverflowingNeg for [< $sign $size >] {
                overflowing_neg!();
            }

            impl ops::UnboundShl for [< $sign $size >] {
                unbound_shl!();
            }

            impl ops::UnboundShr for [< $sign $size >] {
                unbound_shr!();
            }
        );
    };
}

macro_rules! signed_impls {
    ($($size:literal)*) => {
        paste::paste!(
            $(
                impl [< Bits $size >] for [< i $size >] {}
                
                impl Signed for [< i $size >] {
                    fn checked_abs(self) -> Option<Self> {
                        self.checked_abs()
                    }
                }

                impl Int for [< i $size >] {
                    const_signed_metadata!($size);
                    const_as_impl!(
                        0 1 2 3 4 5 6 7 8 9
                        10 11 12 13 14 15 16 17 18 19 
                        20 21 22 23 24 25 26 27 28 29 
                        30 31 32 33 34 35 36 37 38 39
                        40 41 42 43 44 45 46 47 48 49
                        50 51 52 53 54 55 56 57 58 59
                        60 61 62 63 64 65 66 67 68 69
                        70 71 72 73 74 75 76 77 78 79
                        80 81 82 83 84 85 86 87 88 89
                        90 91 92 93 94 95 96 97 98 99
                        100
                    );
                }

                common_impls!(i $size);
            )*
        );
    };
}

macro_rules! unsigned_impls {
    ($($size:literal)*) => {
        paste::paste!(
            $(
                impl [< Bits $size >] for [< u $size >] {}
                impl Unsigned for [< u $size >] {}
                impl Int for [< u $size >] {
                    const_unsigned_metadata!($size);
                    const_as_impl!(
                        0 1 2 3 4 5 6 7 8 9
                        10 11 12 13 14 15 16 17 18 19 
                        20 21 22 23 24 25 26 27 28 29 
                        30 31 32 33 34 35 36 37 38 39
                        40 41 42 43 44 45 46 47 48 49
                        50 51 52 53 54 55 56 57 58 59
                        60 61 62 63 64 65 66 67 68 69
                        70 71 72 73 74 75 76 77 78 79
                        80 81 82 83 84 85 86 87 88 89
                        90 91 92 93 94 95 96 97 98 99
                        100
                    );
                }

                common_impls!(u $size);
            )*
        );
    };
}

pub trait Signed: Sized {
    fn checked_abs(self) -> Option<Self>;
}
pub trait Unsigned: Sized {}

pub trait Int
where 
    Self: Copy,
    Self: Eq,
    Self: PartialEq,
    Self: Ord,
    Self: PartialOrd,
    Self: TryInto<i8>,
    Self: TryInto<i16>,
    Self: TryInto<i32>,
    Self: TryInto<i64>,
    Self: TryInto<i128>,
    Self: TryInto<u8>,
    Self: TryInto<u16>,
    Self: TryInto<u32>,
    Self: TryInto<u64>,
    Self: TryInto<u128>,
    Self: TryFrom<i8>,
    Self: TryFrom<i16>,
    Self: TryFrom<i32>,
    Self: TryFrom<i64>,
    Self: TryFrom<i128>,
    Self: TryFrom<u8>,
    Self: TryFrom<u16>,
    Self: TryFrom<u32>,
    Self: TryFrom<u64>,
    Self: TryFrom<u128>,
    Self: core::ops::Add<Output = Self>,
    Self: core::ops::Sub<Output = Self>,
    Self: core::ops::Mul<Output = Self>,
    Self: core::ops::Div<Output = Self>,
    Self: core::ops::Shl<Output = Self>,
    Self: core::ops::Shr<Output = Self>,
    Self: core::ops::Rem<Output = Self>,
    Self: core::ops::BitAnd<Output = Self>,
    Self: core::ops::BitOr<Output = Self>,
    Self: ops::Add,
    Self: ops::Sub,
    Self: ops::Mul,
    Self: ops::Div,
    Self: ops::Shl,
    Self: ops::Shr,
    Self: ops::Rem,
    Self: ops::Pow,
    Self: ops::SaturatingAdd,
    Self: ops::SaturatingSub,
    Self: ops::SaturatingMul,
    Self: ops::SaturatingDiv,
    Self: ops::SaturatingPow,
    Self: ops::WrappingAdd,
    Self: ops::WrappingSub,
    Self: ops::WrappingMul,
    Self: ops::WrappingDiv,
    Self: ops::WrappingRem,
    Self: ops::WrappingPow,
    Self: ops::WrappingNeg,
    Self: ops::CheckedAdd,
    Self: ops::CheckedSub,
    Self: ops::CheckedMul,
    Self: ops::CheckedDiv,
    Self: ops::CheckedShl,
    Self: ops::CheckedShr,
    Self: ops::CheckedPow,
    Self: ops::CheckedIlog,
    Self: ops::OverflowingAdd,
    Self: ops::OverflowingSub,
    Self: ops::OverflowingMul,
    Self: ops::OverflowingDiv,
    Self: ops::OverflowingRem,
    Self: ops::OverflowingShl,
    Self: ops::OverflowingShr,
    Self: ops::OverflowingPow,
    Self: ops::OverflowingNeg,
    Self: ops::UnboundShl,
    Self: ops::UnboundShr {
    const IS_SIGNED: bool;
    const BITS: Self;
    const BITS_U128: u128;
    const BITS_I128: i128;
    const MAX: Self;
    const MIN: Self;
    const MAX_U128: u128;
    const MIN_U128: u128 = 0;
    const MAX_I128: i128;
    const MIN_I128: i128;
    const_as!(
        0 1 2 3 4 5 6 7 8 9
        10 11 12 13 14 15 16 17 18 19 
        20 21 22 23 24 25 26 27 28 29 
        30 31 32 33 34 35 36 37 38 39
        40 41 42 43 44 45 46 47 48 49
        50 51 52 53 54 55 56 57 58 59
        60 61 62 63 64 65 66 67 68 69
        70 71 72 73 74 75 76 77 78 79
        80 81 82 83 84 85 86 87 88 89
        90 91 92 93 94 95 96 97 98 99
        100
    );
}

signed_impls!(
    8 
    16 
    32 
    64 
    128
);

unsigned_impls!(
    8 
    16 
    32 
    64 
    128
);

ops!(
    abs => Self
    tan => Self
    sin => Self
    cos => Self
    tanh => Self
    sinh => Self
    cosh => Self
    atan => Self
    asin => Self
    acos => Self
    ceil => Self
    floor => Self
    trunc => Self
    to_degree => Self
    to_radian => Self
);

macro_rules! impls {
    ($($size:ty)*) => {
        paste::paste!(
            $(
                impl Float for $size {
                    fn powf(self, exp: Self) -> Self {
                        self.powf(exp)
                    }

                    fn powi(self, exp: i32) -> Self {
                        self.powi(exp)
                    }
                }

                impl ops::Add for $size {
                    add!();
                }

                impl ops::Sub for $size {
                    sub!();
                }

                impl ops::Mul for $size {
                    mul!();
                }

                impl ops::Div for $size {
                    div!();
                }

                impl ops::Shl for $size {
                    shl!();
                }

                impl ops::Shr for $size {
                    shr!();
                }

                impl ops::Rem for $size {
                    rem!();
                }

                impl ops::Pow for $size {
                    pow!();
                }

                impl ops::Abs for $size {
                    abs!();
                }

                impl ops::Tan for $size {
                    tan!();
                }

                impl ops::Sin for $size {
                    sin!();
                }

                impl ops::Cos for $size {
                    cos!();
                }

                impl ops::Tanh for $size {
                    tanh!();
                }

                impl ops::Sinh for $size {
                    sinh!();
                }

                impl ops::Cosh for $size {
                    cosh!();
                }

                impl ops::ArcTan for $size {
                    atan!();
                }

                impl ops::ArcSin for $size {
                    asin!();
                }

                impl ops::ArcCos for $size {
                    acos!();
                }

                impl ops::Ceil for $size {
                    ceil!();
                }

                impl ops::Floor for $size {
                    floor!();
                }

                impl ops::Trunc for $size {
                    trunc!();
                }

                impl ops::ToDegree for $size {
                    to_degree!();
                }

                impl ops::ToRadian for $size {
                    to_radian!();
                }
            )*
        );
    };
}

pub trait Float 
where
    Self: Sized,
    Self: Copy,
    Self: ops::ArcTan,
    Self: ops::ArcSin,
    Self: ops::ArcCos,
    Self: ops::Tanh,
    Self: ops::Sinh,
    Self: ops::Cosh,
    Self: ops::Tan,
    Self: ops::Sin,
    Self: ops::Cos,
    Self: ops::Ceil,
    Self: ops::Floor,
    Self: ops::Trunc,
    Self: core::ops::Add<Output=Self>,
    Self: core::ops::Sub<Output=Self>,
    Self: core::ops::Mul<Output=Self>,
    Self: core::ops::Div<Output=Self>,
    Self: core::ops::Rem<Output=Self>,
    Self: ops::Add,
    Self: ops::Sub,
    Self: ops::Mul,
    Self: ops::Div,
    Self: ops::Rem,
    Self: ops::ToDegree,
    Self: ops::ToRadian,
    Self: ops::Abs {
    fn powf(self, exp: Self) -> Self;
    fn powi(self, exp: i32) -> Self;
}

impls!(
    f32 
    f64
);
impl Bits32 for f32 {}
impl Bits64 for f64 {}



macro_rules! num_impls {
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

num_impls!(
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