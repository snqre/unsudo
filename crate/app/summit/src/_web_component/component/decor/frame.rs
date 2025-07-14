use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct FrameProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}
#[component]
pub fn Frame(props: FrameProps) -> Element {
    rsx!(
        layout::Col {
            style: format!(
                r#"
                    border-radius: 2px;
                    border-style: solid;
                    border-color: {};
                    {}
                "#,
                color::CARBON,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}