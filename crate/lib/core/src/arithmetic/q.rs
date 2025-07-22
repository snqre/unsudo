use super::*;

pub type Q0U8 = Q0<u8>;
pub type Q1U8 = Q1<u8>;
pub type Q2U8 = Q2<u8>;

pub type Q0U16 = Q0<u16>;
pub type Q1U16 = Q1<u16>;
pub type Q2U16 = Q2<u16>;
pub type Q3U16 = Q3<u16>;
pub type Q4U16 = Q4<u16>;

pub type Q0<T> = Q<0, T, DefaultMode>;
pub type Q1<T> = Q<1, T, DefaultMode>;
pub type Q2<T> = Q<2, T, DefaultMode>;
pub type Q3<T> = Q<3, T, DefaultMode>;
pub type Q4<T> = Q<4, T, DefaultMode>;
pub type Q5<T> = Q<5, T, DefaultMode>;
pub type Q6<T> = Q<6, T, DefaultMode>;
pub type Q7<T> = Q<7, T, DefaultMode>;
pub type Q8<T> = Q<8, T, DefaultMode>;
pub type Q9<T> = Q<9, T, DefaultMode>;
pub type Q10<T> = Q<10, T, DefaultMode>;
pub type Q11<T> = Q<11, T, DefaultMode>;
pub type Q12<T> = Q<12, T, DefaultMode>;
pub type Q13<T> = Q<13, T, DefaultMode>;
pub type Q14<T> = Q<14, T, DefaultMode>;
pub type Q15<T> = Q<15, T, DefaultMode>;
pub type Q16<T> = Q<16, T, DefaultMode>;
pub type Q17<T> = Q<17, T, DefaultMode>;
pub type Q18<T> = Q<18, T, DefaultMode>;
pub type Q19<T> = Q<19, T, DefaultMode>;
pub type Q20<T> = Q<20, T, DefaultMode>;
pub type Q21<T> = Q<21, T, DefaultMode>;
pub type Q22<T> = Q<22, T, DefaultMode>;
pub type Q23<T> = Q<23, T, DefaultMode>;
pub type Q24<T> = Q<24, T, DefaultMode>;
pub type Q25<T> = Q<25, T, DefaultMode>;
pub type Q26<T> = Q<26, T, DefaultMode>;
pub type Q27<T> = Q<27, T, DefaultMode>;
pub type Q28<T> = Q<28, T, DefaultMode>;
pub type Q29<T> = Q<29, T, DefaultMode>;
pub type Q30<T> = Q<30, T, DefaultMode>;
pub type Q31<T> = Q<31, T, DefaultMode>;
pub type Q32<T> = Q<32, T, DefaultMode>;
pub type Q33<T> = Q<33, T, DefaultMode>;
pub type Q34<T> = Q<34, T, DefaultMode>;
pub type Q35<T> = Q<35, T, DefaultMode>;
pub type Q36<T> = Q<36, T, DefaultMode>;
pub type Q37<T> = Q<37, T, DefaultMode>;
pub type Q38<T> = Q<38, T, DefaultMode>;

pub type Rad0<T> = Q<0, T, RadianMode>;
pub type Rad1<T> = Q<1, T, RadianMode>;
pub type Rad2<T> = Q<2, T, RadianMode>;
pub type Rad3<T> = Q<3, T, RadianMode>;
pub type Rad4<T> = Q<4, T, RadianMode>;
pub type Rad5<T> = Q<5, T, RadianMode>;
pub type Rad6<T> = Q<6, T, RadianMode>;
pub type Rad7<T> = Q<7, T, RadianMode>;
pub type Rad8<T> = Q<8, T, RadianMode>;
pub type Rad9<T> = Q<9, T, RadianMode>;
pub type Rad10<T> = Q<10, T, RadianMode>;
pub type Rad11<T> = Q<11, T, RadianMode>;
pub type Rad12<T> = Q<12, T, RadianMode>;
pub type Rad13<T> = Q<13, T, RadianMode>;
pub type Rad14<T> = Q<14, T, RadianMode>;
pub type Rad15<T> = Q<15, T, RadianMode>;
pub type Rad16<T> = Q<16, T, RadianMode>;
pub type Rad17<T> = Q<17, T, RadianMode>;

pub type Fixed<T> = T;
pub type Ratio<T> = T;

pub type Radian<T> = T;
pub type Degree<T> = T;

pub type Result<T> = core::result::Result<T, Error>;

