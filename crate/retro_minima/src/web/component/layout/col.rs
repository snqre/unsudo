use super::*;



#[component]
pub fn Col() -> Element {
    rsx! {
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
            "#,
        }
    }
}