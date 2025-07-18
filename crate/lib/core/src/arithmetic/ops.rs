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


pub trait TrigOps {
    type Output;
    fn tan(self) -> Self::Output;
    fn sin(self) -> Self::Output;
    fn cos(self) -> Self::Output;
    fn arc_tan(self) -> Self::Output;
    fn arc_sin(self) -> Self::Output;
    fn arc_cos(self) -> Self::Output;
    fn tanh(self) -> Self::Output;
    fn sinh(self) -> Self::Output;
    fn cosh(self) -> Self::Output;
}



ops_with_no_args!(
    WrappingNeg wrapping_neg => Self
    OverflowingNeg overflowing_neg => (Self, bool)
    Abs abs => Self

  
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