#[repr(u8)]
pub enum Error {
    Overflow,
    Underflow,
    DivByZero,
    RemByZero,
    InvalidShift
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[repr(transparent)]
pub struct Q<const A: u8, B=u32, C=DefaultMode, D=DefaultEngine> 
where
    B: int::Int,
    C: Mode,
    D: Engine {
    n: B,
    ph_0: core::marker::PhantomData<C>,
    ph_1: core::marker::PhantomData<D>
}

#[derive(Debug)]
pub struct DefaultMode;

#[derive(Debug)]
pub struct RadianMode;

#[derive(Debug)]
pub struct DegreeMode;

#[derive(Debug)]
pub struct DefaultEngine;


//
// port.mode
//

pub trait Mode 
where
    Self: core::fmt::Debug {}

impl Mode for DefaultMode {}
impl Mode for RadianMode {}
impl Mode for DegreeMode {}


//
// port.engine
//

pub trait Engine 
where 
    Self: Sized {
    #[inline]
    fn add<T>(x: Fixed<T>, y: Fixed<T>) -> Result<Fixed<T>> 
    where
        T: int::Int {
        x.checked_add(y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(x: Fixed<T>, y: Fixed<T>) -> Result<Fixed<T>> 
    where
        T: int::Int {
        x.checked_sub(y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<const A: u8, B>(x: Fixed<B>, y: Fixed<B>) -> Result<Fixed<B>>
    where
        B: int::Int {
        Self::muldiv(x, y, Self::scale::<A, _>())
    }

    #[inline]
    fn div<const A: u8, B>(x: Fixed<B>, y: Fixed<B>) -> Result<Fixed<B>> 
    where
        B: int::Int {
        let scale: u128 = Self::scale::<A, _>();
        if scale.is_power_of_two() {
            return Ok(
                x << scale
                    .trailing_zeros()
                    .try_into()
                    .ok()
                    .expect("Unable to convert `u128` into `B`.")
            )
        }
        let scale: B = unsafe {
            scale.try_into().unwrap_unchecked()
        };
        Self::muldiv(x, scale, y)
    }

    #[inline]
    fn rem<T>(x: T, y: T) -> Result<T>
    where
        T: int::Int {
        if y == T::AS_0 {
            return Err(Error::RemByZero)
        }
        Ok(x % y)
    }

    fn shl<T>(x: T, y: T) -> Result<T> 
    where
        T: int::Int {
        let y: u32 = y
            .try_into()
            .ok()
            .ok_or(Error::InvalidShift)?;
        x.checked_shl(y).ok_or(Error::InvalidShift)
    }

    fn shr<T>(x: T, y: T) -> Result<T>
    where
        T: int::Int {
        let y: u32 = y
            .try_into()
            .ok()
            .ok_or(Error::InvalidShift)?;
        x.checked_shr(y).ok_or(Error::InvalidShift)
    }

    #[inline]
    fn bitand<T>(x: T, y: T) -> T
    where
        T: int::Int {
        x & y
    }

    #[inline]
    fn bitor<T>(x: T, y: T) -> T
    where
        T: int::Int {
        x | y
    }


    #[inline]
    fn csc<const A: u8, B>(angle: Radian<B>) -> Result<Ratio<B>>
    where
        B: int::Int {
        Self::inv::<A, _>(Self::sin::<A, _>(angle)?)
    }

    #[inline]
    fn sec<const A: u8, B>(angle: Radian<B>) -> Result<Ratio<B>>
    where
        B: int::Int {
        Self::inv::<A, _>(Self::cos::<A, _>(angle)?)
    }

    #[inline]
    fn cot<const A: u8, B>(angle: Radian<B>) -> Result<Ratio<B>>
    where
        B: int::Int {
        Self::inv::<A, _>(Self::tan::<A, _>(angle)?)
    }

    #[inline]
    fn inv<const A: u8, B>(n: B) -> Result<B>
    where
        B: int::Int {
        Self::div::<A, _>(Self::scale::<A, _>(), n)
    }

    #[inline]
    fn atan<const A: u8, B>(ratio: Ratio<B>) -> Result<Radian<B>>
    where
        B: int::Int {
        let mut pow: B = ratio;
        let mut sum: B = ratio;
        let mut sign: bool = false;
        for i in (3..=25).step_by(2) {
            pow = Self::muldiv(pow, ratio, Self::scale::<A, _>())?;
            pow = Self::muldiv(pow, ratio, Self::scale::<A, _>())?;
            let i: B = i.try_into().ok().unwrap();
            let term: B = pow.checked_div(i).ok_or(Error::DivByZero)?;
            sum = if sign {
                sum.checked_sub(term).ok_or(Error::Underflow)?
            } else {
                sum.checked_add(term).ok_or(Error::Overflow)?
            };
            sign = !sign;
        }
        Ok(sum)
    }

    #[inline]
    fn asin<const A: u8, B>(ratio: Ratio<B>) -> Result<Radian<B>>
    where
        B: int::Int {
        if ratio == B::AS_0 {
            return Ok(B::AS_0)
        }
        if ratio == Self::scale::<A, _>() {
            return Self::div::<A, _>(Self::pi::<A, _>(), B::AS_2)
        }
        let sq: B = Self::muldiv(ratio, ratio, Self::scale::<A, _>())?;
        let mut ret: B = ratio;
        let mut pow: B = ratio;
        let coef: [(u16, u16); 8] = [
            (1, 1),
            (1, 6),
            (3, 40),
            (5, 112),
            (35, 1152),
            (63, 2816),
            (231, 13312),
            (429, 30720)
        ];
        for &(a, b) in &coef {
            let (a, b) = {
                let a: B = a.try_into().ok().unwrap();
                let b: B = b.try_into().ok().unwrap();
                (a, b)
            };
            pow = Self::muldiv(pow, sq, Self::scale::<A, _>())?;
            let term: B = Self::muldiv(pow, a, Self::scale::<A, _>())?;
            let term: B = Self::div::<A, _>(term, b)?;
            ret = Self::add(ret, term)?;
        }
        Ok(ret)
    }

    #[inline]
    fn acos<const A: u8, B>(ratio: Ratio<B>) -> Result<Radian<B>> 
    where
        B: int::Int {
        let scale: B = Self::scale::<A, _>();
        let pi: B = Self::pi::<A, _>();
        let pi_2: B = pi / B::AS_2;
        if ratio == scale {
            return Ok(B::AS_0)
        }
        if B::IS_SIGNED && ratio == Self::to_negative(scale) {
            return Ok(pi)
        }
        Self::sub(pi_2, Self::asin::<A, _>(ratio)?)
    }

    #[inline]
    fn tan<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Ratio<Fixed<B>>>
    where
        B: int::Int {
        Self::div::<A, _>(Self::sin::<A, _>(angle)?, Self::cos::<A, _>(angle)?)
    }

    #[inline]
    fn sin<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Ratio<Fixed<B>>>
    where
        B: int::Int {
        Self::cos::<A, _>(Self::sub(Self::to_radian::<A, _>(Self::deg90::<A, _>()?)?, angle)?)
    }

    #[inline]
    fn cos<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Ratio<Fixed<B>>>
    where
        B: int::Int {
        let scale: B = Self::scale::<A, _>();
        let pi: B = Self::pi::<A, _>();
        let pi_2: B = pi.checked_mul(B::AS_2).ok_or(Error::Overflow)?;
        let mut n: B = angle % pi_2;
        if n < B::AS_0 {
            n = n.checked_add(pi_2).ok_or(Error::Overflow)?;
        }
        if n > pi {
            n = n.checked_sub(pi_2).ok_or(Error::Underflow)?;
        }
        let mut term: B = scale;
        let mut ret: B = scale;
        let mut sign: bool = true;
        let mut key: B = B::AS_1;
        loop {
            term = Self::muldiv(term, n, scale)?;
            term = Self::muldiv(term, n, scale)?;
            term = term.checked_div((B::AS_2 * key - B::AS_1) * (B::AS_2 * key)).ok_or(Error::DivByZero)?;
            if term == B::AS_0 {
                break
            }
            ret = if sign {
                ret.checked_sub(term).ok_or(Error::Underflow)?
            } else {
                ret.checked_add(term).ok_or(Error::Overflow)?
            };
            sign = !sign;
            key = key.checked_add(B::AS_1).ok_or(Error::Overflow)?;
        }
        Ok(ret)
    }


    #[inline]
    fn to_radian<const A: u8, B>(angle: Degree<Fixed<B>>) -> Result<Radian<Fixed<B>>>
    where
        B: int::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            n.try_into().unwrap_unchecked()
        };
        Self::muldiv(angle, Self::pi::<A, _>(), n * Self::scale::<A, _>())
    }

    #[inline]
    fn to_degree<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Degree<Fixed<B>>>
    where
        B: int::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            n.try_into().unwrap_unchecked()
        };
        Self::muldiv(angle, n * Self::scale::<A, _>(), Self::pi::<A, _>())
    }

    #[inline]
    fn to_negative<T>(n: T) -> T 
    where 
        T: int::Int {
        if n == T::AS_0 {
            T::AS_0
        } else {
            T::AS_0 - n
        }
    }

    #[inline]
    fn to_positive<T>(n: T) -> T 
    where 
        T: int::Int {
        if n == T::AS_0 {
            T::AS_0
        } else if n > T::AS_0 {
            n
        } else {
            T::AS_0 - n
        }
    }

    #[inline]
    fn muldiv<T>(x: T, y: T, z: T) -> Result<T> 
    where
        T: int::Int {
        if z == T::AS_0 {
            return Err(Error::DivByZero)
        }
        match (T::BITS_U128, T::IS_SIGNED) {
            (_, true) if T::BITS_U128 <= 64 => {
                let ret: T = x.checked_mul(y).ok_or(Error::Overflow)?;
                let ret: T = ret / z;
                Ok(ret)
            },
            (_, false) if T::BITS_U128 < 128 => {
                let (a, b) = Self::wide_mul(x, y)?;
                if b < z {
                    return Err(Error::Overflow)
                }
                if b == T::AS_0 {
                    Ok(a / z)
                } else {
                    Ok(Self::fold(a, b, z)? / z)
                }
            },
            (128, _) => {
                let ret: T = x.checked_mul(y).ok_or(Error::Overflow)?;
                Ok(ret / z)
            },
            _ => unsafe {
                ::core::hint::unreachable_unchecked();
            }
        }
    }

    #[inline]
    fn fold<T>(x: T, y: T, z: T) -> Result<T> 
    where 
        T: int::Int {
        if T::IS_SIGNED {
            Self::fold_signed(x, y, z)
        } else {
            Self::fold_unsigned(x, y, z)
        }
    }

    #[inline]
    fn fold_signed<T>(x: T, y: T, z: T) -> Result<T> 
    where
        T: int::Int {
        let (x, y, z) = unsafe {
            let x: i128 = x.try_into().unwrap_unchecked();
            let y: i128 = y.try_into().unwrap_unchecked();
            let z: i128 = z.try_into().unwrap_unchecked();
            (x, y, z)
        };
        let ret: i128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
        if ret > T::MAX_I128 {
            return Err(Error::Overflow)
        }
        if ret < T::MIN_I128 {
            return Err(Error::Underflow)
        }  
        let ret: T = unsafe {
            ret.try_into().unwrap_unchecked()
        };
        Ok(ret)
    }

    #[inline]
    fn fold_unsigned<T>(x: T, y: T, z: T) -> Result<T> 
    where
        T: int::Int {
        let (x, y, z) = unsafe {
            let x: u128 = x.try_into().unwrap_unchecked();
            let y: u128 = y.try_into().unwrap_unchecked();
            let z: u128 = z.try_into().unwrap_unchecked();
            (x, y, z)
        };
        let ret: u128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
        if ret > T::MAX_U128 {
            return Err(Error::Overflow)
        }
        if ret < T::MIN_U128 {
            return Err(Error::Underflow)
        }  
        let out: T = unsafe {
            ret.try_into().unwrap_unchecked()
        };
        Ok(out)
    }

    #[inline]
    fn wide_mul<T>(x: T, y: T) -> Result<(T, T)> 
    where
        T: int::Int {
        if T::IS_SIGNED {
            Self::wide_mul_signed(x, y)
        } else {
            Self::wide_mul_unsigned(x, y)
        }
    }

    #[inline]
    fn wide_mul_signed<T>(x: T, y: T) -> Result<(T, T)> 
    where
        T: int::Int {
        if !T::IS_SIGNED || T::BITS_U128 > 64 {
            panic!()
        }
        let n: T = T::BITS / T::AS_2;
        let mask: T = (T::AS_1 << n) - T::AS_1;
        let (lo_lo, lo_hi, hi_lo, hi_hi) = {
            let x_lo: T = x & mask;
            let x_hi: T = x >> n;
            let y_lo: T = y & mask;
            let y_hi: T = y >> n;
            let lo_lo: T = x_lo.checked_mul(y_lo).ok_or(Error::Overflow)?;
            let lo_hi: T = x_lo.checked_mul(y_hi).ok_or(Error::Overflow)?;
            let hi_lo: T = x_hi.checked_mul(y_lo).ok_or(Error::Overflow)?;
            let hi_hi: T = x_hi.checked_mul(y_hi).ok_or(Error::Overflow)?;
            (lo_lo, lo_hi, hi_lo, hi_hi)
        };
        let a: T = lo_hi.checked_add(hi_lo).ok_or(Error::Overflow)?;
        let b: T = a << n;
        let hi: T = if lo_lo > lo_lo.wrapping_add(b) {
            T::AS_1
        } else {
            T::AS_0
        };
        let hi: T = hi_hi
            .checked_add(a >> n)
            .ok_or(Error::Overflow)?
            .checked_add(hi)
            .ok_or(Error::Overflow)?;
        let lo = lo_lo.wrapping_add(b);
        Ok((lo, hi))
    }

    #[inline]
    fn wide_mul_unsigned<T>(x: T, y: T) -> Result<(T, T)> 
    where
        T: int::Int {
        if T::IS_SIGNED || T::BITS_U128 > 64 {
            panic!()
        }
        let (x, y) = unsafe {
            let x: u128 = x.try_into().unwrap_unchecked();
            let y: u128 = y.try_into().unwrap_unchecked();
            (x, y)
        };
        let (a, b) = {
            let x_hi: u128 = x >> 64;
            let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
            let y_hi: u128 = y >> 64;
            let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
            let lo_lo: u128 = x_lo * y_lo;
            let lo_hi: u128 = x_lo * y_hi;
            let hi_lo: u128 = x_hi * y_lo;
            let hi_hi: u128 = x_hi * y_hi;
            let m: u128 = lo_hi + hi_lo;
            let c: u128 = ((m < lo_hi) as u128) << 64;
            let m_lo: u128 = m << 64;
            let m_hi: u128 = m >> 64;
            let a: u128 = lo_lo.wrapping_add(m_lo);
            let b: u128 = hi_hi + m_hi + c + ((a < lo_lo) as u128);
            (a, b)
        };
        if T::BITS_U128 == 128 {
            unsafe {
                let a: T = a.try_into().unwrap_unchecked();
                let b: T = b.try_into().unwrap_unchecked();
                return Ok((a, b))
            }
        }
        if a > T::MAX_U128 {
            return Err(Error::Overflow)
        }
        if a < T::MIN_U128 {
            return Err(Error::Underflow)
        }
        if b > T::MAX_U128 {
            return Err(Error::Overflow)
        }
        if b < T::MIN_U128 {
            return Err(Error::Underflow)
        }
        let (a, b) = unsafe {
            let a: T = a.try_into().unwrap_unchecked();
            let b: T = b.try_into().unwrap_unchecked();
            (a, b)
        };
        Ok((a, b))
    }

    #[inline]
    fn deg90<const A: u8, B>() -> Result<Degree<Fixed<B>>> 
    where 
        B: int::Int {
        if B::IS_SIGNED {
            let n: i128 = 90;
            let n: B = unsafe {
                n.try_into().unwrap_unchecked()
            };
            n
        } else {
            let n: u128 = 90;
            let n: B = unsafe {
                n.try_into().unwrap_unchecked()
            };
            n
        }.checked_mul(Self::scale::<A, _>()).ok_or(Error::Overflow)
    }

    #[inline]
    fn pi<const A: u8, B>() -> B 
    where 
        B: int::Int {
        if B::IS_SIGNED {
            let pi: i128 = Self::signed_pi::<A>();
            let pi: B = unsafe {
                pi.try_into().unwrap_unchecked()
            };
            return pi
        }
        let pi: u128 = Self::unsigned_pi::<A>();
        let pi: B = unsafe {
            pi.try_into().unwrap_unchecked()
        };
        pi
    }

    #[inline]
    fn signed_pi<const T: u8>() -> i128 {
        assert!(T != 0);
        assert!(T <= 37);
        match T {
            1 => 31,
            2 => 314,
            3 => 3141,
            4 => 31415,
            5 => 314159,
            6 => 3141592,
            7 => 31415926,
            8 => 314159265,
            9 => 3141592653,
            10 => 31415926535,
            11 => 314159265358,
            12 => 3141592653589,
            13 => 31415926535897,
            14 => 314159265358979,
            15 => 3141592653589793,
            16 => 31415926535897932,
            17 => 314159265358979323,
            18 => 3141592653589793238,
            19 => 31415926535897932384,
            20 => 314159265358979323846,
            21 => 3141592653589793238462,
            22 => 31415926535897932384626,
            23 => 314159265358979323846264,
            24 => 3141592653589793238462643,
            25 => 31415926535897932384626433,
            26 => 314159265358979323846264338,
            27 => 3141592653589793238462643383,
            28 => 31415926535897932384626433832,
            29 => 314159265358979323846264338327,
            30 => 3141592653589793238462643383279,
            31 => 31415926535897932384626433832795,
            32 => 314159265358979323846264338327950,
            33 => 3141592653589793238462643383279502,
            34 => 31415926535897932384626433832795028,
            35 => 314159265358979323846264338327950288,
            36 => 3141592653589793238462643383279502884,
            37 => 31415926535897932384626433832795028841,
            _ => panic!()
        }
    }

    #[inline]
    fn unsigned_pi<const T: u8>() -> u128 {
        match T {
            1 => 31,
            2 => 314,
            3 => 3141,
            4 => 31415,
            5 => 314159,
            6 => 3141592,
            7 => 31415926,
            8 => 314159265,
            9 => 3141592653,
            10 => 31415926535,
            11 => 314159265358,
            12 => 3141592653589,
            13 => 31415926535897,
            14 => 314159265358979,
            15 => 3141592653589793,
            16 => 31415926535897932,
            17 => 314159265358979323,
            18 => 3141592653589793238,
            19 => 31415926535897932384,
            20 => 314159265358979323846,
            21 => 3141592653589793238462,
            22 => 31415926535897932384626,
            23 => 314159265358979323846264,
            24 => 3141592653589793238462643,
            25 => 31415926535897932384626433,
            26 => 314159265358979323846264338,
            27 => 3141592653589793238462643383,
            28 => 31415926535897932384626433832,
            29 => 314159265358979323846264338327,
            30 => 3141592653589793238462643383279,
            31 => 31415926535897932384626433832795,
            32 => 314159265358979323846264338327950,
            33 => 3141592653589793238462643383279502,
            34 => 31415926535897932384626433832795028,
            35 => 314159265358979323846264338327950288,
            36 => 3141592653589793238462643383279502884,
            37 => 31415926535897932384626433832795028841,
            38 => 314159265358979323846264338327950288419,
            _ => unsafe {
                ::core::hint::unreachable_unchecked()
            }
        }
    }


    #[inline]
    fn scale<const A: u8, B>() -> B 
    where
        B: int::Int {
        if B::IS_SIGNED {
            let scale: i128 = Self::signed_scale::<A>();
            let scale: B = unsafe {
                scale.try_into().unwrap_unchecked()
            };
            return scale
        }
        let scale: u128 = Self::unsigned_scale::<A>();
        let scale: B = unsafe {
            scale.try_into().unwrap_unchecked()
        };
        scale
    }

    #[inline]
    fn signed_scale<const T: u8>() -> i128 {
        assert!(T >= 1);
        assert!(T <= 38);
        const BASE: i128 = 10;
        match T {
            1 => BASE.pow(1),
            2 => BASE.pow(2),
            3 => BASE.pow(3),
            4 => BASE.pow(4),
            5 => BASE.pow(5),
            6 => BASE.pow(6),
            7 => BASE.pow(7),
            8 => BASE.pow(8),
            9 => BASE.pow(9),
            10 => BASE.pow(10),
            11 => BASE.pow(11),
            12 => BASE.pow(12),
            13 => BASE.pow(13),
            14 => BASE.pow(14),
            15 => BASE.pow(15),
            16 => BASE.pow(16),
            17 => BASE.pow(17),
            18 => BASE.pow(18),
            19 => BASE.pow(19),
            20 => BASE.pow(20),
            21 => BASE.pow(21),
            22 => BASE.pow(22),
            23 => BASE.pow(23),
            24 => BASE.pow(24),
            25 => BASE.pow(25),
            26 => BASE.pow(26),
            27 => BASE.pow(27),
            28 => BASE.pow(28),
            29 => BASE.pow(29),
            30 => BASE.pow(30),
            31 => BASE.pow(31),
            32 => BASE.pow(32),
            33 => BASE.pow(33),
            34 => BASE.pow(34),
            35 => BASE.pow(35),
            36 => BASE.pow(36),
            37 => BASE.pow(37),
            38 => BASE.pow(38),
            _ => unreachable!()
        }
    }

    #[inline]
    fn unsigned_scale<const T: u8>() -> u128 {
        assert!(T >= 1);
        assert!(T <= 38);
        const BASE: u128 = 10;
        match T {
            1 => BASE.pow(1),
            2 => BASE.pow(2),
            3 => BASE.pow(3),
            4 => BASE.pow(4),
            5 => BASE.pow(5),
            6 => BASE.pow(6),
            7 => BASE.pow(7),
            8 => BASE.pow(8),
            9 => BASE.pow(9),
            10 => BASE.pow(10),
            11 => BASE.pow(11),
            12 => BASE.pow(12),
            13 => BASE.pow(13),
            14 => BASE.pow(14),
            15 => BASE.pow(15),
            16 => BASE.pow(16),
            17 => BASE.pow(17),
            18 => BASE.pow(18),
            19 => BASE.pow(19),
            20 => BASE.pow(20),
            21 => BASE.pow(21),
            22 => BASE.pow(22),
            23 => BASE.pow(23),
            24 => BASE.pow(24),
            25 => BASE.pow(25),
            26 => BASE.pow(26),
            27 => BASE.pow(27),
            28 => BASE.pow(28),
            29 => BASE.pow(29),
            30 => BASE.pow(30),
            31 => BASE.pow(31),
            32 => BASE.pow(32),
            33 => BASE.pow(33),
            34 => BASE.pow(34),
            35 => BASE.pow(35),
            36 => BASE.pow(36),
            37 => BASE.pow(37),
            38 => BASE.pow(38),
            _ => unreachable!()
        }
    }
}

impl Engine for DefaultEngine {}

mod any_mode {
    use super::*;

