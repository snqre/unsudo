use super::*;



#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Props {
    pub start: f32,
    pub end: f32,
    pub duration: Duration,
    pub easing: fn(f32, f32, f32, f32) -> f32
}

#[component]
pub fn Counter(props: Props) -> Element {
    let mut count = use_motion(props.start);

    use_effect(move || {
        count.animate_to(props.end, AnimationConfig::new(AnimationMode::Tween(Tween {
            duration: props.duration,
            easing: props.easing
        })))
    });

    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
            "#,
            { format!("{:.2}", count.get_value()) }
        }
    }
}