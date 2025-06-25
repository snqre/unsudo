use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct VerticalPageSectionProps {
    pub top: Option<Element>,
    pub top_style: Option<String>,
    pub bottom: Option<Element>,
    pub bottom_style: Option<String>,
    pub wrapper_style: Option<String>,
    pub children: Option<Element>,

}

#[component]
pub fn VerticalPageSection(props: VerticalPageSectionProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: space-between;
                align-items: center;
                width: 100vw;
                height: 100vh;
                overflow-x: hidden;
                overflow-y: hidden;
                scroll-snap-align: start;
                {props.wrapper_style.to_owned().unwrap_or_default()}
            "#,
            div {
                style: r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    min-width: 100%;
                    {props.top_style.to_owned().unwrap_or_default()}
                "#,
                { props.top }
            }
            div {
                style: r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: space-between;
                    align-items: center;
                    width: 100%;
                    height: 100%;
                    padding: 20px;
                "#,
                { props.children }
            }
            div {
                style: r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    min-width: 100%;
                    {props.bottom_style.to_owned().unwrap_or_default()}
                "#,
                { props.bottom }
            }
        }
    }
}