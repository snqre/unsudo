use super::*;

pub static PAGE_OVERLAY_Z: u64 = u64::MAX / 2;

#[derive(Props, Clone, PartialEq)]
pub struct PageProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub overlay_class: Option<String>,
    pub overlay_style: Option<String>,
    pub overlay: Option<Element>,
    pub content_class: Option<String>,
    pub content_style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Page(props: PageProps) -> Element {
    rsx! {
        Holder {
            class: props.class,
            style: props.style,
            Overlay {
                class: props.overlay_class,
                style: props.overlay_style,
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
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
fn Holder(props: HolderProps) -> Element {
    rsx! {
        Stack {
            class: props.class,
            style: format! {
                r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    height: 100%;
                    min-height: 100%;
                    max-height: 100%;
                    flex: 1;
                    {}
                "#,
                props.style.to_owned().unwrap_or_default()
            },
            { props.children }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct OverlayProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
fn Overlay(props: OverlayProps) -> Element {
    rsx! {
        StackItem {
            z: PAGE_OVERLAY_Z,
            class: props.class,
            style: format! {
                r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    height: 100%;
                    min-height: 100%;
                    max-height: 100%;
                    flex: 1;
                    overflow-x: hidden;
                    overflow-y: hidden;
                    {}
                "#,
                props.style.to_owned().unwrap_or_default()
            },
            { props.children }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct ContentProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
fn Content(props: ContentProps) -> Element {
    rsx! {
        StackItem {
            z: 0,
            class: props.class,
            style: format! {
                r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    height: 100%;
                    min-height: 100%;
                    max-height: 100%;
                    flex: 1;
                    overflow-x: hidden;
                    overflow-y: auto;
                    scroll-snap-type: y mandatory;
                    scroll-behaviour: smooth;
                    {}
                "#,
                props.style.to_owned().unwrap_or_default()
            },
        }
    }
}