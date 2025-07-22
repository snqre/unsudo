use super::*;

pub mod float;
pub mod int;
pub mod num;
pub mod ops;
pub mod q;
    mod semantic;

pub type Result<T> = core::result::Result<T, Error>;

#[repr(u8)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero,
    InvalidShift
}