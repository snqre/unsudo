use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ColProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}