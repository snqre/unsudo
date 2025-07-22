use super::*;

#[component]
pub fn StackFill(props: CommonProps) -> Element {
    rsx!(
        Stack {
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