use super::*;

pub trait Engine
where
    Self: EngineArithmeticFragment,
    Self: EngineConversionFragment,
    Self: EngineMuldivFragment,
    Self: EngineSignFragment,
    Self: EngineTrigConversionFragment,
    Self: EngineTrigFragment,
    Self: EngineTrigHyperbolicFragment,
    Self: EngineTrigReciprocalFragment {}

impl<T> Engine for T
where
    T: EngineArithmeticFragment,
    T: EngineConversionFragment,
    T: EngineMuldivFragment,
    T: EngineSignFragment,
    T: EngineTrigConversionFragment,
    T: EngineTrigFragment,
    T: EngineTrigHyperbolicFragment,
    T: EngineTrigReciprocalFragment {}