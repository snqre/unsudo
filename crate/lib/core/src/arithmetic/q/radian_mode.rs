use super::*;

#[derive(Debug)]
pub struct RadianMode;

impl Mode for RadianMode {}

impl<const A: u8, B, C> TryFrom<Q<A, B, DegreeMode, C>> for Q<A, B, RadianMode, C>
where
    B: num::Int,
    C: Engine {
    type Error = Error;

    fn try_from(value: Q<A, B, DegreeMode, C>) -> core::result::Result<Self, Self::Error> {
        let ret: Self = C::to_radian::<A, _>(value.n)?.into();
        Ok(ret)
    }
}

impl<const A: u8, B, C> Q<A, B, RadianMode, C> 
where
    B: num::Int,
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