use super::*;

modwire::expose!(
    default
    degree
    radian
);

pub trait Mode {}
impl Mode for DefaultMode {}
impl Mode for RadianMode {}
impl Mode for DegreeMode {}