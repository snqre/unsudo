use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Props {
    pub size: String,
    pub url: Asset
}

#[component]
pub fn Icon(props: Props) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                background-image: url({props.url});
                background-size: contain;
                background-position: center;
                background-repeat: no-repeat;
                cursor: pointer;
                width: {props.size};
                aspect-ratio: 1 / 1;
            "#
        }
    }
}