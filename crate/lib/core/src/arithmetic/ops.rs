use super::*;

pub trait Overflowing 
where
    Self: OverflowingAdd,
    Self: OverflowingSub,
    Self: OverflowingMul,
    Self: OverflowingDiv,
    Self: OverflowingRem,
    Self: OverflowingShl,
    Self: OverflowingShr,
    Self: OverflowingPow {}

pub trait OverflowingAdd 
where
    Self: Sized {
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
}

pub trait OverflowingSub
where
    Self: Sized {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
}

pub trait OverflowingMul
where
    Self: Sized {
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
}

pub trait OverflowingDiv
where
    Self: Sized {
    fn overflowing_div(self, rhs: Self) -> (Self, bool);
}

pub trait OverflowingRem
where
    Self: Sized {
    fn overflowing_rem(self, rhs: Self) -> (Self, bool);
}

pub trait OverflowingShl
where
    Self: Sized {
    fn overflowing_shl(self, rhs: u32) -> (Self, bool);
}

pub trait OverflowingShr
where
    Self: Sized {
    fn overflowing_shr(self, rhs: u32) -> (Self, bool);
}

pub trait OverflowingPow
where
    Self: Sized {
    fn overflowing_pow(self, exp: u32) -> (Self, bool);
}

impl<T> Overflowing for T
where
    T: OverflowingAdd,
    T: OverflowingSub,
    T: OverflowingMul,
    T: OverflowingDiv,
    T: OverflowingRem,
    T: OverflowingShl,
    T: OverflowingShr,
    T: OverflowingPow {}


// #region

pub trait Wrapping
where
    Self: WrappingAdd,
    Self: WrappingSub,
    Self: WrappingMul,
    Self: WrappingDiv,
    Self: WrappingRem,
    Self: WrappingPow,
    Self: WrappingNeg {}

pub trait WrappingAdd {
    fn wrapping_add(self, rhs: Self) -> Self;
}

pub trait WrappingSub {
    fn wrapping_sub(self, rhs: Self) -> Self;
}

pub trait WrappingMul {
    fn wrapping_mul(self, rhs: Self) -> Self;
}

pub trait WrappingDiv {
    fn wrapping_div(self, rhs: Self) -> Self;
}

pub trait WrappingRem {
    fn wrapping_rem(self, rhs: Self) -> Self;
}

pub trait WrappingPow {
    fn wrapping_pow(self, exp: u32) -> Self;
}

pub trait WrappingNeg {
    fn wrapping_neg(self) -> Self;
}

impl<T> Wrapping for T
where
    T: WrappingAdd,
    T: WrappingSub,
    T: WrappingMul,
    T: WrappingDiv,
    T: WrappingRem,
    T: WrappingPow,
    T: WrappingNeg {}


// #region

pub trait Saturating
where
    Self: SaturatingAdd,
    Self: SaturatingSub,
    Self: SaturatingMul,
    Self: SaturatingDiv {}

pub trait SaturatingAdd {
    fn saturating_add(self, rhs: Self) -> Self;
}

pub trait SaturatingSub {
    fn saturating_sub(self, rhs: Self) -> Self;
}

pub trait SaturatingMul {
    fn saturating_mul(self, rhs: Self) -> Self;
}

pub trait SaturatingDiv {
    fn saturating_div(self, rhs: Self) -> Self;
}

impl<T> Saturating for T
where
    T: SaturatingAdd,
    T: SaturatingSub,
    T: SaturatingMul,
    T: SaturatingDiv {}


// #region

pub trait Checked
where
    Self: CheckedAdd,
    Self: CheckedSub,
    Self: CheckedMul,
    Self: CheckedDiv,
    Self: CheckedShl,
    Self: CheckedShr,
    Self: CheckedPow {}

pub trait CheckedAdd 
where
    Self: Sized {
    fn checked_add(self, rhs: Self) -> Result<Self>;
}

pub trait CheckedSub
where
    Self: Sized {
    fn checked_sub(self, rhs: Self) -> Result<Self>;
}

pub trait CheckedMul
where
    Self: Sized {
    fn checked_mul(self, rhs: Self) -> Result<Self>;
}

pub trait CheckedDiv
where
    Self: Sized {
    fn checked_div(self, rhs: Self) -> Result<Self>;
}

pub trait CheckedShl
where
    Self: Sized {
    fn checked_shl(self, rhs: Self) -> Result<Self>;
}

pub trait CheckedShr
where
    Self: Sized {
    fn checked_shr(self, rhs: Self) -> Result<Self>;
}

pub trait CheckedPow
where
    Self: Sized {
    fn checked_pow(self, rhs: Self) -> Result<Self>;
}

impl<T> Checked for T
where
    T: CheckedAdd,
    T: CheckedSub,
    T: CheckedMul,
    T: CheckedDiv,
    T: CheckedShl,
    T: CheckedShr,
    T: CheckedPow {}


// #region

pub trait ToRadian
where
    Self: Sized {
    fn to_radian(self) -> Result<Self>;
}

pub trait ToDegree
where
    Self: Sized {
    fn to_degree(self) -> Result<Self>;
}


// #region

pub trait Trig
where
    Self: Tan,
    Self: Sin,
    Self: Cos {}

pub trait Tan 
where
    Self: Sized {
    fn tan(self) -> Result<Self>;
}

pub trait Sin 
where
    Self: Sized {
    fn sin(self) -> Result<Self>;
}

pub trait Cos 
where
    Self: Sized {
    fn cos(self) -> Result<Self>;
}

impl<T> Trig for T
where
    T: Tan,
    T: Sin,
    T: Cos {}


// #region

pub trait TrigHyperbolic
where
    Self: Tanh,
    Self: Sinh,
    Self: Cosh {}

pub trait Tanh
where
    Self: Sized {
    fn tanh(self) -> Result<Self>;
}

pub trait Sinh
where
    Self: Sized {
    fn sinh(self) -> Result<Self>;
}

pub trait Cosh
where
    Self: Sized {
    fn cosh(self) -> Result<Self>;
}

impl<T> TrigHyperbolic for T
where
    T: Tanh,
    T: Sinh,
    T: Cosh {}


// #region

pub trait TrigInverse
where
    Self: ArcTan,
    Self: ArcSin,
    Self: ArcCos {}

pub trait ArcTan 
where
    Self: Sized {
    fn atan(self) -> Result<Self>;
}

pub trait ArcSin
where
    Self: Sized {
    fn asin(self) -> Result<Self>;
}

pub trait ArcCos
where
    Self: Sized {
    fn acos(self) -> Result<Self>;
}

impl<T> TrigInverse for T 
where
    T: ArcTan,
    T: ArcSin,
    T: ArcCos {}


// #region

pub trait TrigReciprocal
where
    Self: Sec,
    Self: Csc,
    Self: Cot {}

pub trait Sec 
where
    Self: Sized {
    fn sec(self) -> Result<Self>;
}

pub trait Csc
where
    Self: Sized {
    fn csc(self) -> Result<Self>;
}

pub trait Cot
where
    Self: Sized {
    fn cot(self) -> Result<Self>;
}

impl<T> TrigReciprocal for T
where
    T: Sec,
    T: Csc,
    T: Cot {}