use super::*;

expose!(
    failure
    success
    warning
);

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
struct CommonStructureProps {
    pub icon: Option<Element>,
    pub color: Option<String>,
    pub style: Option<String>,
    pub stripe_style: Option<String>,
    pub children: Option<Element>
}

#[component]
fn Scaffold(props: CommonStructureProps) -> Element {
    rsx!(
        layout::Col {
            attrs: None.with_style(r#"
                align-items: start;
                gap: 20px;
            "#),
            layout::Row {
                attrs: None.with_style(r#"
                    justify-content: start;
                    width: 100%;
                "#),
                { props.icon }
            }
            layout::Col {
                attrs: None.with_style(&format!(
                    r#"
                        justify-content: space-between;
                        min-width: 400px;
                        max-width: 400px;
                        background: {};
                        border-color: {};
                        border-width: 1px;
                        border-radius: 2px;
                        animation: faulty-neon 1s ease-in;
                    "#,
                    color::CARBON,
                    color::SILVER
                )),

            }
        }



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
                decor::HStripe {
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
                decor::HStripe {
                    style: r#"
                        width: 100%;
                        height: 5px;
                        {props.stripe_style.to_owned().unwrap_or_default()}
                    "#,
                    color: props.color.to_owned()
                }
            }
        }
    )
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct CommonProps {
    pub icon_attrs: Option<extendable::AttrsProps>,
    pub icon_event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Success(props: CommonProps) -> Element {
    rsx! {
        Scaffold {
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

    rsx!(
        Scaffold {
            icon: rsx!(
                decor::Icon {
                    url: Url::Internal(asset!("asset/icon/success.svg")),
                    w: "20px",
                    attrs: props.icon_attrs,
                    event: props.icon_event.on(extendable::EventProps {
                        on_click: Some(),
                        ..Default::default()
                    })
                }
            )
            
        }
    )
}