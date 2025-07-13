use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct HStripeProps {
    pub w: String,
    pub h: String,
    pub color: String,
    pub animation_duration_seconds: f64,
    pub attrs: Box<Option<extendable::AttrsProps>>,
    pub event: Box<Option<extendable::EventProps>>
}

#[component]
pub fn HStripe(props: HStripeProps) -> Element {
    rsx!(
        extendable::Node {
            attrs: props.attrs.with_style_before(&format!(
                r#"
                    width: {};
                    height: {};
                    background-image: repeating-linear-gradient(
                        45deg,
                        {},
                        {} 10px,
                        transparent 10px,
                        transparent 20px
                    );
                    background-repeat: repeat-x;
                    animation: move-stripe {}s linear infinite;
                "#,
                props.w,
                props.h,
                props.color,
                props.color,
                props.animation_duration_seconds
            )),
            event: props.event.unwrap_or_default()
        }
    )
}