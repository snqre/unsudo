use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct SocialIconGroupProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn SocialIconGroup(props: SocialIconGroupProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                gap: 10px;
            "#,
            { props.children }
        }
    }
}