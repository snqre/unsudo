use crate::arithmetic::num::int;
use crate::arithmetic::q;
use ::core::result;

pub mod point_2d;
pub mod point_3d;
pub mod point_4d;
pub mod point_n;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    Q(q::Error)
}

pub trait Coordinate<
    const A: usize, 
    const B: u8,
          C,
          D> 
where
    C: int::Int,
    D: q::Engine {
    fn distance_between(&self, other: Self) -> Result<q::Q<B, C, q::DefaultMode, D>>;
}