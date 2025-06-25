use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct LinkedButtonProps {
    pub url: String,
    pub leads_to_external_url: Option<bool>, 
    pub r#type: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>,
    pub on_click: Option<EventHandler<Event<MouseData>>>,
    pub on_mouse_enter: Option<EventHandler<Event<MouseData>>>,
    pub on_mouse_leave: Option<EventHandler<Event<MouseData>>>
}

#[component]
pub fn LinkedButton(props: LinkedButtonProps) -> Element {
    rsx! {
        Link {
            style: r#"
                all: unset;
                display: contents;
                cursor: pointer;
            "#,
            to: props.url,
            layout::Row {
                style: r#"
                    {props.style.to_owned().unwrap_or_default()}
                "#,
                Button {
                    style: r#"
                        cursor: inherit;
                    "#,
                    r#type: props.r#type,
                    on_click: props.on_click,
                    on_mouse_enter: props.on_mouse_enter,
                    on_mouse_leave: props.on_mouse_leave,
                    { props.children }
                }
                if let Some(true) = props.leads_to_external_url {
                    decor::Icon { size: "{web::sequence(2)}px", url: asset!("asset/icon/arrow_tr.svg") }
                }
            }
        }
    }
}