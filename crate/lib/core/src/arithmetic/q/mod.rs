use super::*;

::modwire::expose!(
    pub default_engine
    pub default_mode_var
    pub default_mode
        deg90
    pub degree_mode
    pub engine_arithmetic_fragment
    pub engine_conversion_fragment
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
    B: num::Int,
    C: Mode,
    D: Engine {
    n: B,
    ph_0: core::marker::PhantomData<C>,
    ph_1: core::marker::PhantomData<D>
}

#[inline]
pub const fn new<const A: u8, B, C, D>(n: B) -> Q<A, B, C, D>
where
    B: num::Int,
    C: Mode,
    D: Engine {
    Q::new(n)
}

impl<const A: u8, B, C, D> Q<A, B, C, D> 
where
    B: num::Int,
    C: Mode,
    D: Engine {
    #[inline]
    pub const fn new(n: B) -> Self {
        Self {
            n,
            ph_0: ::core::marker::PhantomData,
            ph_1: ::core::marker::PhantomData
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
    B: num::Int,
    C: Mode,
    D: Engine {
    #[inline]
    fn from(value: B) -> Self {
        Self::new(value)
    }
}

impl<const A: u8, B, C, D> Default for Q<A, B, C, D>
where
    B: num::Int,
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
    B: num::Int,
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
    B: num::Int,
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
    B: num::Int,
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
    B: num::Int,
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
    B: num::Int,
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
    B: num::Int,
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
    B: num::Int,
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
    B: num::Int,
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
    B: num::Int,
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
    B: num::Int,
    C: Mode,
    D: Engine {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
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