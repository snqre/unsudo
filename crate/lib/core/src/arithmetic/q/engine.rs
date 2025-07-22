use super::*;

pub trait Engine<T> 
where
    Self: EngineArithmeticFragment<T>,
    Self: EngineConversionFragment,
    Self: EngineMuldivFragment,
    Self: EngineSignFragment,
    Self: EngineTrigConversionFragment,
    Self: EngineTrigFragment,
    T: num::Int {}

impl<T> Engine for T
where
    T: EngineArithmeticFragment,
    T: EngineConversionFragment,
    T: EngineMuldivFragment,
    T: EngineSignFragment,
    T: EngineTrigConversionFragment,
    T: EngineTrigFragment {}