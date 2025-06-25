use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct HeroCaseProps {
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn HeroCase(props: HeroCaseProps) -> Element {
    rsx! {
        div {
            div {
                style: r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    min-width: 100%;
                    max-width: 1440px;
                    min-height: 10px;
                    border-radius: 2px;
                    border-style: solid;
                    border-color: {color::CARBON};
                    {props.style.to_owned().unwrap_or_default()}
                "#,
                { props.children }
            }
            decor::HStripe {
                w: "100%",
                h: "5px"
            }
        }
    }
}