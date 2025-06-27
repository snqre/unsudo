use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ColFProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn ColF(props: ColFProps) -> Element {
    rsx! {
        Col {
            style: r#"
                width: 100%;
                min-width: 100%;
                max-width: 100%;
                height: 100%;
                min-height: 100%;
                max-height: 100%;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}