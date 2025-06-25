use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct SimpleButtonProps {
    pub on_click: Option<EventHandler<Event<MouseData>>>,
    pub on_mouse_enter: Option<EventHandler<Event<MouseData>>>,
    pub on_mouse_leave: Option<EventHandler<Event<MouseData>>>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn SimpleButton(props: SimpleButtonProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
            "#,
            button {
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
                style: r#"
                    all: unset;
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    font-family: br cobane;
                    font-size: 1em;
                    font-weight: normal;
                    cursor: pointer;
                    color: {color::SILVER};
                    {props.style.to_owned().unwrap_or_default()}
                "#,
                { props.children }
            }
            decor::Icon { size: "15px", url: asset!("asset/icon/arrow_tr.svg") }
        }
    }
}