use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RowProps {
    pub attrs: Option<div::AttrsProps>,
    pub event: Option<div::EventProps>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Row(props: RowProps) -> Element {
    rsx! {
        div::Div {
            attrs: props.attrs.unwrap_or_default().merge(div::AttrsProps {
                style: Some(r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    {props.style.to_owned().unwrap_or_default()}          
                "#),
                ..Default::default()
            }),
            event: props.event.unwrap_or_default(),
            { props.children }
        }
    }
}