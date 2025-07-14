use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ClickProps {
    pub on_click: Option<EventHandler<Event<MouseData>>>,
    pub children: Option<Element>
}

#[component]
pub fn Click(props: ClickProps) -> Element {
    rsx!(
        div {
            style: r#"
                display: contents;
                cursor: pointer;
            "#,
            onclick: move |data| if let Some(on_click) = props.on_click {
                on_click(data)
            },
            { props.children }
        }
    )
}