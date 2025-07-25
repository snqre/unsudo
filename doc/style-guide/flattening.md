### Module Flattening
Avoid manually repeating `mod` and `use` for every module. Use `modwire::expose!` for flattened exports:

###### Yes
```rust
modwire::expose!(
    pub module_0
    pub module_1
    module_2
);
```

###### No
```rust
mod module_0
mod module_1
mod module_2
pub use module_0::*;
pub use module_1::*;
use module_2::*;
```