    impl<const A: u8, B, C, D> Q<A, B, C, D> 
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        #[inline]
        pub const fn new(n: B) -> Self {
            Self {
                n,
                ph_0: core::marker::PhantomData,
                ph_1: core::marker::PhantomData
            }
        }

        #[inline]
        pub fn to_negative(self) -> Self {
            Self::new(D::to_negative(self.n))
        }

        #[inline]
        pub fn to_positive(self) -> Self {
            Self::new(D::to_positive(self.n))
        }
    }

    impl<const A: u8, B, C, D> From<B> for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        #[inline]
        fn from(value: B) -> Self {
            Self::new(value)
        }
    }

    impl<const A: u8, B, C, D> Default for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        #[inline]
        fn default() -> Self {
            Self {
                n: B::AS_0,
                ph_0: core::marker::PhantomData,
                ph_1: core::marker::PhantomData
            }
        }
    }

    impl<const A: u8, B, C, D> core::ops::Add for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        type Output = Result<Self>;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            let x: B = self.n;
            let y: B = rhs.n;
            Ok(Self::new(D::add(x, y)?))
        }
    }

    impl<const A: u8, B, C, D> core::ops::Sub for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        type Output = Result<Self>;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            let x: B = self.n;
            let y: B = rhs.n;
            Ok(Self::new(D::sub(x, y)?))
        }
    }

    impl<const A: u8, B, C, D> core::ops::Mul for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        type Output = Result<Self>;

        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            let x: B = self.n;
            let y: B = rhs.n;
            Ok(Self::new(D::mul::<A, _>(x, y)?))
        }
    }

    impl<const A: u8, B, C, D> core::ops::Div for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        type Output = Result<Self>;

        #[inline]
        fn div(self, rhs: Self) -> Self::Output {
            let x: B = self.n;
            let y: B = rhs.n;
            Ok(Self::new(D::div::<A, _>(x, y)?))   
        }
    }

    impl<const A: u8, B, C, D> core::ops::Rem for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        type Output = Result<Self>;

        #[inline]
        fn rem(self, rhs: Self) -> Self::Output {
            let x = self.n;
            let y = rhs.n;
            Ok(Self::new(D::rem(x, y)?))
        }
    }

    impl<const A: u8, B, C, D> core::ops::Shl for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        type Output = Result<Self>;

        #[inline]
        fn shl(self, rhs: Self) -> Self::Output {
            let x: B = self.n;
            let y: B = rhs.n;
            Ok(Self::new(D::shl(x, y)?))
        }
    }

    impl<const A: u8, B, C, D> core::ops::Shr for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        type Output = Result<Self>;

        #[inline]
        fn shr(self, rhs: Self) -> Self::Output {
            let x: B = self.n;
            let y: B = rhs.n;
            Ok(Self::new(D::shr(x, y)?))
        }
    }

    impl<const A: u8, B, C, D> core::ops::BitAnd for Q<A, B, C, D> 
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        type Output = Self;
        
        #[inline]
        fn bitand(self, rhs: Self) -> Self::Output {
            let x: B = self.n;
            let y: B = rhs.n;
            Self::new(D::bitand(x, y))
        }
    }

    impl<const A: u8, B, C, D> core::ops::BitOr for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        type Output = Self;

        #[inline]
        fn bitor(self, rhs: Self) -> Self::Output {
            let x: B = self.n;
            let y: B = rhs.n;
            Self::new(D::bitor(x, y))
        }
    }

    impl<const A: u8, B, C, D> PartialEq for Q<A, B, C, D>
    where
        B: int::Int,
        C: Mode,
        D: Engine {
        fn eq(&self, other: &Self) -> bool {
            self.n == other.n
        }
    }
}

