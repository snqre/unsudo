use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PageProps {
    pub overlay: Option<Element>,
    pub overlay_style: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Page(props: PageProps) -> Element {
    rsx! {
        Stack {
            style: r#"
                width: 100%;
                min-width: 100%;
                max-width: 100%;
                height: 100%;
                min-height: 100%;
                max-height: 100%;
                flex: 1;
            "#,
            StackItem {
                z: u64::MAX,
                style: r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    height: 100%;
                    min-height: 100%;
                    max-height: 100%;
                    flex: 1;
                    overflow-x: hidden;
                    overflow-y: hidden;
                    {props.overlay_style.to_owned().unwrap_or_default()}
                "#,
                { props.overlay }
            }
            StackItem {
                z: 0,
                style: r#"
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
                    {props.style.to_owned().unwrap_or_default()}
                "#,
                { props.children }
            }
        }
    }
}