use super::*;

pub trait EngineConversionFragment {
    fn cast<const A: u8, const B: u8, C>(n: C) -> Result<C>
    where
        C: num::Int {
        if B > A {
            
        } else if B < A {

        } else {
            Ok(n)
        }
    }
}