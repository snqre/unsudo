use dioxus::prelude::*;

modwire::expose!(
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
    spacial_page_item
    spacial_page
);

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct CommonProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

mod stylesheet {
    pub static ABS_POS_RESET: &str = r#"
        top: 0%;
        left: 0%;
    "#;

    pub static FILL_VIEW: &str = r#"
        min-width: 100vw;
        max-width: 100vw;
        width: 100vw;
        min-height: 100vh;
        max-height: 100vh;
        height: 100vh;
    "#;

    pub static FILL: &str = r#"
        width: 100%;
        height: 100%;
        flex: 1;
    "#;
}