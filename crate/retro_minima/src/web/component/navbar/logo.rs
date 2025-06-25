use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct LogoProps {}

#[component]
pub fn Logo(props: LogoProps) -> Element {
    rsx! {
        div { 
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                min-width: 100px;
            "#,
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    font-family: alien skyline;
                    font-size: 3em;
                    font-weight: normal;
                "#,
                "unSUDO"
            }
        }
    }
}