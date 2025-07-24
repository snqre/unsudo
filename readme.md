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
