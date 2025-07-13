use super::*;

#[component]
pub fn ColFill(props: CommonProps) -> Element {
    rsx!(
        Col {
            class: props.class,
            style: format!(
                r#"
                    {}
                    {}
                "#,
                stylesheet::FILL,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}