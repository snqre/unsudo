use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Props {
    pub w: String,
    pub h: String,
    pub color: String
}

#[component]
pub fn HorizontalStripe(props: Props) -> Element {
    rsx! {
        div {
            style: format!(r#"
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
                animation: move-stripe 60s linear infinite;
            "#, 
                props.w, 
                props.h,
                props.color,
                props.color
            )
        }
    }
}