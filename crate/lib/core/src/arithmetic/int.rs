use super::*;

macro_rules! const_as {
    ($($n:literal)*) => {
        $(
            paste::paste! {
                const [< AS_ $n >]: Self;
            }
        )*
    };
}

macro_rules! const_as_impl {
    ($($n:literal)*) => {
        $(
            paste::paste! {
                const [< AS_ $n >]: Self = $n;
            }
        )*
    };
}

macro_rules! fn_impls {
    () => {
        fn add(self, rhs: Self) -> Self {
            self.add(rhs)
        }

        fn sub(self, rhs: Self) -> Self {
            self.sub(rhs)
        }

        fn mul(self, rhs: Self) -> Self {
            self.mul(rhs)
        }

        fn div(self, rhs: Self) -> Self {
            self.div(rhs)
        }

        fn shl(self, rhs: Self) -> Self {
            self.shl(rhs)
        }

        fn shr(self, rhs: Self) -> Self {
            self.shr(rhs)
        }

        fn pow(self, exp: u32) -> Self {
            self.pow(exp)
        }

        fn saturating_add(self, rhs: Self) -> Self {
            self.saturating_add(rhs)
        }

        fn saturating_sub(self, rhs: Self) -> Self {
            self.saturating_sub(rhs)
        }

        fn saturating_mul(self, rhs: Self) -> Self {
            self.saturating_mul(rhs)
        }

        fn saturating_div(self, rhs: Self) -> Self {
            self.saturating_div(rhs)
        }

        fn saturating_pow(self, exp: u32) -> Self {
            self.saturating_pow(exp)
        }

        fn wrapping_add(self, rhs: Self) -> Self {
            self.wrapping_add(rhs)
        }

        fn wrapping_sub(self, rhs: Self) -> Self {
            self.wrapping_sub(rhs)
        }

        fn wrapping_mul(self, rhs: Self) -> Self {
            self.wrapping_mul(rhs)
        }

        fn wrapping_div(self, rhs: Self) -> Self {
            self.wrapping_div(rhs)
        }

        fn wrapping_rem(self, rhs: Self) -> Self {
            self.wrapping_rem(rhs)
        }

        fn wrapping_pow(self, exp: u32) -> Self {
            self.wrapping_pow(exp)
        }

        fn wrapping_neg(self) -> Self {
            self.wrapping_neg()
        }

        fn checked_add(self, rhs: Self) -> Option<Self> {
            self.checked_add(rhs)
        }

        fn checked_sub(self, rhs: Self) -> Option<Self> {
            self.checked_sub(rhs)
        }

        fn checked_mul(self, rhs: Self) -> Option<Self> {
            self.checked_mul(rhs)
        }

        fn checked_div(self, rhs: Self) -> Option<Self> {
            self.checked_div(rhs)
        }

        fn checked_shl(self, rhs: u32) -> Option<Self> {
            self.checked_shl(rhs)
        }

        fn checked_shr(self, rhs: u32) -> Option<Self> {
            self.checked_shr(rhs)
        }

        fn checked_pow(self, rhs: u32) -> Option<Self> {
            self.checked_pow(rhs)
        }

        fn checked_ilog(self, base: Self) -> Option<u32> {
            self.checked_ilog(base)
        }

        fn unbounded_shl(self, rhs: u32) -> Self {
            self.unbounded_shl(rhs)
        }

        fn unbounded_shr(self, rhs: u32) -> Self {
            self.unbounded_shr(rhs)
        }

        
    };
}

pub trait Signed where Self: Sized {
    fn checked_abs(self) -> Option<Self>;
}
pub trait Unsigned where Self: Sized {}
pub trait Int where Self: Copy {
    const IS_SIGNED: bool;
    const BITS: u8;
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
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn shl(self, rhs: Self) -> Self;
    fn shr(self, rhs: Self) -> Self;
    fn pow(self, exp: u32) -> Self;
    fn saturating_add(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    fn saturating_mul(self, rhs: Self) -> Self;
    fn saturating_div(self, rhs: Self) -> Self;
    fn saturating_pow(self, exp: u32) -> Self;
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn wrapping_rem(self, rhs: Self) -> Self;
    fn wrapping_pow(self, exp: u32) -> Self;
    fn wrapping_neg(self) -> Self;
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_shl(self, rhs: u32) -> Option<Self>;
    fn checked_shr(self, rhs: u32) -> Option<Self>;
    fn checked_pow(self, rhs: u32) -> Option<Self>;
    fn checked_ilog(self, base: Self) -> Option<u32>;
    fn unbounded_shl(self, rhs: u32) -> Self;
    fn unbounded_shr(self, rhs: u32) -> Self;
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);
    fn overflowing_shl(self, rhs: u32) -> (Self, bool);
    fn overflowing_shr(self, rhs: u32) -> (Self, bool);
    fn overflowing_pow(self, rhs: u32) -> (Self, bool);
    fn overflowing_neg(self) -> (Self, bool);
}

impl Bits8 for i8 {}
impl Signed for i8 {
    fn checked_abs(self) -> Option<Self> {
        self.checked_abs()
    }
}
impl Int for i8 {
    const IS_SIGNED: bool = true;
    const BITS: u8 = 8;
    const MAX: Self = i8::MAX;
    const MIN: Self = i8::MIN;
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
    fn_impls!();
}

impl Bits16 for i16 {}
impl Signed for i16 {
    fn checked_abs(self) -> Option<Self> {
        self.checked_abs()
    }
}
impl Int for i16 {
    const IS_SIGNED: bool = true;
    const BITS: u8 = 16;
    const MAX: Self = i16::MAX;
    const MIN: Self = i16::MIN;
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
    fn_impls!();
}