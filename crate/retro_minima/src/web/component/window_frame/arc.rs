use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ArcProps {
    pub children: Option<Element>
}

#[component]
pub fn Arc(props: ArcProps) -> Element {
    rsx! {
        Rect {
            style: r#"
                border-top-left-radius: 200px;
                border-top-right-radius: 200px;
                border-bottom-left-radius: 2px;
                border-bottom-right-radius: 2px;
            "#,
            { props.children }
        }
    }
}