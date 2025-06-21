use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct FormProps {
    style: Option<String>,
    children: Option<Element>
}

#[component]
pub fn Form(props: FormProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: start;
                align-items: center;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            { props.children }
        }
    }
}


#[component]
pub fn Input() -> Element {
    rsx! {
        input {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                min-width: 100%;
                
            "#,
            "Hello World"
        }
    }
}

#[component]
pub fn Button() -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                gap: 10px;
            "#,
            button {
                style: r#"
                    all: unset;
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    min-width: 200px;
                    color: {color::CARBON};
                    background: linear-gradient(to left, {color::STEEL}, {color::SILVER});
                    cursor: pointer;
                    transition: 1s linear;
                    border-radius: 0.8px;
                    padding: 4px;
                    position: relative;
                "#,
                div {
                    style: r#"
                        min-width: 1px;
                        background: {color::interpolate((color::OBSIDIAN, color::SILVER), 0.05f32)};
                        border-radius: 0.8px;
                        color: transparent;
                        padding: 4px;
                        position: absolute;
                    "#
                }
                
                "Hello World"
            }
        }
    }
}