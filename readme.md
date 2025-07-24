The workspace exposes central dependencies with versions of stable builds of each internal dependency. If a specific crate requires a different version of a crate, it can do so but that is not preferred. The public projects are automatically up to date with all changes from the workpace's internal crates, change the version and republish when certain crates have changed. Public crates reexport internal crates, and must start with the `unsudo` namespace.

# 🦀 Rust Code Style Guide

This guide defines the preferred code structure and ordering for Rust modules in this project. The goal is to maximize clarity, encourage consistent patterns, and support abstraction-first design.

---

## 📦 Module Item Order

Each Rust module should follow this structure:

```rust
// === Imports ===
use std::collections::HashMap;

// === Macros ===
macro_rules! my_macro { ... }

// === Traits ===
pub trait MyTrait { ... }

// === Type Aliases ===
pub type MyAlias = Box<dyn MyTrait>;

// === Data Structures ===
pub struct MyStruct { ... }

// === Inherent Implementations ===
impl MyStruct {
    pub fn new() -> Self { ... }
}

// === Trait Implementations ===
impl MyTrait for MyStruct {
    fn do_thing(&self) { ... }
}

// === Free Functions ===
pub fn make_struct() -> MyStruct { ... }
```

```rust
// avoid destructuring modules and keep the namespace clean. more verbosity is the cost of a clean namespace, and renaming entire sections of code becomes easier.
use core::result;

// public macro on top
#[macro_export()]
macro_rules! macro_0 {}
macro_rules! macro_1 {}
macro_rules! macro_2 {}

// internally public macro middle
macro_rules! macro_2 {}

// private macro below
macro_rules! macro_0 {}
macro_rules! macro_1 {}
macro_rules! macro_2 {}

// public trait above
pub trait Public {}

// below
trait Private {}



pub struct HelloWorld {}

impl HelloWorld {}

impl Public for HelloWorld {}
```



```rust
pub struct Engine;

// The port/dependencies of the domain should be located within the same module as the type/struct/enum to allow for easier discoverability.
impl car::Engine for Engine {
    
    // All required types should mostly be accesible from the same entry point.
    fn vroom(&self) -> car::Result<&'static str> {
        "vroom"
    }
}


// default type of the module
car::Car {

}

```




### Import #1

###### Yes
```rust
use ::serde;

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
struct Foo;

```
###### No
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
struct Foo;
```


### Import #2

###### No
```rust
use crate::shape;

fn main() {
    let circle: shape::circle::Circle = shape::circle::Circle::default();
}

```
###### Yes
```rust
use crate::shape::circle;

fn main() {
    let circle: circle::Circle = circle::Circle::default();
}
```




### Generics & Trait Bounds
When writing trait bounds in a `where` clause with one constraint per line:

```rust
fn foo<T>()
where
    T: Sized,
    T: Clone,
    T: Copy {}
``` 

You get cleaner IDE tooltips, better diffs, and easier visual scanning. Each trait is clearly separated, and IDEs like VS Code, IntelliJ Rust, or rust-analyzer will show better per-bound suggestions and signature previews.
Compare this to the `+` form:

```rust
fn foo<T: Sized + Clone + Copy>() {}
```

```rust
fn foo<T>()
where
    T Sized + Clone + Copy;
```

* Harder to parse at a glance (especially with long generic bounds).
* IDEs often collapse or truncate long inline constraints.
* Less diff-friendly when adding/removing bounds.

###### Yes
```rust
fn bar<T>()
where
    T: Sized,
    T: Clone,
    T: Copy {}
```

###### No
```rust
fn bar<T>()
where
    T Sized + Clone + Copy;
```

###### Yes
```rust
trait SizedCopy 
where
    Self: Sized,
    Self: Clone,
    Self: Copy {}

impl<T> SizedCopy for T
where
    T: Sized,
    T: Clone,
    T: Copy {}

fn bar<T: SizedCopy>() {}
```

###### No
```rust
fn bar<T: Sized + Clone + Copy>() {}
```


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