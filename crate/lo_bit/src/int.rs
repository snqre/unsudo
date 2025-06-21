use super::*;

macro_rules! as_const_sig {
    ($value:literal) => {
        ::paste::paste! {
            const [<AS_ $value>]: Self;
            const [<AS_ $value _U128>]: u128;
            const [<AS_ $value _I128>]: i128;
        }
    };
    ($($value:literal)*) => {
        $(as_const_sig! { $value })*
    };
}

macro_rules! try_into_sig {
    ($size:ty) => {
        ::paste::paste! {
            fn [<into_ $size>](self) -> Result<$size>;
        }   
    };
    ($($size:ty)*) => {
        $(try_into_sig! { $size })*
    };
}

macro_rules! as_const {
    ($value:literal) => {
        ::paste::paste! {
            const [<AS_ $value>]: Self = $value;
            const [<AS_ $value _U128>]: u128 = $value;
            const [<AS_ $value _I128>]: i128 = $value;
        }
    };
    ($($value:literal)*) => {
        $(as_const! { $value })*
    };
}

macro_rules! try_into {
    ($size:ty) => {
        ::paste::paste! {
            fn [<into_ $size>](self) -> Result<$size> {
                self.try_into()
                    .ok()
                    .ok_or(Error::UnsupportedConversion)
            }
        }
    };
    ($($size:ty)*) => {
        $(try_into! { $size })*
    };
}

macro_rules! implementation {
    ($size:ident $bit:literal $signed:expr) => {
        impl Int for $size {
            const IS_SIGNED: bool = $signed;
            const BIT: Self = 8;
            const BIT_U128: u128 = 8;
            const BIT_I128: i128 = 8;
            const MAX: Self = $size::MAX;
            const MAX_U128: u128 = $size::MAX as u128;
            const MAX_I128: i128 = $size::MAX as i128;
            const MIN: Self = $size::MIN;
            const MIN_U128: u128 = $size::MIN as u128;
            const MIN_I128: i128 = $size::MIN as i128;

            as_const! {
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
            }

            #[inline]
            fn from_u128(value: u128) -> Result<Self> {
                <$size>::try_from(value).map_err(|_| Error::UnsupportedConversion)
            }
            
            #[inline]
            fn from_i128(value: i128) -> Result<Self> {
                <$size>::try_from(value).map_err(|_| Error::UnsupportedConversion)
            }

            #[inline]
            fn pow(self, exp: u32) -> Self { 
                self.pow(exp) 
            }

            #[inline]
            fn checked_add(self, rhs: Self) -> Result<Self> { 
                self.checked_add(rhs).ok_or(Error::Overflow)
            }

            #[inline]
            fn checked_sub(self, rhs: Self) -> Result<Self> { 
                self.checked_sub(rhs).ok_or(Error::Underflow)
            }

            #[inline]
            fn checked_mul(self, rhs: Self) -> Result<Self> { 
                self.checked_mul(rhs).ok_or(Error::Overflow)
            }

            #[inline]
            fn checked_div(self, rhs: Self) -> Result<Self> { 
                self.checked_div(rhs).ok_or(Error::DivByZero)
            }

            #[inline]
            fn checked_rem(self, rhs: Self) -> Result<Self> { 
                self.checked_rem(rhs).ok_or(Error::DivByZero)
            }

            #[inline]
            fn checked_pow(self, exp: u32) -> Result<Self> { 
                self.checked_pow(exp).ok_or(Error::Overflow)
            }

            #[inline]
            fn wrapping_add(self, rhs: Self) -> Self { 
                self.wrapping_add(rhs) 
            }

            #[inline]
            fn wrapping_sub(self, rhs: Self) -> Self { 
                self.wrapping_sub(rhs) 
            }

            #[inline]
            fn wrapping_mul(self, rhs: Self) -> Self { 
                self.wrapping_mul(rhs) 
            }

            #[inline]
            fn wrapping_div(self, rhs: Self) -> Self { 
                self.wrapping_div(rhs) 
            }

            #[inline]
            fn wrapping_rem(self, rhs: Self) -> Self { 
                self.wrapping_rem(rhs) 
            }

            #[inline]
            fn wrapping_pow(self, exp: u32) -> Self { 
                self.wrapping_pow(exp) 
            }

            #[inline]
            fn wrapping_neg(self) -> Self {
                self.wrapping_neg() 
            }

            #[inline]
            fn overflowing_add(self, rhs: Self) -> (Self, bool) { 
                self.overflowing_add(rhs) 
            }

            #[inline]
            fn overflowing_sub(self, rhs: Self) -> (Self, bool) { 
                self.overflowing_sub(rhs) 
            }

            #[inline]
            fn overflowing_mul(self, rhs: Self) -> (Self, bool) { 
                self.overflowing_mul(rhs) 
            }

            #[inline]
            fn overflowing_div(self, rhs: Self) -> (Self, bool) { 
                self.overflowing_div(rhs) 
            }

            #[inline]
            fn overflowing_neg(self) -> (Self, bool) { 
                self.overflowing_neg() 
            }

            #[inline]
            fn saturating_add(self, rhs: Self) -> Self { 
                self.saturating_add(rhs) 
            }

            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self { 
                self.saturating_sub(rhs) 
            }

            #[inline]
            fn saturating_mul(self, rhs: Self) -> Self { 
                self.saturating_mul(rhs) 
            }

            #[inline]
            fn saturating_div(self, rhs: Self) -> Self { 
                self.saturating_div(rhs) 
            }

            #[inline]
            fn saturating_pow(self, exp: u32) -> Self {
                self.saturating_pow(exp) 
            }

            #[inline]
            fn sqrt(self) -> Self {
                self.isqrt()
            }

            try_into! {
                u8
                i8
                u16
                i16
                u32
                i32
                u64
                i64
                u128
                i128
            }
        }
    };
    ($($size:ident $bit:literal $signed:expr)*) => {
        $(implementation! { $size $bit $signed })*
    };
}

