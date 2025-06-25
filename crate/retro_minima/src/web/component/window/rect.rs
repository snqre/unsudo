use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RectProps {
    pub style: Option<String>,
    pub children: Option<Element>,
}

#[component]
pub fn Rect(props: RectProps) -> Element {
    rsx! {
        window_frame::Rect {
            style: r#"
                background: {color::JET};
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}