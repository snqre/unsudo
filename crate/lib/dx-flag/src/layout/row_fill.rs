use super::*;

#[component]
pub fn RowFill(props: CommonProps) -> Element {
    rsx!(
        Row {
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