use super::*;

pub trait EngineTrigReciprocalFragment 
where
    Self: EngineArithmeticFragment,
    Self: EngineTrigFragment {
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
        Self::div::<A, _>(scale::<A, _>(), n)
    }
}
