use super::*;

#[component]
pub fn ZipLine() -> Element {
    rsx! {
        div {
            style: r#"
                width: 1000px;
                height: 5px;
                background: {color::SILVER};
                position: relative;
            "#,
            div {
                style: r#"
                    display: flex;
                    flex-direction: center;
                    align-items: center;
                    width: 50px;
                    aspect-ratio: 1 / 1;
                    background: {color::SPRING};
                    position: absolute;
                "#
            }
        }
    }
}