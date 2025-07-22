#![no_std]
use modwire::expose;

pub mod arithmetic;
//pub mod ds;


///
/// ```rs
/// fn do_something(username: &str, balance: u32) -> Result<u32, &str> {
///     require!(username != "Foo" => Err("Invalid username."));
///     require!(balance >= 200 => Err("Insufficient balance."));
///     // ..
/// }
/// ```
/// 
#[macro_export]
macro_rules! require {
    ($condition:expr => $ret:expr) => {
        if !$condition {
            return $ret
        }
    };
}