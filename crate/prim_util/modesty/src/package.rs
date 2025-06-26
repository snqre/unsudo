#[macro_export]
macro_rules! package {
    ($($module:ident)*) => {
        $(
            mod $module;
            pub use $module::*;
        )*
    };
}