mod default_mode {
    use super::*;

    impl<const A: u8, B, C> From<Q<A, B, RadianMode, C>> for Q<A, B, DefaultMode, C>
    where
        B: int::Int,
        C: Engine {
        fn from(value: Q<A, B, RadianMode, C>) -> Self {
            Self::new(value.n)
        }
    }

    impl<const A: u8, B, C> From<Q<A, B, DegreeMode, C>> for Q<A, B, DefaultMode, C>
    where
        B: int::Int,
        C: Engine {
        fn from(value: Q<A, B, DegreeMode, C>) -> Self {
            Self::new(value.n)
        }
    }
}

mod radian_mode {
    use super::*;

    impl<const A: u8, B, C> TryFrom<Q<A, B, DegreeMode, C>> for Q<A, B, RadianMode, C>
    where
        B: int::Int,
        C: Engine {
        type Error = Error;

        fn try_from(value: Q<A, B, DegreeMode, C>) -> core::result::Result<Self, Self::Error> {
            let ret: Self = C::to_radian::<A, _>(value.n)?.into();
            Ok(ret)
        }
    }

    impl<const A: u8, B, C> Q<A, B, RadianMode, C> 
    where
        B: int::Int,
        C: Engine {
        #[inline]
        pub fn sec(self) -> Result<Self> {
            let ret: Self = C::sec::<A, _>(self.n)?.into();
            Ok(ret)
        }

        #[inline]
        pub fn csc(self) -> Result<Self> {
            let ret: Self = C::csc::<A, _>(self.n)?.into();
            Ok(ret)
        }

        #[inline]
        pub fn cot(self) -> Result<Self> {
            let ret: Self = C::cot::<A, _>(self.n)?.into();
            Ok(ret)
        }

        #[inline]
        pub fn atan(self) -> Result<Self> {
            let ret: Self = C::atan::<A, _>(self.n)?.into();
            Ok(ret)
        }

        #[inline]
        pub fn asin(self) -> Result<Self> {
            let ret: Self = C::asin::<A, _>(self.n)?.into();
            Ok(ret)
        }

        #[inline]
        pub fn acos(self) -> Result<Self> {
            let ret: Self = C::acos::<A, _>(self.n)?.into();
            Ok(ret)
        }
        
        #[inline]
        pub fn tan(self) -> Result<Self> {
            let ret: Self = C::tan::<A, _>(self.n)?.into();
            Ok(ret)
        }

        #[inline]
        pub fn sin(self) -> Result<Self> {
            let ret: Self = C::sin::<A, _>(self.n)?.into();
            Ok(ret)
        }

        #[inline]
        pub fn cos(self) -> Result<Self> {
            let ret: Self = C::cos::<A, _>(self.n)?.into();
            Ok(ret)
        }
    }
}

