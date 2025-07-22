use super::*;

modwire::expose!(
    absolute_offset
    children
    offset
    size
);

static AUTO: &str = "auto";

type Stylesheet = String;