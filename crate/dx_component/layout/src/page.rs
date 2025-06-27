use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PageProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Page(props: PageProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                width: 100%;
                height: 100%;
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