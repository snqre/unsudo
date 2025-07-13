use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RootProps {
    attrs: Option<dioxus_extendable::AttrsProps>,
    event: Option<dioxus_extendable::EventProps>,
    children: Option<Element>
}

#[component]
pub fn Root(props: RootProps) -> Element {
    rsx!(
        dioxus_layout::Col {
            attrs: props.attrs.with_style_before(&format!(
                r#"
                    border-color: {};
                    border-width: 1px;
                    border-style: solid;
                    border-radius: 2px;
                    padding: 16px;
                "#,
                color::OBSIDIAN
            )),
            event: props.event,
            { props.children }
        }
    )
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


#[component]
pub fn DropDown() -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                flex: 1;
                min-width: 100%;
                min-height: auto;
                color: white;
                cursor: pointer;
                color: {color::SILVER};
                padding: 1px;
                border-radius: 2px;
                font-size: 0.25em;
                font-family: Stray;
                position: relative;
                gap: 16px
            "#,
            div { 
                style: r#"
                    font-family: Alien Skyline; 
                    font-size: 32em
                "#, 
                "unSUDO"
            }
            div {
                style: r#"
                    font-family: BR Cobane;
                    font-size: 8em;
                "#,
                "Autonomous Decentralized Protocols"
            }
        }
    }
}

