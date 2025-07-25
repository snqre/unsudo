# Generics & Trait Bounds
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

## Yes
```rust
fn bar<T>()
where
    T: Sized,
    T: Clone,
    T: Copy {}
```

## No
```rust
fn bar<T>()
where
    T Sized + Clone + Copy;
```

## Yes
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

## No
```rust
fn bar<T: Sized + Clone + Copy>() {}
```