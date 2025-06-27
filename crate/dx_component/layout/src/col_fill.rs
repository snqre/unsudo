use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ColFillProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn ColFill(props: ColFillProps) -> Element {
    rsx! {
        Col {
            style: r#"
                {FILL}
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}