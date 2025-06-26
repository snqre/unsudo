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
        layout::Row {
            style: r#"
                gap: {web::sequence(1u16)}px;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}