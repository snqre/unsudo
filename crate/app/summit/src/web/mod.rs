use super::*;
use ::dioxus_motion::prelude::*;
use ::dioxus_extendable::ChainEventHandler as _;
use ::dioxus_extendable::ChainEventHandlerExt as _;
use ::dioxus_extendable::Edit as _;
use ::dioxus_extendable::EditClass as _;
use ::dioxus_extendable::EditClassExt as _;
use ::dioxus_extendable::EditExt as _;
use ::dioxus_extendable::EditStyle as _;
use ::dioxus_extendable::EditStyleExt as _;
use ::dioxus_extendable as extendable;
use ::dioxus_layout as layout;

pub mod component;
pub mod route;
pub mod color;
pub mod easing;
pub mod kernel;

#[allow(unused)]
pub fn u(k: u16) -> f64 {
    let base: f64 = 8.0;
    let ratio: f64 = 1.618f64;
    let value: f64 = base * ratio.powf(k as f64);
    value.round()
}

#[allow(unused)]
pub fn vrem(value: f64) -> String {
    "{value}vw + 1rem".to_owned()
}

pub fn sequence(k: u16) -> f64 {
    let base: f64 = 8.0f64;
    let ratio: f64 = 1.618f64;
    let value: f64 = base * ratio.powf(k as f64);
    value.round()
}



#[derive(Clone)]
#[derive(PartialEq)]
pub enum Url {
    External(String),
    Internal(Asset)
}

impl ToString for Url {
    fn to_string(&self) -> String {
        match self {
            Self::External(x) => x.to_string(),
            Self::Internal(x) => x.to_string()
        }
    }
}