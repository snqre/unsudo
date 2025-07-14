use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct WarningProps {
    pub children: Option<Element>
}

#[component]
pub fn Warning(props: WarningProps) -> Element {
    rsx! {
        Scaffold {
            icon: rsx! { div {
                style: r#"
                    width: 20px;
                    aspect-ratio: 1 / 1;
                    background-image: url({asset!("asset/icon/warning.svg")});
                    background-size: contain;
                    background-repeat: no-repeat;
                    animation: faulty-neon 10s ease-in infinite;
                "#
            } },
            style: r#"
                border-color: {color::HONEY};
                box-shadow-color: {color::HONEY};
            "#,
            color: color::HONEY.to_owned(),
            { props.children }
        }
    }
}