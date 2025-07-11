#![deny(warnings)]

use ::dioxus::prelude::*;
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

static FILL_MOD: &extendable::AttrsProps = &extendable::AttrsProps {
    access_key: None,
    align_content: None,
    aria_active_descendant: None,
    align_items: None,
    align_self: None,
    alignment_adjust: None,
    alignment_baseline: None,
    all: None,
    alt: None,
    animation: None,
    animation_delay: None,
    animation_direction: None,
    animation_duration: None,
    animation_fill_mode: None,
    animation_name: None,
    animation_play_state: None,
    animation_timing_function: None,
    aria_atomic: None,
    aria_auto_complete: None,
    aria_busy: None,
    aria_checked: None,
    aria_col_count: None,
    aria_col_index: None,
    aria_col_span: None,
    aria_controls: None,
    aria_current: None,
    min_width: "100%".into(),
    max_width: "100%".into(),
    width: "100%".into(),
    min_height: "100%".into(),
    max_height: "100%".into(),
    height: "100%".into()
};

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32
}