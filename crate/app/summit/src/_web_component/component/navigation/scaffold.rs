use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ScaffoldProps {
    pub left: Option<Element>,
    pub right: Option<Element>,
    pub children: Option<Element>
}

#[component]
pub fn Scaffold(props: ScaffoldProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                align-items: center;
                width: 100%;
                min-width: 0%;
                max-width: {web::sequence(11)}px;
                padding: 20px;
            "#,
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: start;
                    align-items: center;
                "#,
                { props.left }
            }
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                "#,
                { props.children }
            }
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: end;
                    align-items: center;
                "#,
                { props.right }
            }
        }
    }
}