use super::*;

pub trait EngineTrigHyperbolicFragment 
where
    Self: EngineArithmeticFragment,
    Self: EngineMuldivFragment,
    Self: EngineSignFragment {
    #[inline]
    fn atan<const A: u8, B>(ratio: Ratio<B>) -> Result<Radian<B>>
    where
        B: num::Int {
        let mut pow: B = ratio;
        let mut sum: B = ratio;
        let mut sign: bool = false;
        for i in (3..=25).step_by(2) {
            pow = Self::muldiv(pow, ratio, scale::<A, _>())?;
            pow = Self::muldiv(pow, ratio, scale::<A, _>())?;
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
        B: num::Int {
        if ratio == B::AS_0 {
            return Ok(B::AS_0)
        }
        if ratio == scale::<A, _>() {
            return Self::div::<A, _>(pi::<A, _>(), B::AS_2)
        }
        let sq: B = Self::muldiv(ratio, ratio, scale::<A, _>())?;
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
            pow = Self::muldiv(pow, sq, scale::<A, _>())?;
            let term: B = Self::muldiv(pow, a, scale::<A, _>())?;
            let term: B = Self::div::<A, _>(term, b)?;
            ret = Self::add(ret, term)?;
        }
        Ok(ret)
    }

    #[inline]
    fn acos<const A: u8, B>(ratio: Ratio<B>) -> Result<Radian<B>> 
    where
        B: num::Int {
        let scale: B = scale::<A, _>();
        let pi: B = pi::<A, _>();
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