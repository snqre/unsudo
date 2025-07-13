use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct LogoProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>
}

#[component]
pub fn Logo(props: LogoProps) -> Element {
    rsx!(
        Link {
            style: r#"
                all: unset;
                display: contents;
            "#,
            to: "/",
            layout::Row {
                attrs: props.attrs.with_style_before(&format!(
                    r#"
                        font-family: alien skyline;
                        font-size: {}px;
                        font-weight: normal;
                        color: {};
                        cursor: pointer;
                    "#,
                    u(3),
                    color::SILVER
                )),
                event: props.event,
                "unSUDO"
            }
        }
    )
}