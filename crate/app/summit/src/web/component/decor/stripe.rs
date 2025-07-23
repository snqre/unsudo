use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum StripeDirection {
    Vertical,
    Horizontal
}
impl Default for StripeDirection {
    fn default() -> Self {
        Self::Horizontal
    }
}

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct StripeProps {
    pub direction: Option<StripeDirection>,
    pub w: String,
    pub h: String,
    pub color: String,
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn Stripe(props: StripeProps) -> Element {
    rsx!(
        div {
            style: format!(
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
                    animation: move-stripe 30s linear infinite;
                    {}
                "#,
                props.w,
                props.h,
                props.color,
                props.color,
                props.style.unwrap_or_default()
            )
        }
    )
}