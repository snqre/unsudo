

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ButtonProps {
    pub children: Option<Element>
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx!(
        div {
            style: format!(
                r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    font-size: {}px;
                    font-family: br cobane;
                    font-weight: bold;
                    color: {};
                    cursor: pointer;
                    
                "#,
                golden_ratio::from(2),
                color::SILVER
            ),
            { props.children }
        }
    )
}