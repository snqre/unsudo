use super::*;

pub struct DefaultMode;

impl<const A: u8, B, D> Q<A, B, DefaultMode, D> 
where
    B: int::Int,
    D: Engine,
    (): Ok<A, B> {
    pub const fn new(v: B) -> Self {
        Self {
            data: v,
            mode: core::marker::PhantomData,
            engine: core::marker::PhantomData
        }
    }
}