use super::*;

pub trait EngineArithmeticFragment 
where 
    Self: EngineMuldivFragment {
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
        Self::muldiv(x, y, scale::<A, _>())
    }

    #[inline]
    fn div<const A: u8, B>(x: Fixed<B>, y: Fixed<B>) -> Result<Fixed<B>> 
    where
        B: num::Int {
        let scale: u128 = scale::<A, _>();
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

    #[inline]
    fn rem<T>(x: T, y: T) -> Result<T>
    where
        T: num::Int {
        if y == T::AS_0 {
            return Err(Error::RemByZero)
        }
        Ok(x % y)
    }

    fn shl<T>(x: T, y: T) -> Result<T> 
    where
        T: num::Int {
        let y: u32 = y
            .try_into()
            .ok()
            .ok_or(Error::InvalidShift)?;
        x.checked_shl(y).ok_or(Error::InvalidShift)
    }

    fn shr<T>(x: T, y: T) -> Result<T>
    where
        T: num::Int {
        let y: u32 = y
            .try_into()
            .ok()
            .ok_or(Error::InvalidShift)?;
        x.checked_shr(y).ok_or(Error::InvalidShift)
    }

    #[inline]
    fn bitand<T>(x: T, y: T) -> T
    where
        T: num::Int {
        x & y
    }

    #[inline]
    fn bitor<T>(x: T, y: T) -> T
    where
        T: num::Int {
        x | y
    }
}