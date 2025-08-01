pub type Result<T> = core::result::Result<T, Error>;

#[repr(u8)]
pub enum Error {
    Overflow,
    Underflow,
    DivByZero,
    RemByZero,
    InvalidShift
}