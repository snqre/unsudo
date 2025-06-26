use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct TopologicalPageSectionProps {
    pub x: u32,
    pub y: u32,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn TopologicalPageSection(props: TopologicalPageSectionProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                width: 100vw;
                height: 100vh;
                font-size: 2em;
                grid-column: {props.x + 1};
                grid-row: {props.y + 1};
            "#,
            div {
                style: r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    width: 100vw;
                    height: 100vh;
                    {props.style.to_owned().unwrap_or_default()}
                "#,
                { props.children }
            }
        }
    }
}