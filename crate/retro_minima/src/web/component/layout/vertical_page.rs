use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct VerticalPageProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn VerticalPage(props: VerticalPageProps) -> Element {
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

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct SectionProps {
    pub navbar: Option<Element>,
    pub footer: Option<Element>,
    pub children: Option<Element>
}

#[component]
pub fn Section(props: SectionProps) -> Element {
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
            "#,
            { props.navbar }
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
            { props.footer }
        }
    }
}