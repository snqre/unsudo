use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct StackProps {
    pub on_mouse_enter: Option<EventHandler<Event<MouseData>>>,
    pub on_mouse_leave: Option<EventHandler<Event<MouseData>>>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Stack(props: StackProps) -> Element {
    rsx! {
        Col {
            on_mouse_enter: props.on_mouse_enter,
            on_mouse_leave: props.on_mouse_leave,
            style: r#"
                position: relative;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}