#![no_std]


struct Network<const A: usize> {
    listeners: [FnMut(); A]
}