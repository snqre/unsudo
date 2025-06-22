use super::*;

pub mod warning {
    use super::*;

    #[derive(Props)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    pub struct WarningProps {
        pub children: Option<Element>
    }

    #[component]
    pub fn Warning(props: WarningProps) -> Element {
        rsx! {
            common::Card {
                icon: rsx! { div {
                    style: r#"
                        width: 20px;
                        aspect-ratio: 1 / 1;
                        background-image: url({asset!("asset/icon/warning.svg")});
                        background-size: contain;
                        background-repeat: no-repeat;
                        animation: faulty-neon 10s ease-in infinite;
                    "#
                } },
                style: r#"
                    border-color: {color::HONEY};
                    box-shadow-color: {color::HONEY};
                "#,
                color: color::HONEY.to_owned(),
                { props.children }
            }
        }
    }
}

pub mod success {
    use super::*;

    #[derive(Props)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    pub struct SuccessProps {
        pub children: Option<Element>
    }

    #[component]
    pub fn Success(props: SuccessProps) -> Element {
        rsx! {
            common::Card {
                icon: rsx! { div {
                    style: r#"
                        width: 20px;
                        aspect-ratio: 1 / 1;
                        background-image: url({asset!("asset/icon/success.svg")});
                        background-size: contain;
                        background-repeat: no-repeat;
                        animation: faulty-neon 10s ease-in infinite;
                    "#
                } },
                style: r#"
                    border-color: {color::SPRING};
                    box-shadow-color: {color::SPRING};
                "#,
                color: color::SPRING.to_owned(),
                { props.children }
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
                    padding: 20px;
                    color: {color::SPRING};
                "#,
                { props.children }
            }
        }
    }
}

pub mod failure {
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
            common::Card {
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
                    Header { "Cache Incompatible" }
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
                    font-family: br cobane;
                    font-size: 0.5em;
                    color: {color::SILVER};
                "#,
                { props.children }
            }
        }
    }

    #[component]
    pub fn Divisor() -> Element {
        rsx! {
            div {
                style: r#"
                    height: 1px;
                    margin-top: 20px;
                    margin-bottom: 20px;
                "#
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
                    font-size: 3em;
                    color: {color::SILVER};
                    padding: 20px;
                "#,
                { props.children }
            }
        }
    }

    #[component]
    pub fn Button() -> Element {
        let mut background_color: Signal<&str> = use_signal(||  "transparent");
        let mut color: Signal<&str> = use_signal(|| color::IMPERIAL_RED);
        rsx! {
            button {
                onmouseenter: move |_| {
                    background_color.set(color::IMPERIAL_RED);
                    color.set(color::CARBON);
                },
                onmouseleave: move |_| {
                    background_color.set("transparent");
                    color.set(color::IMPERIAL_RED);
                },
                style: r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    width: 150px;
                    height: 50px;
                    background: {background_color()};
                    border-width: 1px;
                    border-style: solid;
                    border-color: {color::IMPERIAL_RED};
                    border-radius: 2px;
                    border-top-left-radius: 10px;
                    border-bottom-right-radius: 10px;
                    color: {color()};
                    cursor: pointer;
                    font-family: br cobane;
                    font-size: 1em;
                    transition: 0s linear;
                "#,
                "Cancel"
            }
        }
    }
}

mod common {
    use super::*;

    #[derive(Props)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    pub struct Props {
        pub icon: Option<Element>,
        pub color: Option<String>,
        pub style: Option<String>,
        pub stripe_style: Option<String>,
        pub children: Option<Element>
    }

    #[component]
    pub fn Card(props: Props) -> Element {
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
                        display: flex;
                        flex-direction: row;
                        justify-content: start;
                        align-items: center;
                        width: 100%;
                    "#,
                    { props.icon }
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
                        border-color: {color::SILVER};
                        border-width: 1px;
                        border-style: solid;
                        border-radius: 2px;
                        animation: faulty-neon 1s ease-in;
                        {props.style.to_owned().unwrap_or_default()}
                    "#,
                    horizontal_stripe::HorizontalStripe {
                        style: r#"
                            width: 100%;
                            height: 5px;
                            {props.stripe_style.to_owned().unwrap_or_default()}
                        "#,
                        color: props.color.to_owned()
                    }
                    div {
                        style: r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: start;
                            align-items: center;
                            padding: 10px;
                        "#,
                        { props.children }
                    }
                    horizontal_stripe::HorizontalStripe {
                        style: r#"
                            width: 100%;
                            height: 5px;
                            {props.stripe_style.to_owned().unwrap_or_default()}
                        "#,
                        color: props.color.to_owned()
                    }
                }
            }
        }
    }
}