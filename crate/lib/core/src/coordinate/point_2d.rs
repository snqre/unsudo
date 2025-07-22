use super::*;

pub struct Point2d<const A: u8, B, C=q::DefaultEngine>
where
    B: int::Int,
    C: q::Engine {
    pub x: q::Q<A, B, q::DefaultMode, C>,
    pub y: q::Q<A, B, q::DefaultMode, C>
}

impl<const A: u8, B, C> Coordinate<10, A, B, C> for Point2d<> 
where
    B: int::Int,
    C: q::Engine {
    fn distance_between(&self, other: Self) -> Result<q::Q<A, B, q::DefaultMode, C>> {
        (|| -> q::Result<q::Q<A, B, q::DefaultMode, C>> {
            let dx: q::Q<A, B, q::DefaultMode, C> = (self.x - other.x)?;
            let dx: q::Q<A, B, q::DefaultMode, C> = (dx * dx)?;
            let dy: q::Q<A, B, q::DefaultMode, C> = (self.y - other.y)?;
            let dy: q::Q<A, B, q::DefaultMode, C> = (dy * dy)?;
            let sum: q::Q<A, B, q::DefaultMode, C> = (dx + dy)?;
            let sum: q::Q<A, B, q::DefaultMode, C> = sum.sqrt();
            Ok(sum)
        })().map_err(Error::Q)
    }
}