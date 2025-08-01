use super::*;

#[derive(Debug)]
pub struct DefaultEngine;

impl EngineArithmeticFragment for DefaultEngine {}
impl EngineConversionFragment for DefaultEngine {}
impl EngineMuldivFragment for DefaultEngine {}
impl EngineSignFragment for DefaultEngine {}
impl EngineTrigConversionFragment for DefaultEngine {}
impl EngineTrigFragment for DefaultEngine {}
impl EngineTrigHyperbolicFragment for DefaultEngine {}
impl EngineTrigReciprocalFragment for DefaultEngine {}