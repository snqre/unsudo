use ::prelude::*;
use ::dioxus::prelude::*;

fn fill() -> String {
    r#"
        width: 100%;
        min-width: auto;
        max-width: auto;
        height: 100%;
        min-height: auto;
        max-height: auto;
    "#.to_owned()
}

bundle!(
    auto_grid
    col
    filled_auto_grid
    filled_col
    filled_grid
    filled_row
    filled_stack
    grid_item
    grid
    page_item
    page
    row
    stack_item
    stack
    topo_page_item
    topo_page
);