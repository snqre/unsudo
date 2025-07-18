use super::*;

pub struct DegreeMode;

impl<const A: u8, B, C> Q<A, B, DegreeMode, C>
where
    B: int::Int,
    C: Engine,
    (): Ok<A, B> {
    pub fn new(angle: B) -> Self {
        Self {
            data: angle,
            mode: core::marker::PhantomData,
            engine: core::marker::PhantomData
        }
    }

    pub fn cos(self) -> Result<Self> {
        Ok(
            Self::new(
                C::cos(
                    semantic::Radian(
                        semantic::Fixed(self.v)))?))
    }
}