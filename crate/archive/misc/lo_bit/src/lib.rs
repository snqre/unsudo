#![no_std]
#![forbid(clippy::panic)]

use core::hash;
use core::iter;
use core::slice;
use core::marker;
use core::str;
use ::core::ops as ops;
use ::core::fmt as fmt;
use ::core::result as result;

mod utf8;
mod map;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
pub enum Error {
    Overflow,
    Underflow,
    DivByZero,
    UnsupportedConversion,
    Empty,
    EmptyKey,
    ZeroChunkSize,
    InvalidUtf8
}