use super::*;

pub trait EngineTrigFragment 
where 
    Self: EngineArithmeticFragment,
    Self: EngineTrigConversionFragment {
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
        Self::cos::<A, _>(Self::sub(Self::to_radian::<A, _>(deg90::<A, _>()?)?, angle)?)
    }

    #[inline]
    fn cos<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Ratio<Fixed<B>>>
    where
        B: num::Int {
        let scale: B = scale::<A, _>();
        let pi: B = pi::<A, _>();
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
}