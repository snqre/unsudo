/// Flattens the module structure for consumption.
/// # Example
/// ```rs
/// expose!(
///     merged_module_0
///     merged_module_1
///     merged_module_2
/// );
/// ```
/// # Example Expansion
/// ```rs
/// mod merged_module_0; pub use merged_module_0::*;
/// mod merged_module_1; pub use merged_module_1::*;
/// mod merged_module_2; pub use merged_module_2::*;
/// ```
#[macro_export]
macro_rules! expose {
    ($($vis:vis $module:ident)*) => {
        $(
            mod $module; $vis use $module::*;
        )*
    };
}