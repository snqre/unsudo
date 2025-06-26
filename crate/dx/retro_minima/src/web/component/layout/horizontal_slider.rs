use super::*;
use std::time::Duration;

#[derive(Props, Clone, PartialEq)]
pub struct HorizontalSliderProps {

}

#[component]
pub fn HorizontalSlider(props: HorizontalSliderProps) -> Element {
    rsx! {
        div {
            style: r#"
                width: 100%;
                height: 100%;
            "#,
            Slider {
                from_x: "100px",
                from_y: "0px",
                to_x: "1000px",
                to_y: "50px",
                duration: 10000.0,
                delay: 30.0
            }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct SliderProps {
    pub from_x: String,
    pub from_y: String,
    pub to_x: String,
    pub to_y: String,
    pub duration: f64,
    pub delay: f64
}

#[component]
fn Slider(props: SliderProps) -> Element {
    let x: Signal<_> = use_signal(|| props.from_x);
    let y: Signal<_> = use_signal(|| props.from_y);

    use_effect(move || {
        let mut x: Signal<_> = x;
        let mut y: Signal<_> = y;
        let to_x: String = props.to_x.to_owned();
        let to_y: String = props.to_y.to_owned();
        x.set(to_x);
        y.set(to_y);
    });

    rsx! {
        Row {
            style: r#"
                width: 100px;
                aspect-ratio: 1 / 1;
                background: green;
                transform: translate({x()}, {y()});
                transition-property: all;
                transition-duration: 10s;
                transition-timing-function: linear;
                transition-delay: 0s;
            "#
        }
    }
}