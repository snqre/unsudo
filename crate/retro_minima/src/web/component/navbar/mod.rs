use super::*;

leak!(
    drop_down_container
    logo
);

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RootProps {
    pub left: Option<Element>,
    pub right: Option<Element>,
    pub children: Option<Element>
}

#[component]
pub fn Root(props: RootProps) -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                align-items: center;
                min-width: 100%;
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