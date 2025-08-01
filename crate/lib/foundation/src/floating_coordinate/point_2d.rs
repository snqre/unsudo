use crate::arithmetic::num::float;

pub struct Point2D<T> 
where
    T: float::Float {
    pub x: T,
    pub y: T
}