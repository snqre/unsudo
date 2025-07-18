use super::*;

pub struct RadianMode;

impl<const A: u8, B, C> Q<A, B, DefaultMode, C>
where
    B: int::Int,
    C: Engine,
    (): Ok<A, B> {
    
    #[inline]
    pub fn cos(self) -> Result<Self> {
        
    }
}