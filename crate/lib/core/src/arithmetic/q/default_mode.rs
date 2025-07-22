use super::*;

#[derive(Debug)]
pub struct DefaultMode;

impl Mode for DefaultMode {}

impl<const A: u8, B, C> From<Q<A, B, RadianMode, C>> for Q<A, B, DefaultMode, C>
where
    B: int::Int,
    C: Engine {
    fn from(value: Q<A, B, RadianMode, C>) -> Self {
        new(value.n)
    }
}

impl<const A: u8, B, C> From<Q<A, B, DegreeMode, C>> for Q<A, B, DefaultMode, C>
where
    B: int::Int,
    C: Engine {
    fn from(value: Q<A, B, DegreeMode, C>) -> Self {
        new(value.n)
    }
}