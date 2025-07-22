use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct SpacialPageItemProps {
    pub x: u32,
    pub y: u32,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn SpacialPageItem(props: SpacialPageItemProps) -> Element {
    rsx!(
        Col {
            class: props.class,
            style: format!(
                r#"
                    width: 100vw;
                    height: 100vh;
                    grid-column: {};
                    grid-row: {};
                    {}
                "#,
                props.x + 1,
                props.y + 1,
                props.style.unwrap_or_default()
            ),
            Col {
                style: r#"
                    width: 100vw;
                    height: 100vh;
                "#,
                { props.children }
            }
        }
    )
}