## What should go in `lib.rs` or `main.rs`?
* `lib.rs`: only re-exports public interfaces.
* `main.rs`: only compose and wire hexagons + adaptors; no logic