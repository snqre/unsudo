use super::*;

pub struct DefaultEngine;

impl Engine for DefaultEngine {}

pub trait Engine 
where
    Self: Sized {
    #[inline]
    fn add<T>(x: &T, y: &T) -> Result<T>
    where
        T: int::Int {
        x.checked_add(*y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(x: &T, y: &T) -> Result<T>
    where
        T: int::Int {
        x.checked_sub(*y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<const A: u8, B>(x: &B, y: &B) -> Result<B>
    where
        B: int::Int {
        Self::muldiv(x, y, scale::into::<A, _>())
    }

    #[inline]
    fn div<const A: u8, B>(x: &B, y: &B) -> Result<B>
    where
        B: int::Int {
        let scale: u128 = scale::into::<A, _>();
        if scale.is_power_of_two() {
            let out: B = x << scale.trailing_zeros().try_into().unwrap();
            return Ok(out)
        }
        let scale: B = unsafe {
            scale.try_into().unwrap_unchecked()
        };
        Self::muldiv(x, &scale, y)
    }

    fn shl<T>(x: &T, y: &u32) -> Result<T> 
    where
        T: int::Int {
        let x: T = *x;
        let y: u32 = *y;
        x.checked_shl(y).ok_or(Error::InvalidShift)
    }

    fn shr<T>(x: T, y: u32) -> Result<T>
    where
        T: int::Int {
        x.checked_shr(y).ok_or(Error::InvalidShift)
    }

    #[inline]
    fn to_negative<T>(n: &T) -> T 
    where 
        T: int::Int {
        if n == T::AS_0 {
            return T::AS_0;
        }
        T::AS_0 - *n
    }

    #[inline]
    fn to_positive<T>(n: &T) -> T 
    where 
        T: int::Int {
        if n == T::AS_0 {
            return T::AS_0;
        }
        if n > T::AS_0 {
            return n;
        }
        T::AS_0 - n
    }

    #[inline]
    fn muldiv<T>(x: &T, y: &T, z: &T) -> Result<T> 
    where
        T: int::Int {
        let x: T = *x;
        let y: T = *y;
        let z: T = *z;
        if z == T::AS_0 {
            return Err(Error::DivisionByZero);
        }
        match (T::BITS, T::IS_SIGNED) {
            (_, true) if T::BITS <= 64 => {
                let out: T = x.checked_mul(y).ok_or(Error::Overflow)?;
                let out: T = out / z;
                Ok(out)
            },
            (_, false) if T::BITS < 128 => {
                let (a, b) = wide_mul(x, y)?;
                if b >= z {
                    return Err(Error::Overflow)
                }
                if b == T::AS_0 {
                    return Ok(a / z)
                }
                Ok(fold(a, b, z)? / z)
            },
            (128, _) => {
                let out: T = x.checked_mul(y).ok_or(Error::Overflow)?;
                Ok(out / z)
            },
            _ => unsafe {
                ::core::hint::unreachable_unchecked();
            }
        }
    }
}

#[inline]
fn fold<T>(x: &T, y: &T, z: &T) -> Result<T> 
where 
    T: int::Int {
    if T::IS_SIGNED {
        return fold_signed(x, y, z);
    }
    fold_unsigned(x, y, z)
}

#[inline]
fn fold_signed<T>(x: &T, y: &T, z: &T) -> Result<T> 
where
    T: int::Int {
    let (x, y, z) = unsafe {
        let x: i128 = x.try_into().unwrap_unchecked();
        let y: i128 = y.try_into().unwrap_unchecked();
        let z: i128 = z.try_into().unwrap_unchecked();
        (x, y, z)
    };
    let out: i128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if out > T::MAX_I128 {
        return Err(Error::Overflow);
    }
    if out < T::MIN_I128 {
        return Err(Error::Underflow);
    }
    let out: T = unsafe {
        out.try_into().unwrap_unchecked()
    };
    Ok(out)
}

#[inline]
fn fold_unsigned<T>(x: &T, y: &T, z: &T) -> Result<T> 
where
    T: int::Int {
    let (x, y, z) = unsafe {
        let x: u128 = x.try_into().unwrap_unchecked();
        let y: u128 = y.try_into().unwrap_unchecked();
        let z: u128 = z.try_into().unwrap_unchecked();
        (x, y, z)
    };
    let out: u128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if out > T::MAX_U128 {
        return Err(Error::Overflow);
    }
    if out < T::MIN_U128 {
        return Err(Error::Underflow);
    }
    let out: T = unsafe {
        out.try_into().unwrap_unchecked()
    };
    Ok(out)
}

#[inline]
fn wide_mul<T>(x: T, y: T) -> Result<(T, T)> 
where
    T: int::Int {
    if T::IS_SIGNED {
        return wide_mul_signed(x, y)
    }
    wide_mul_unsigned(x, y)
}

#[inline]
fn wide_mul_signed<T>(x: T, y: T) -> Result<(T, T)> 
where
    T: int::Int {
    assert!(T::IS_SIGNED);
    assert!(T::BITS <= 64);
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
    let lo: T = lo_lo.wrapping_add(b);
    Ok((lo, hi))
}

#[inline]
fn wide_mul_unsigned<T>(x: T, y: T) -> Result<(T, T)> 
where
    T: int::Int {
    assert!(!T::IS_SIGNED);
    assert!(T::BITS <= 64);
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
    if T::BITS == 128 {
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

pub struct DefaultMode;
pub struct RadianMode;
pub struct DegreeMode;
pub trait Mode {}
impl Mode for DefaultMode {}
impl Mode for RadianMode {}
impl Mode for DegreeMode {}

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero,
    InvalidShift
}

pub struct Q<const A: u8, B = u128, C = DefaultMode, D = DefaultEngine> 
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {
    v: B,
    m_0: core::marker::PhantomData<C>,
    m_1: core::marker::PhantomData<D>
}

impl<const A: u8, B, C, D> core::fmt::Display for Q<A, B, C, D>
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v: &B = &self.v;
        let v: &str = v.into();
        let c: [char; 3];
        for c in v.chars() {
            
        }
        Ok(())
    }
}

impl<'a, const A: u8, B, C, D> Q<'a, A, B, C, D>
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {
    pub fn new(v: &B) -> Self {
        Self {
            v,
            m_0: core::marker::PhantomData,
            m_1: core::marker::PhantomData
        }
    }

    pub fn to_negative(self) -> Self {
        Q::new(D::to_negative(&self.v))
    }

    pub fn to_positive(self) -> Self {
        Q::new(D::to_positive(&self.v))
    }
}

impl<'a, const A: u8, B, C, D> Eq for Q<'a, A, B, C, D> 
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {}

    impl<'a, const A: u8, B, C, D> PartialEq for Q<'a, A, B, C, D>
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl<'a, const A: u8, B, C, D> core::ops::Add for Q<'a, A, B, C, D>
where
    B: int::Int,
    C: Mode,
    D: Engine,
    (): precision::Ok<A, B> {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: &B = self.v;
        let y: &B = rhs.v;
        Ok(Q::new(D::add(&x, &y)?))
    }
}

impl<'a, const A: u8, B, C, D>




toga::impls!(
    impl<'a, const A: u8, B, C> Q<'a, A, B, C>
    where
        B: int::Int,
        C: Engine,
        (): precision::Ok<A, B>;
    
    pub fn new(v: &B) -> Self {
        Self {
            v,
            engine: core::marker::PhantomData
        }
    }

    pub fn to_negative(self) -> Self {
        Q::new(C::to_negative(self.v))
    }

    pub fn to_positive(self) -> Self {
        Q::new(C::to_positive(self.v))
    }

    core::ops::Add {
        type Output = Result<Self>;

        fn add(self, rhs: Self) -> Self::Output {
            let x: &B = self.v;
            let y: &B = rhs.v;
            Ok(Q::new(C::add(&x, &y)?))
        }
    }

    core::ops::Sub {
        type Output = Result<Self>;

        fn sub(self, rhs: Self) -> Self::Output {
            let x: &B = self.v;
            let y: &B = rhs.v;
            Ok(Q::new(C::sub(&x, &y)?))
        }
    }

    core::ops::Mul {
        type Output = Result<Self>;

        fn mul(self, rhs: Self) -> Self::Output {
            let x: B = self.v;
            let y: B = rhs.v;
            Ok(Q::new(C::mul(x, y)?))   
        }
    }

    core::ops::Div {
        type Output = Result<Self>;

        fn div(self, rhs: Self) -> Self::Output {
            let x: B = self.v;
            let y: B = rhs.v;
            Ok(Q::new(C::div(x, y)?))   
        }
    }

    core::ops::Shl {
        type Output = Result<Self>;

        #[inline]
        fn shl(self, rhs: Self) -> Self::Output {
            let x: B = self.v;
            let y: B = rhs.v;
            let y: u32 = y.try_into().ok_or(Error::InvalidShift)?;
            Ok(Q::new(C::shl(x, y)?))
        }
    }

    core::ops::Shr {
        type Output = Result<Self>;

        #[inline]
        fn shr(self, rhs: Self) -> Self::Output {
            let x: B = self.v;
            let y: B = rhs.v;
            let y: u32 = y.try_into().ok_or(Error::InvalidShift)?;
            Ok(Q::new(C::shr(x, y)?))
        }
    }
);