#![allow(unused)]

use super::*;

#[derive(Clone, PartialEq)]
#[repr(u8)]
pub enum HeadingKind {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6
}

#[derive(Props, Clone, PartialEq)]
pub struct HeadingProps {
    pub kind: Option<HeadingKind>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Heading(props: HeadingProps) -> Element {
    let (_, font_size) = match props.kind {
        Some(HeadingKind::H1) => ("h1", "2.25rem"),
        Some(HeadingKind::H2) => ("h2", "1.875rem"),
        Some(HeadingKind::H3) => ("h3", "1.5rem"),
        Some(HeadingKind::H4) => ("h4", "1.25rem"),
        Some(HeadingKind::H5) => ("h5", "1rem"),
        Some(HeadingKind::H6) => ("h6", "0.875rem"),
        None => ("none", "")
    };
    rsx! {
        layout::Row {
            div {
                style: r#"
                    font-family: br cobane;
                    font-size: {font_size};
                    color: {color::SILVER};
                    {props.style.to_owned().unwrap_or_default()}
                "#,
                { props.children }
            }
        }
    }
}