use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub r#type: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>,
    pub on_click: Option<EventHandler<Event<MouseData>>>,
    pub on_mouse_enter: Option<EventHandler<Event<MouseData>>>,
    pub on_mouse_leave: Option<EventHandler<Event<MouseData>>>
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            r#type: props.r#type,
            style: r#"
                all: unset;
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                font-family: br cobane;
                font-size: {web::sequence(2)};
                font-weight: bold;
                color: {color::SILVER};
                cursor: pointer;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            onclick: move |e| {
                if let Some(on_click) = props.on_click {
                    on_click(e);
                }
            },
            onmouseenter: move |e| {
                if let Some(on_mouse_enter) = props.on_mouse_enter {
                    on_mouse_enter(e);
                }
            },
            onmouseleave: move |e| {
                if let Some(on_mouse_leave) = props.on_mouse_leave {
                    on_mouse_leave(e);
                }
            },
            { props.children }
        }
    }
}