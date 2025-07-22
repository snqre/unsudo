use super::*;

#[derive(Debug)]
pub struct DegreeMode;

impl Mode for DegreeMode {}

impl<const A: u8, B, C> TryFrom<Q<A, B, RadianMode, C>> for Q<A, B, DegreeMode, C>
where
    B: int::Int,
    C: Engine {
    type Error = Error;

    fn try_from(value: Q<A, B, RadianMode, C>) -> core::result::Result<Self, Self::Error> {
        let ret: Self = C::to_degree::<A, _>(value.n)?.into();
        Ok(ret)
    }
}