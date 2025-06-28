use super::*;

bundle!(
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
fn CommonStructure(props: CommonStructureProps) -> Element {
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
    }
}