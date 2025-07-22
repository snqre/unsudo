use super::*;

pub trait EngineSignFragment {
    #[inline]
    fn to_negative<T>(n: T) -> T 
    where 
        T: num::Int {
        if n == T::AS_0 {
            T::AS_0
        } else {
            T::AS_0 - n
        }
    }

    #[inline]
    fn to_positive<T>(n: T) -> T 
    where 
        T: num::Int {
        if n == T::AS_0 {
            T::AS_0
        } else if n > T::AS_0 {
            n
        } else {
            T::AS_0 - n
        }
    }
}