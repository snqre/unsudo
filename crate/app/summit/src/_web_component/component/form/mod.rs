use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RootProps {
    class: Option<String>,
    style: Option<String>,
    children: Option<Element>
}

#[component]
pub fn Root(props: RootProps) -> Element {
    rsx!(
        dioxus_layout::Col {
            style: format!(
                r#"
                    border-color: {};
                    border-width: 1px;
                    border-style: solid;
                    border-radius: 2px;
                    padding: 16px;
                    {}
                "#,
                color::OBSIDIAN,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}



#[derive(Clone)]
#[derive(PartialEq)]
enum InputState {
    Idle,
    HoldingInput
}

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct InputProps {
    pub label: String,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let mut state: Signal<InputState> = use_signal(|| InputState::Idle);
    let mut input: Signal<String> = use_signal(|| "".to_owned());

    let opacity: f64 = if state() == InputState::Idle {
        0.0
    } else {
        1.0
    };

    let placeholder: String = if state() == InputState::Idle {
        props.label.to_owned()
    } else {
        "".to_owned()
    };

    rsx!(
        layout::Col {
            class: props.class,
            style: format!(
                r#"
                    justify-content: start;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            if !input().is_empty() {
                layout::Row {
                    style: format!(
                        r#"
                            width: 100%;
                            opacity: {};
                        "#,
                        opacity
                    ),
                    { props.label }
                }
            }
            input {
                placeholder: placeholder,
                style: format!(
                    r#"
                        all: unset;
                        display: flex;
                        flex-direction: row;
                        justify-content: center;
                        align-items: center;
                        min-width: 100%;
                    "#
                ),
                oninput: move |e| {
                    let new_input: String = e.value();

                    if new_input.is_empty() {
                        state.set(InputState::Idle);
                    } else {
                        state.set(InputState::HoldingInput);
                    }

                    input.set(new_input);
                }
            }
        }
    )
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

