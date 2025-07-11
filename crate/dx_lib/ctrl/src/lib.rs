#![deny(warnings)]

use ::prelude::*;
use ::dioxus::prelude::*;

static AUTO: &str = "auto";

type Stylesheet = String;

bundle!(
    absolute_offset
    children
    offset
    size
);