use ::dioxus::prelude::*;
use ::dioxus_extendable::ChainEventHandler as _;
use ::dioxus_extendable::ChainEventHandlerExt as _;
use ::dioxus_extendable::EditExt as _;
use ::dioxus_extendable::Edit as _;
use ::dioxus_extendable::EditClassExt as _;
use ::dioxus_extendable::EditClass as _;
use ::dioxus_extendable::EditStyleExt as _;
use ::dioxus_extendable::EditStyle;
use ::dioxus_extendable as extendable;

::modwire::expose!(
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

static ABSOLUTE_POSITION_RESET_STYLE_MOD: &str = r#"
    top: 0%;
    left: 0%;
"#;

static VIEW_FILL_STYLE_MOD: &str = r#"
    min-width: 100vw;
    max-width: 100vw;
    width: 100vw;
    min-height: 100vh;
    max-height: 100vh;
    height: 100vh;
"#;

static FILL_STYLE_MOD: &str = r#"
    min-width: 100%;
    max-width: 100%;
    width: 100%;
    min-height: 100%;
    max-height: 100%;
    height: 100%;
"#;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32
}