### Item Visibility and Grouping Order
To maintain clarity and consistency, modules should follow a logical and visible ordering of items. The general order is:

1. Macros
2. Traits
3. Type Aliases
4. Structs/Enum/Type (Grouped with Their impls)
5. Free Functions

```rust
macro_rules! some_macro! {}
pub trait SomeTrait {}

pub type SomeType = &str;

pub struct SomeStruct;
impl SomeStruct {}

fn bar() {}
```

`struct` and their associated `impl` blocks are treated as a single unit and should not be separated within the same file.

To promote readability and maintain a consistent structure across modules, public items should always appear before their related private counterparts within the same group (e.g., structs, enums, traits). This applies especially when grouping a data type with its implementation block. For instance:

```rust
    pub enun Error {}
    impl Error {}

    enum InternalError {}
    impl InternalError {}
```

In this example, Error is a public enum and appears at the top of its group, followed by its impl block. The private `InternalError` and its `impl` are placed beneath it. This ordering ensures that developers reading the module can quickly locate and understand the public interface, without being distracted by internal or supporting details. It also aligns with the principle of progressive disclosure, where the most relevant, externally visible types come first, while internal logic is layered underneath in a structured and predictable manner.