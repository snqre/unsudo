use super::*;

pub static PAGE_OVERLAY_Z_INDEX: u64 = u64::MAX / 2;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PageProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub overlay: Option<Element>,
    pub content: Option<Element>
}

#[component]
pub fn Page(props: PageProps) -> Element {
    rsx!(
        Holder {
            class: props.class,
            style: props.style,
            Overlay { { props.overlay } }
            Content { { props.content } }
        }
    )
}

#[component]
fn Holder(props: CommonProps) -> Element {
    rsx!(
        Stack {
            class: props.class,
            style: format!(
                r#"
                    flex: 1;
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

#[component]
fn Overlay(props: CommonProps) -> Element {
    rsx!(
        StackItem {
            z: PAGE_OVERLAY_Z_INDEX,
            class: props.class,
            style: format!(
                r#"
                    flex: 1;
                    overflow-x: hidden;
                    overflow-y: hidden;
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

#[component]
fn Content(props: CommonProps) -> Element {
    rsx!(
        StackItem {
            z: 0,
            class: props.class,
            style: format!(
                r#"
                    flex: 1;
                    overflow-x: hidden;
                    overflow-y: auto;
                    scroll-snap-type: y mandatory;
                    scroll-behaviour: smooth;
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