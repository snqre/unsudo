use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RectProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Rect(props: RectProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                border-radius: 2px;
                border-style: solid;
                border-color: {color::CARBON};
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}