pub trait Int 
where
    Self: Sized,
    Self: Clone,
    Self: Copy,
    Self: fmt::Debug,
    Self: fmt::Display,
    Self: ops::Add<Output=Self>,
    Self: ops::AddAssign,
    Self: ops::Sub<Output=Self>,
    Self: ops::SubAssign,
    Self: ops::Mul<Output=Self>,
    Self: ops::MulAssign,
    Self: ops::Div<Output=Self>,
    Self: ops::DivAssign,
    Self: ops::Shl<Output=Self>,
    Self: ops::ShlAssign,
    Self: ops::Shr<Output=Self>,
    Self: ops::ShrAssign,
    Self: ops::BitAnd<Output=Self>,
    Self: ops::BitAndAssign,
    Self: ops::BitOr<Output=Self>,
    Self: ops::BitOrAssign,
    Self: ops::Rem,
    Self: ops::RemAssign,
    Self: Eq,
    Self: PartialEq,
    Self: Ord,
    Self: PartialOrd {

    const IS_SIGNED: bool;
    const BIT: Self;
    const BIT_U128: u128;
    const BIT_I128: i128;
    const MAX: Self;
    const MAX_U128: u128;
    const MAX_I128: i128;
    const MIN: Self;
    const MIN_U128: u128;
    const MIN_I128: i128;

    as_const_sig! {
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
    }

    fn from_u128(value: u128) -> Result<Self>;
    fn from_i128(value: i128) -> Result<Self>;
    fn pow(self, exp: u32) -> Self;
    fn checked_add(self, rhs: Self) -> Result<Self>;
    fn checked_sub(self, rhs: Self) -> Result<Self>;
    fn checked_mul(self, rhs: Self) -> Result<Self>;
    fn checked_div(self, rhs: Self) -> Result<Self>;
    fn checked_rem(self, rhs: Self) -> Result<Self>;
    fn checked_pow(self, exp: u32) -> Result<Self>;
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn wrapping_rem(self, rhs: Self) -> Self;
    fn wrapping_pow(self, exp: u32) -> Self;
    fn wrapping_neg(self) -> Self;
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    fn overflowing_neg(self) -> (Self, bool);
    fn saturating_add(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    fn saturating_mul(self, rhs: Self) -> Self;
    fn saturating_div(self, rhs: Self) -> Self;
    fn saturating_pow(self, exp: u32) -> Self;
    fn sqrt(self) -> Self;

    #[inline]
    fn to_negative(self) -> Self {
        if self == Self::AS_0 {
            return self;
        }
        Self::AS_0 - self
    }

    #[inline]
    fn to_positive(self) -> Self {
        if self >= Self::AS_0 {
            return self
        }
        Self::AS_0 - self
    }

    #[inline]
    fn into_integer<T>(self) -> T 
        where 
            T: Int {
        if Self::IS_SIGNED {
            unsafe {
                let out: i128 = self.into_i128().unwrap_unchecked();
                let out: T = T::from_i128(out).unwrap_unchecked();
                return out
            }
        }
        unsafe {
            let out: u128 = self.into_u128().unwrap_unchecked();
            T::from_u128(out).unwrap_unchecked()
        }
    }

    try_into_sig! {
        u8
        i8
        u16
        i16
        u32
        i32
        u64
        i64
        u128
        i128
    }
}

implementation! {
    u8 8 false
    i8 8 true
    u16 16 false
    i16 16 true
    u32 32 false
    i32 32 true
    u64 64 false
    i64 64 true
    u128 128 false
    i128 128 true
}