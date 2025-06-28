#[macro_export]
macro_rules! bundle {
    ($($module:ident)*) => {
        $(
            mod $module;
            pub use $module::*;
        )*
    };
}