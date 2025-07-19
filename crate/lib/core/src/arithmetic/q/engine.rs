use super::*;

pub type Q1<T> = Q<1, T>;
pub type Q2<T> = Q<2, T>;
pub type Q3<T> = Q<3, T>;
pub type Q4<T> = Q<4, T>;
pub type Q5<T> = Q<5, T>;
pub type Q6<T> = Q<6, T>;
pub type Q7<T> = Q<7, T>;
pub type Q8<T> = Q<8, T>;
pub type Q9<T> = Q<9, T>;
pub type Q10<T> = Q<10, T>;
pub type Q11<T> = Q<11, T>;
pub type Q12<T> = Q<12, T>;
pub type Q13<T> = Q<13, T>;
pub type Q14<T> = Q<14, T>;
pub type Q15<T> = Q<15, T>;
pub type Q16<T> = Q<16, T>;
pub type Q17<T> = Q<17, T>;
pub type Q18<T> = Q<18, T>;
pub type Q19<T> = Q<19, T>;
pub type Q20<T> = Q<20, T>;
pub type Q21<T> = Q<21, T>;
pub type Q22<T> = Q<22, T>;
pub type Q23<T> = Q<23, T>;
pub type Q24<T> = Q<24, T>;
pub type Q25<T> = Q<25, T>;
pub type Q26<T> = Q<26, T>;
pub type Q27<T> = Q<27, T>;


type Fixed<T> = T;
type Ratio<T> = T;

type Radian<T> = T;
type Degree<T> = T;


pub type Result<T> = core::result::Result<T, Error>;

#[repr(u8)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero,
    InvalidShift
}

pub struct Q<const A: u8, B, C = DefaultMode, D = DefaultEngine>
where
    B: num::Int,
    C: Mode,
    D: Engine {
    v: B,
    m_0: ::core::marker::PhantomData<C>,
    m_1: ::core::marker::PhantomData<D>
}

pub struct DefaultMode;
pub struct RadianMode;
pub struct DegreeMode;


pub struct DefaultEngine;

