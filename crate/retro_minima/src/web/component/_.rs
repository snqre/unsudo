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
                transition: 0.1s ease-out;
            "#,
            { props.children }
        }
    }
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RollingButtonProps {
    pub child_on_idle: Element,
    pub child_on_hover: Element
}

#[component]
pub fn RollingButton(props: RollingButtonProps) -> Element {
    let mut border_color: Signal<&str> = use_signal(|| color::CARBON);
    let idle_child_top_on_idle: &str = "0%";
    let idle_child_top_on_hover: &str = "-250%";
    let hover_child_top_on_idle: &str = "250%";
    let hover_child_top_on_hover: &str = "0%";
    let mut idle_child_top: Signal<&str> = use_signal(|| idle_child_top_on_idle);
    let mut hover_child_top: Signal<&str> = use_signal(|| hover_child_top_on_idle);

    rsx! {
        button {
            onmouseenter: move |_| {
                border_color.set(color::OFFICE_BLUE);
                idle_child_top.set(idle_child_top_on_hover);
                hover_child_top.set(hover_child_top_on_hover);
            },
            onmouseleave: move |_| {
                border_color.set(color::CARBON);
                idle_child_top.set(idle_child_top_on_idle);
                hover_child_top.set(hover_child_top_on_idle);
            },
            style: r#"
                all: unset;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                border-width: 1px;
                border-style: solid;
                border-color: {border_color()};
                border-radius: 2px;
                position: relative;
                width: 200px;
                aspect-ratio: 3 / 1;
                cursor: pointer;
                font-family: br cobane;
                font-size: 1em;
                font-weight: normal;
                overflow-x: hidden;
                overflow-y: hidden;
            "#,
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    position: absolute;
                    transform: translate(0, {idle_child_top()});
                    transition: 0.1s ease-in-out;
                "#,
                { props.child_on_idle }
            }
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    position: absolute;
                    transform: translate(0, {hover_child_top()});
                    transition: 0.1s ease-in-out;
                "#,
                { props.child_on_hover }
            }
        }
    }
}