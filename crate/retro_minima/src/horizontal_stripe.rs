use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Props {
    pub w: Option<String>,
    pub h: Option<String>,
    pub color: Option<String>,
    pub style: Option<String>,
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
                {}
            "#, 
                props.w.to_owned().unwrap_or_default(), 
                props.h.to_owned().unwrap_or_default(),
                props.color.to_owned().unwrap_or_default(),
                props.color.to_owned().unwrap_or_default(),
                props.style.to_owned().unwrap_or_default()
            )
        }
    }
}