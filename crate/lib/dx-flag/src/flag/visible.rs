use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct VisibleProps {
    pub on_visible: Option<EventHandler<Event<VisibleData>>>,
    pub children: Option<Element>
}

#[component]
pub fn Visible(props: VisibleProps) -> Element {
    rsx!(
        div {
            style: "display: contents;",
            onvisible: move |data| if let Some(on_visible) = props.on_visible {
                on_visible(data)
            },
            { props.children }
        }
    )
}