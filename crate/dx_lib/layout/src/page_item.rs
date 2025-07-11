use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PageItemProps {
    pub top: Option<Element>,
    pub top_style: Option<String>,
    pub bottom: Option<Element>,
    pub bottom_style: Option<String>,
    pub background: Option<Element>,
    pub background_style: Option<String>,
    pub wrapper_style: Option<String>,
    pub children: Option<Element>,

}

#[component]
pub fn PageItem(props: PageItemProps) -> Element {
    rsx! {
        Stack {
            style: r#"
                width: 100vw;
                min-width: 100vw;
                max-width: 100vw;
                height: 100vh;
                min-height: 100vh;
                max-height: 100vh;
            "#,
            StackItem {
                z: 0,
                style: r#"
                    top: 0%;
                    left: 0%;
                    width: 100vw;
                    min-width: 100vw;
                    max-width: 100vw;
                    height: 100vh;
                    min-height: 100vh;
                    max-height: 100vh;
                    {props.background_style.to_owned().unwrap_or_default()}
                "#,
                { props.background }
            }
            StackItem {
                z: 1,
                style: r#"
                    left: 0%;
                    top: 0%;
                    justify-content: space-between;
                    width: 100vw;
                    min-width: 100vw;
                    max-width: 100vw;
                    height: 100vh;
                    min-height: 100vh;
                    max-height: 100vh;
                    scroll-snap-align: start;
                    {props.wrapper_style.to_owned().unwrap_or_default()}
                "#,
                if let Some(top) = props.top {
                    div {
                        style: r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: center;
                            min-width: 100%;
                            {props.top_style.to_owned().unwrap_or_default()}
                        "#,
                        { top }
                    }
                }
                if let Some(children) = props.children {
                    div {
                        style: r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: space-between;
                            align-items: center;
                            width: 100%;
                            height: 100%;
                        "#,
                        { children }
                    }
                }
                if let Some(bottom) = props.bottom {
                    div {
                        style: r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: center;
                            min-width: 100%;
                            {props.bottom_style.to_owned().unwrap_or_default()}
                        "#,
                        { bottom }
                    }
                }
            }
        }
    }
}