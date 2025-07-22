macro_rules! impl_native {
    ($size:ty) => {
        paste::paste!(
            impl Float for $size {}
        );
    };
    ($($size:ty)*) => {
        paste::paste!(
            $(
                impl_native!($size);
            )*
        );
    };
}

pub trait Float 
where
    Self: Sized,
    Self: Default,
    Self: Copy,
    Self: PartialEq,
    Self: PartialOrd,
    Self: core::ops::Add<Output = Self>,
    Self: core::ops::AddAssign,
    Self: core::ops::Sub<Output = Self>,
    Self: core::ops::SubAssign,
    Self: core::ops::Mul<Output = Self>,
    Self: core::ops::MulAssign,
    Self: core::ops::Div<Output = Self>,
    Self: core::ops::DivAssign,
    Self: core::ops::Rem<Output = Self>,
    Self: core::ops::RemAssign {}

impl_native!(
    f32
    f64
);