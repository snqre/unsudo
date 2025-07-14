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
        Redirect {
            to: "/",
            dioxus_layout::Row {
                style: format!(
                    r#"
                        font-family: alien skyline;
                        font-size: {}px;
                        font-weight: normal;
                        color: {};
                        {}
                    "#,
                    web::golden_unit::from(3),
                    web::color::SILVER,
                    props.style.unwrap_or_default()
                ),
                "unSUDO"
            }
        }
    )
}