use super::*;

pub trait Engine 
where
    Self: EngineArithmeticFragment,
    Self: EngineMuldivFragment,
    Self: EngineSignFragment,
    Self: EngineTrigConversionFragment,
    Self: EngineTrigFragment {}

impl<T> Engine for T
where
    T: EngineArithmeticFragment,
    T: EngineMuldivFragment,
    T: EngineSignFragment,
    T: EngineTrigConversionFragment,
    T: EngineTrigFragment {}