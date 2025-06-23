use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Props {
    pub children: Option<Element>
}

#[component]
pub fn Button(props: Props) -> Element {
    let mut border_color: Signal<&str> = use_signal(|| color::CARBON);
    rsx! {
        button {
            onmouseenter: move |_| border_color.set(color::JET),
            onmouseleave: move |_| border_color.set(color::CARBON),
            style: r#"
                all: unset;
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                border-width: 1px;
                border-style: solid;
                border-color: {border_color()};
                border-radius: 2px;
                padding: 20px;
                cursor: pointer;
                font-family: br cobane;
                font-size: 1em;
                font-weight: normal;
            "#,
            { props.children }
        }
    }
}