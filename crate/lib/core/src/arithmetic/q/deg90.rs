use super::*;

#[inline]
pub fn deg90<const A: u8, B>() -> Result<Degree<Fixed<B>>> 
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
    }.checked_mul(scale::<A, _>()).ok_or(Error::Overflow)
}