use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Props)]
pub struct FailureModalProps {
    pub children: Option<Element>
}

#[component]
pub fn FailureModal(props: FailureModalProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: start;
                gap: 20px;
            "#,
            div {
                style: r#"
                    width: 20px;
                    aspect-ratio: 1 / 1;
                    background-image: url({asset!("asset/icon/error.svg")});
                    background-size: contain;
                    background-repeat: no-repeat;
                    animation: faulty-neon 10s ease-in infinite;
                "#
            }
            div {
                style: r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: space-between;
                    align-items: center;
                    min-width: 400px;
                    max-width: 400px;
                    min-height: 50px;
                    background: {color::CARBON};
                    color: {color::SILVER};
                    border-color: {color::IMPERIAL_RED};
                    border-width: 1px;
                    border-style: solid;
                    border-radius: 2px;
                    box-shadow: 0 0 12px 1px {color::IMPERIAL_RED};
                    animation: faulty-neon 1s ease-in;
                    animation-delay: 0s;
                    animation-direction: normal;
                    animation-fill-mode: both;
                "#,
                horizontal_stripe::HorizontalStripe {
                    w: "100%",
                    h: "5px",
                    color: "{color::IMPERIAL_RED}"
                }
                div {
                    style: r#"
                        display: flex;
                        flex-direction: column;
                        justify-content: start;
                        align-items: center;
                        padding: 20px;
                        gap: 20px;
                    "#,
                    { props.children }
                }
                horizontal_stripe::HorizontalStripe {
                    w: "100%",
                    h: "5px",
                    color: "{color::IMPERIAL_RED}"
                }
            }
        }
    }
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct HeaderProps {
    pub children: Option<Element>
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
        div {
            style: r#"
                font-family: alien skyline;
                font-size: 3em;
                color: {color::IMPERIAL_RED};
            "#,
            { props.children }
        }
    }
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct MessageProps {
    pub children: Option<Element>
}

#[component]
pub fn Message(props: MessageProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                width: 100%;
                font-family: br cobane;
                font-size: 1em;
                color: {color::SILVER};
            "#,
            { props.children }
        }
    }
}