use super::*;

leak!(
    auto_col
    auto_grid
    auto_row
    col
    grid_item
    grid
    horizontal_slider
    parallax
    row
    stack_item
    stack
    topological_page_section
    topological_page
    vertical_page_section
    vertical_page
);

#[derive(Clone, PartialEq)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32
}

impl From<(u32, u32)> for Coordinate {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}