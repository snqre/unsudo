use super::*;

ops!(
    add rhs Self => Self
    sub rhs Self => Self
    mul rhs Self => Self
    div rhs Self => Self
    shl rhs Self => Self
    shr rhs Self => Self
    rem rhs Self => Self
    pow exp u32 => Self
);

ops!(
    abs => Self
    tan => Self
    sin => Self
    cos => Self
    tanh => Self
    sinh => Self
    cosh => Self
    atan => Self
    asin => Self
    acos => Self
    ceil => Self
    floor => Self
    trunc => Self
    to_degree => Self
    to_radian => Self
);

macro_rules! impls {
    ($($size:ty)*) => {
        paste::paste!(
            $(
                impl Float for $size {
                    fn powf(self, exp: Self) -> Self {
                        self.powf(exp)
                    }

                    fn powi(self, exp: i32) -> Self {
                        self.powi(exp)
                    }
                }

                impl ops::Add for $size {
                    add!();
                }

                impl ops::Sub for $size {
                    sub!();
                }

                impl ops::Mul for $size {
                    mul!();
                }

                impl ops::Div for $size {
                    div!();
                }

                impl ops::Shl for $size {
                    shl!();
                }

                impl ops::Shr for $size {
                    shr!();
                }

                impl ops::Rem for $size {
                    rem!();
                }

                impl ops::Pow for $size {
                    pow!();
                }

                impl ops::Abs for $size {
                    abs!();
                }

                impl ops::Tan for $size {
                    tan!();
                }

                impl ops::Sin for $size {
                    sin!();
                }

                impl ops::Cos for $size {
                    cos!();
                }

                impl ops::Tanh for $size {
                    tanh!();
                }

                impl ops::Sinh for $size {
                    sinh!();
                }

                impl ops::Cosh for $size {
                    cosh!();
                }

                impl ops::ArcTan for $size {
                    atan!();
                }

                impl ops::ArcSin for $size {
                    asin!();
                }

                impl ops::ArcCos for $size {
                    acos!();
                }

                impl ops::Ceil for $size {
                    ceil!();
                }

                impl ops::Floor for $size {
                    floor!();
                }

                impl ops::Trunc for $size {
                    trunc!();
                }

                impl ops::ToDegree for $size {
                    to_degree!();
                }

                impl ops::ToRadian for $size {
                    to_radian!();
                }
            )*
        );
    };
}

pub trait Float 
where
    Self: Sized,
    Self: Copy,
    Self: ops::ArcTan,
    Self: ops::ArcSin,
    Self: ops::ArcCos,
    Self: ops::Tanh,
    Self: ops::Sinh,
    Self: ops::Cosh,
    Self: ops::Tan,
    Self: ops::Sin,
    Self: ops::Cos,
    Self: ops::Ceil,
    Self: ops::Floor,
    Self: ops::Trunc,
    Self: core::ops::Add<Output=Self>,
    Self: core::ops::Sub<Output=Self>,
    Self: core::ops::Mul<Output=Self>,
    Self: core::ops::Div<Output=Self>,
    Self: core::ops::Rem<Output=Self>,
    Self: ops::Add,
    Self: ops::Sub,
    Self: ops::Mul,
    Self: ops::Div,
    Self: ops::Rem,
    Self: ops::ToDegree,
    Self: ops::ToRadian,
    Self: ops::Abs {
    fn powf(self, exp: Self) -> Self;
    fn powi(self, exp: i32) -> Self;
}

impls!(
    f32 
    f64
);
impl Bits32 for f32 {}
impl Bits64 for f64 {}