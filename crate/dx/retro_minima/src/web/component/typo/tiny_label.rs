use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct TinyLabelProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn TinyLabel(props: TinyLabelProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                font-family: br cobane;
                font-size: 0.5em;
                font-weight: normal;
                color: {color::SILVER};
                {props.style.to_owned().unwrap_or_default()}
            "#
        }
    }
}