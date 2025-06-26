//! components that do not affect the layout but provide supporting
//! abilities to their children. when the children overide their
//! styling or certain properties these will be ignored.

use super::*;

leak!(
    children
    absolute_offset
    offset
    size
);

static AUTO: &str = "auto";

type Stylesheet = String;