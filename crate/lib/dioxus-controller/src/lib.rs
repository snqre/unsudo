use dioxus::prelude::*;

modwire::expose!(
    pub absolute_offset
    pub children
    pub offset
    pub size
);

static AUTO: &str = "auto";

type Stylesheet = String;