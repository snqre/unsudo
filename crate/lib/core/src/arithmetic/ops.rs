macro_rules! ops {
    ($($trait_name:ident $fn_name:ident $fn_rhs_name:ident $fn_rhs_ty:ty => $fn_out_ty:ty)*) => {
        $(
            pub trait $trait_name 
            where
                Self: Sized {
                fn $fn_name(self, $fn_rhs_name: $fn_rhs_ty) -> $fn_out_ty;
            }
        )*
    };
}

macro_rules! ops_with_no_args {
    ($($trait_name:ident $fn_name:ident => $fn_out_ty:ty)*) => {
        $(
            pub trait $trait_name
            where
                Self: Sized {
                fn $fn_name(self) -> $fn_out_ty;
            }
        )*
    };
}

ops_with_no_args!(
    WrappingNeg wrapping_neg => Self
    OverflowingNeg overflowing_neg => (Self, bool)
    Abs abs => Self
    Tan tan => Self
    Sin sin => Self
    Cos cos => Self
    Tanh tanh => Self
    Sinh sinh => Self
    Cosh cosh => Self
    ArcTan atan => Self
    ArcSin asin => Self
    ArcCos acos => Self
    Ceil ceil => Self
    Floor floor => Self
    Trunc trunc => Self
    ToDegree to_degree => Self
    ToRadian to_radian => Self
);






pub trait ArithmeticOps where Self: Sized {
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn shl(self, rhs: Self) -> Self;
    fn shr(self, rhs: Self) -> Self;
    fn rem(self, rhs: Self) -> Self;
}

pub trait CheckedOps where Self: Sized {
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
}

pub trait WrappingOps where Self: Sized {
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn wrapping_neg(self) -> Self;
}

pub trait OverflowingOps where Self: Sized {
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);
    fn overflowing_neg(self) -> (Self, bool);
}

pub trait TrigOps where Self: Sized {
    fn tan(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

pub trait ArcTrigOps where Self: Sized {
    fn arc_tan(self) -> Self;
    fn arc_sin(self) -> Self;
    fn arc_cos(self) -> Self;
}

pub trait UnboundOps where Self: Sized {
    fn unbound_shl(self, rhs: u32) -> Self;
    fn unbound_shr(self, rhs: u32) -> Self;
}

impl<T> ArithmeticOps for T
where
    T: core::ops::Add<Output = Self>,
    T: core::ops::Sub<Output = Self>,
    T: core::ops::Mul<Output = Self>,
    T: core::ops::Div<Output = Self> {
    fn add(self, rhs: Self) -> Self {
        self + rhs
    }

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }

    fn mul(self, rhs: Self) -> Self {
        self * rhs
    }

    fn div(self, rhs: Self) -> Self {
        self / rhs
    }
}





ops!(
    Add add rhs Self => Self
    Sub sub rhs Self => Self
    Mul mul rhs Self => Self
    Div div rhs Self => Self
    Shl shl rhs Self => Self
    Shr shr rhs Self => Self
    Rem rem rhs Self => Self
    Pow pow exp u32 => Self
    SaturatingAdd saturating_add rhs Self => Self
    SaturatingSub saturating_sub rhs Self => Self
    SaturatingMul saturating_mul rhs Self => Self
    SaturatingDiv saturating_div rhs Self => Self
    SaturatingPow saturating_pow exp u32 => Self
    WrappingAdd wrapping_add rhs Self => Self
    WrappingSub wrapping_sub rhs Self => Self
    WrappingMul wrapping_mul rhs Self => Self
    WrappingDiv wrapping_div rhs Self => Self
    WrappingRem wrapping_rem rhs Self => Self
    WrappingPow wrapping_pow exp u32 => Self
    CheckedAdd checked_add rhs Self => Option<Self>
    CheckedSub checked_sub rhs Self => Option<Self>
    CheckedMul checked_mul rhs Self => Option<Self>
    CheckedDiv checked_div rhs Self => Option<Self>
    CheckedShl checked_shl rhs u32 => Option<Self>
    CheckedShr checked_shr rhs u32 => Option<Self>
    CheckedPow checked_pow rhs u32 => Option<Self>
    CheckedIlog checked_ilog base Self => Option<u32>
    OverflowingAdd overflowing_add rhs Self => (Self, bool)
    OverflowingSub overflowing_sub rhs Self => (Self, bool)
    OverflowingMul overflowing_mul rhs Self => (Self, bool)
    OverflowingDiv overflowing_div rhs Self => (Self, bool)
    OverflowingRem overflowing_rem rhs Self => (Self, bool)
    OverflowingShl overflowing_shl rhs u32 => (Self, bool)
    OverflowingShr overflowing_shr rhs u32 => (Self, bool)
    OverflowingPow overflowing_pow exp u32 => (Self, bool)
    UnboundShl unbound_shl rhs u32 => Self
    UnboundShr unbound_shr rhs u32 => Self
    Max max rhs Self => Self
    Min min rhs Self => Self
);

pub trait Clamp {
    fn clamp(self, min: Self, max: Self) -> Self;
}