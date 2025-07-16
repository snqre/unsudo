use super::*;

pub mod float;
pub mod int;
pub mod num;
    mod ops_gen;
pub mod ops;
    mod precision;
pub mod q;
    mod scale;

pub trait Bits8 {}
pub trait Bits16 {}
pub trait Bits32 {}
pub trait Bits64 {}
pub trait Bits128 {}