mod degree_mode {
    use super::*;

    impl<const A: u8, B, C> TryFrom<Q<A, B, RadianMode, C>> for Q<A, B, DegreeMode, C>
    where
        B: int::Int,
        C: Engine {
        type Error = Error;

        fn try_from(value: Q<A, B, RadianMode, C>) -> core::result::Result<Self, Self::Error> {
            let ret: Self = C::to_degree::<A, _>(value.n)?.into();
            Ok(ret)
        }
    }
}

#[allow(clippy::zero_prefixed_literal)]
#[cfg(test)]
mod test {
    use super::*;

    #[rstest::rstest]
    #[case(5_00.into(), 7_50.into(), 12_50.into())]
    #[case(0_50.into(), 7_50.into(), 8_00.into())]
    #[case(5_00.into(), 0_50.into(), 5_50.into())]
    #[case(0_01.into(), 0_01.into(), 0_02.into())]
    #[case((u128::MAX - 0_01).into(), 0_01.into(), u128::MAX.into())]
    #[case(0_00.into(), 0_01.into(), 0_01.into())]
    fn add(#[case] x: Q2<u128>, #[case] y: Q2<u128>, #[case] cor: Q2<u128>) {
        let ret: Q2<u128> = (x + y).ok().unwrap();
        assert_eq!(ret, cor);
    }

