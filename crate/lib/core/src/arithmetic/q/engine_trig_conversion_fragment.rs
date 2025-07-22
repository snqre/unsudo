use super::*;

pub trait EngineTrigConversionFragment 
where
    Self: EngineMuldivFragment {
    #[inline]
    fn to_radian<const A: u8, B>(angle: Degree<Fixed<B>>) -> Result<Radian<Fixed<B>>>
    where
        B: num::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            n.try_into().unwrap_unchecked()
        };
        Self::muldiv(angle, pi::<A, _>(), n * scale::<A, _>())
    }

    #[inline]
    fn to_degree<const A: u8, B>(angle: Radian<Fixed<B>>) -> Result<Degree<Fixed<B>>>
    where
        B: num::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            n.try_into().unwrap_unchecked()
        };
        Self::muldiv(angle, n * scale::<A, _>(), pi::<A, _>())
    }
}