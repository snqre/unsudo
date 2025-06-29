use super::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        Link {
            style: r#"
                all: unset;
                display: contents;
            "#,
            to: "/",
            layout::Row {
                style: r#"
                    font-family: alien skyline;
                    font-size: {u(3)}px;
                    font-weight: normal;
                    color: {color::SILVER};
                    cursor: pointer;
                "#,
                "unSUDO"
            }
        }
    }
}