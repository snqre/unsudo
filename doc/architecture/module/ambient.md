### Ambient Modules





Ambient Modules
Ambient modules are support modules—they provide traits, utility types, or free functions. The trait (or utility behavior) is the primary focus, and associated types (like errors or results) exist only to support that trait.

Example:

rust
Copy
Edit
// network.rs
pub trait Network {
    fn connect(&self) -> Result<(), Error>;
}

pub struct Error; // supports Network trait
Key Characteristics:

No central concrete type

Typically consists of one or more traits and their helpers

May include free functions or constants

Used across multiple domains (hexagons)

Often reusable, general-purpose abstractions

Summary:

Use hexagon modules for domain logic, where behavior and data are encapsulated behind a named type (e.g., car::Car).

Use ambient modules for supporting abstractions, focused on behavior (traits) without holding state or identity themselves.

This separation helps organize code along architectural boundaries, improves clarity, and makes dependencies between domains and infrastructure explicit.



When the control flow indicates an exit from the current block or flow ie `return` `break` `continue` `panic!()`, `;` should be ommited.