    #[rstest::rstest]
    #[case(7_50.into(), 5_00.into(), 2_50.into())]
    fn sub(#[case] x: Q2<u128>, #[case] y: Q2<u128>, #[case] cor: Q2<u128>) {
        let ret: Q2<u128> = (x - y).ok().unwrap();
        assert_eq!(ret, cor);
    }

    #[rstest::rstest]
    #[case(7_50.into(), 5_00.into(), 37_50.into())]
    #[case(7_50.into(), 0_50.into(), 3_75.into())]
    #[case(0_50.into(), 7_50.into(), 3_75.into())]
    fn mul(#[case] x: Q2<u128>, #[case] y: Q2<u128>, #[case] cor: Q2<u128>) {
        let ret: Q2<u128> = (x * y).ok().unwrap();
        assert_eq!(ret, cor);
    }

    #[rstest::rstest]
    #[case(7_50.into(), 5_00.into(), 1_50.into())]
    #[case(7_50.into(), 0_50.into(), 15_00.into())]
    #[case(0_50.into(), 7_50.into(), 0_06.into())]
    fn div(#[case] x: Q2<u128>, #[case] y: Q2<u128>, #[case] cor: Q2<u128>) {
        let ret: Q2<u128> = (x / y).ok().unwrap();
        assert_eq!(ret, cor);
    }

    #[rstest::rstest]
    #[case(7_50.into(), 2_00.into(), 1_50.into())]
    fn rem(#[case] x: Q2<u128>, #[case] y: Q2<u128>, #[case] cor: Q2<u128>) {
        let ret: Q2<u128> = (x % y).ok().unwrap();
        assert_eq!(ret, cor);
    }
}