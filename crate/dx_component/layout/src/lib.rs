use ::prelude::*;
use ::dioxus::prelude::*;

static FILL: &str = r#"
    width: 100%;
    min-width: auto;
    max-width: auto;
    height: 100%;
    min-height: auto;
    max-height: auto;
"#;

#[derive(Clone, PartialEq)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32
}

bundle!(
    auto_grid_fill
    auto_grid
    col_fill
    col
    grid_fill
    grid_item
    grid
    page_item
    page
    row_fill
    row
    stack_fill
    stack_item
    stack
    topo_page_item
    topo_page
);