pub trait Engine 
where 
    Self: Sized {

    #[inline]
    fn add<T>(x: Fixed<T>, y: Fixed<T>) -> Result<Fixed<T>> 
    where
        T: num::Int {
        x.checked_add(y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(x: Fixed<T>, y: Fixed<T>) -> Result<Fixed<T>> 
    where
        T: num::Int {
        x.checked_sub(y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<const A: u8, B>(x: Fixed<B>, y: Fixed<B>) -> Result<Fixed<B>>
    where
        B: num::Int {
        Self::muldiv(x, y, Self::scale::<A, _>())
    }

    #[inline]
    fn div<const A: u8, B>(x: Fixed<B>, y: Fixed<B>) -> Result<Fixed<B>> 
    where
        B: num::Int {
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

    fn shl<A, B>(x: A, y: u32) -> Result<A> 
    where
        A: num::Int {
        x.checked_shl(y).ok_or(Error::InvalidShift)
    }

    fn shr<A, B>(x: A, y: u32) -> Result<A>
    where
        A: num::Int {
        x.checked_shr(y).ok_or(Error::InvalidShift)
    }


    #[inline]
    fn csc<const A: u8, B>(angle: Radian<B>) -> Result<Ratio<B>>
    where
        B: num::Int {
        Self::inv::<A, _>(Self::sin::<A, _>(angle)?)
    }

    #[inline]
    fn sec<const A: u8, B>(angle: Radian<B>) -> Result<Ratio<B>>
    where
        B: num::Int {
        Self::inv::<A, _>(Self::cos::<A, _>(angle)?)
    }

    #[inline]
    fn cot<const A: u8, B>(angle: Radian<B>) -> Result<Ratio<B>>
    where
        B: num::Int {
        Self::inv::<A, _>(Self::tan::<A, _>(angle)?)
    }

    #[inline]
    fn inv<const A: u8, B>(n: B) -> Result<B>
    where
        B: num::Int {
        Self::div(Self::scale::<A, _>(), n)
    }


    #[inline]
    fn tan<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Ratio<Fixed<B>>>
    where
        B: num::Int {
        Self::div::<A, _>(Self::sin::<A, _>(angle)?, Self::cos::<A, _>(angle)?)
    }

    #[inline]
    fn sin<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Ratio<Fixed<B>>>
    where
        B: num::Int {
        Self::cos::<A, _>(Self::sub(Self::to_radian::<A, _>(Self::deg90::<A, _>()?)?, angle)?)
    }

    #[inline]
    fn cos<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Ratio<Fixed<B>>>
    where
        B: num::Int {
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
            term = term.checked_div((B::AS_2 * key - B::AS_1) * (B::AS_2 * key)).ok_or(Error::DivisionByZero)?;
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
        B: num::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            n.try_into().unwrap_unchecked()
        };
        Self::muldiv(angle, Self::pi::<A, _>(), n * Self::scale::<A, _>())
    }

    #[inline]
    fn to_degree<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Degree<Fixed<B>>>
    where
        B: num::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            n.try_into().unwrap_unchecked()
        };
        Self::muldiv(angle, n * Self::scale::<A, _>(), Self::pi::<A, _>())
    }


    #[inline]
    fn to_negative<T>(n: T) -> T 
    where 
        T: num::Int {
        if n == T::AS_0 {
            T::AS_0
        } else {
            T::AS_0 - n
        }
    }

    #[inline]
    fn to_positive<T>(n: T) -> T 
    where 
        T: num::Int {
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
        T: num::Int {
        if z == T::AS_0 {
            return Err(Error::DivisionByZero)
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
                    Ok(Self::fold(&a, &b, &z)? / z)
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
        T: num::Int {
        if T::IS_SIGNED {
            Self::fold_signed(x, y, z)
        } else {
            Self::fold_unsigned(x, y, z)
        }
    }

    #[inline]
    fn fold_signed<T>(x: T, y: T, z: T) -> Result<T> 
    where
        T: num::Int {
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
        T: num::Int {
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
        T: num::Int {
        if T::IS_SIGNED {
            Self::wide_mul_signed(x, y)
        } else {
            Self::wide_mul_unsigned(x, y)
        }
    }

    #[inline]
    fn wide_mul_signed<T>(x: T, y: T) -> Result<(T, T)> 
    where
        T: num::Int {
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
        T: num::Int {
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
        B: num::Int, 
        (): Ok<A, B> {
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
        B: num::Int {
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
        B: num::Int {
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

pub trait Mode {}


impl Engine for DefaultEngine {}

impl Mode for DefaultMode {}
impl Mode for RadianMode {}
impl Mode for DegreeMode {}


// # global

impl<const A: u8, B, C, D> Default for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine {
    fn default() -> Self {
        Self::new(B::AS_0)
    }
}

impl<const A: u8, B, C, D> Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine {
    pub const fn new(v: B) -> Self {
        Self {
            v,
            m_0: ::core::marker::PhantomData,
            m_1: ::core::marker::PhantomData
        }
    }
}


impl<const A: u8, B, C, D> Eq for Q<A, B, C, D> {}
impl<const A: u8, B, C, D> PartialEq for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}


impl<const A: u8, B, C, D> ::core::ops::Add for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: B = self.v;
        let y: B = rhs.v;
        Ok(Self::new(D::add(x, y)?))
    }
}

impl<const A: u8, B, C, D> ::core::ops::Sub for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: B = self.v;
        let y: B = rhs.v;
        Ok(Self::new(D::sub(x, y)?))   
    }
}

impl<const A: u8, B, C, D> ::core::ops::Mul for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine {
    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let x: B = self.v;
        let y: B = rhs.v;
        Ok(Self::new(D::mul(x, y)?))
    }
}

impl<const A: u8, B, C, D> ::core::ops::Div for Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine {
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let x: B = self.v;
        let y: B = rhs.v;
        Ok(Self::new(D::div(x, y)?))
    }
}


// # default-mode

impl<const A: u8, B, C> From<Q<A, B, RadianMode, C>> for Q<A, B, DefaultMode, C>
where
    B: num::Int,
    C: Engine {
    fn from(value: Q<A, B, RadianMode, C>) -> Self {
        Self::new(value.v)
    }
}

impl<const A: u8, B, C> From<Q<A, B, DegreeMode, C>> for Q<A, B, DefaultMode, C>
where
    B: num::Int,
    C: Engine {
    fn from(value: Q<A, B, DegreeMode, C>) -> Self {
        Self::new(value.v)
    }
}