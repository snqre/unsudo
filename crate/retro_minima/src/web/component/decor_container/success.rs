use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct SuccessProps {
    pub children: Option<Element>
}

#[component]
pub fn Success(props: SuccessProps) -> Element {
    rsx! {
        CommonStructure {
            icon: rsx! { div {
                style: r#"
                    width: 20px;
                    aspect-ratio: 1 / 1;
                    background-image: url({asset!("asset/icon/success.svg")});
                    background-size: contain;
                    background-repeat: no-repeat;
                    animation: faulty-neon 10s ease-in infinite;
                "#
            } },
            style: r#"
                border-color: {color::SPRING};
                box-shadow-color: {color::SPRING};
            "#,
            color: color::SPRING.to_owned(),
            { props.children }
        }
    }
}