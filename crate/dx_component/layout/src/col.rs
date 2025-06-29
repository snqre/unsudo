use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ColProps {
    pub on_mouse_enter: Option<EventHandler<Event<MouseData>>>,
    pub on_mouse_leave: Option<EventHandler<Event<MouseData>>>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        div {
            onmouseenter: move |e| if let Some(on_mouse_enter) = props.on_mouse_enter {
                on_mouse_enter(e)
            },
            onmouseleave: move |e| if let Some(on_mouse_leave) = props.on_mouse_leave {
                on_mouse_leave(e)
            },
            class: props.class,
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