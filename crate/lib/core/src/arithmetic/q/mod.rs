use super::*;

modwire::expose!(
    pub default_mode_var
    pub default_mode
        deg90
    pub degree_mode
    pub engine_arithmetic_fragment
    pub engine_muldiv_fragment
    pub engine_sign_fragment
    pub engine_trig_conversion_fragment
    pub engine_trig_fragment
    pub engine_trig_hyperbolic_fragment
    pub engine_trig_reciprocal_fragment
    pub engine
    pub err
    pub mode
        pi
    pub radian_mode_var
    pub radian_mode
    pub ratio_frac_mode
    pub ratio_mode
        scale
    pub semantic
);

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



impl Mode for DefaultMode {}
impl Mode for RadianMode {}
impl Mode for DegreeMode {}


//
// port.engine
//

pub trait Engines 
where 
    Self: Sized {



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












}

impl Engine for DefaultEngine {}

mod any_mode {
    use super::*;


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

    #[rstest::rstest]
    fn rad_cos(
        #[case] x: Rad2<u128>,
        #[case] y: Rad2<u128>,
        #[case] cor: Rad2<u)    
}