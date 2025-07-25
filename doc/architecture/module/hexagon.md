### Hexagon Modules or Domain
These are modules that define a concrete type as their central focus—typically named after the module itself. They may also include associated domain logic, supporting types, and ports (traits the domain depends on).

* Concrete domain type: `car::Car`.
* May define or depend on ports: `car::Engine`.
* May expose domain-specific errors or helpers.
* Self-contained unit of behavior and state.
* Called a hexagon in line with Hexagonal Architecture.

#### Example:
```rust
mod car {
    // Port
    pub trait Engine {
        fn vroom(&self) -> Result<u8> {
            Ok(200)
        }
    }

    pub type Result<T> = core::result::Result<T, Error>;

    pub enum Error {
        MissingEngine
    }

    pub struct Car<T: Engine>(Option<T>);
    impl Car {
        pub fn drive(&self) -> Result<u8> {
            if let Some(engine) = self.0 {
                return engine.vroom()
            }
            return Err(Error::MissingEngine)
        }
    }
}

```