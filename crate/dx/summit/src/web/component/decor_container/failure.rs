use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct FailureProps {

    pub children: Option<Element>
}

#[component]
pub fn Failure(props: FailureProps) -> Element {
    rsx! {
        CommonStructure {
            icon: rsx! { div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    gap: 20px; 
                "#,
                div {
                    style: r#"
                        width: 20px;
                        aspect-ratio: 1 / 1;
                        background-image: url({asset!("asset/icon/failure.svg")});
                        background-size: contain;
                        background-repeat: no-repeat;
                        animation: faulty-neon 10s ease-in infinite;
                    "#
                }
                typo::Label { "Cache "}
            } },
            style: r#"
                border-color: {color::IMPERIAL_RED};
                box-shadow-color: {color::IMPERIAL_RED};
            "#,
            color: color::IMPERIAL_RED.to_owned(),
            { props.children }
        }
    }
}