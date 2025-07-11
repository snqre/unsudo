use super::*;

pub static PAGE_OVERLAY_Z_INDEX: u64 = u64::MAX / 2;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PageProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub overlay_attrs: Option<extendable::AttrsProps>,
    pub overlay_event: Option<extendable::EventProps>,
    pub overlay: Option<Element>,
    pub content_attrs: Option<extendable::AttrsProps>,
    pub content_event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Page(props: PageProps) -> Element {
    rsx! {
        Holder {
            attrs: props.attrs,
            event: props.event,
            Overlay {
                attrs: props.overlay_attrs,
                event: props.overlay_event,
                { props.overlay }
            }
            Content {
                class: props.content_class,
                style: props.content_style,
                { props.children }
            }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct HolderProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
fn Holder(props: HolderProps) -> Element {
    rsx!(
        Stack {
            attrs: props.attrs.unwrap_or_default().try_override(extendable::AttrsProps {
                min_width: "100%".into(),
                max_width: "100%".into(),
                width: "100%".into(),
                min_height: "100%".into(),
                max_height: "100%".into(),
                height: "100%".into(),
                flex: "1".into(),
                ..Default::default()
            }),
            event: props.event,
            { props.children }
        }
    )
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
struct OverlayProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
fn Overlay(props: OverlayProps) -> Element {
    rsx! {
        StackItem {
            z: PAGE_OVERLAY_Z_INDEX,
            attrs: props.attrs.unwrap_or_default().try_override(extendable::AttrsProps {
                min_width: "100%".into(),
                max_width: "100%".into(),
                width: "100%".into(),
                min_height: "100%".into(),
                max_height: "100%".into(),
                height: "100%".into(),
                flex: "1".into(),
                overflow: "hidden".into(),
                ..Default::default()
            }),
            event: props.event,
            { props.children }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct ContentProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
fn Content(props: ContentProps) -> Element {
    rsx! {
        StackItem {
            z: 0,
            attrs: props.attrs.unwrap_or_default().try_override(extendable::AttrsProps {
                style: r#"
                    
                    overflow-x: hidden;
                    overflow-y: auto;
                    scroll-snap-type: y mandatory;
                    scroll-behaviour: smooth;
                    {props.attrs.style.to_owned().unwrap_or_default()}
                "#.into(),
                min_width: "100%".into(),
                max_width: "100%".into(),
                width: "100%".into(),
                min_height: "100%".into(),
                max_height: "100%".into(),
                height: "100%".into(),
                flex: "1".into(),
                ..Default::default()
            }),

        }
    }
}