use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct LogoProps {
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn Logo(props: LogoProps) -> Element {
    rsx!(
        util::To {
            to: "/",
            layout::Row {
                style: format!(
                    r#"
                        font-family: alien skyline;
                        font-size: {}px;
                        font-weight: normal;
                        color: {};
                        {}
                    "#,
                    u(3),
                    color::SILVER,
                    props.style.unwrap_or_default()
                ),
                "unSUDO"
            }
        }
    )
}