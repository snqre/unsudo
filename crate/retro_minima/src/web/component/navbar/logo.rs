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
                    font-size: {web::sequence(3)}px;
                    font-weight: normal;
                    color: {color::SILVER};
                "#,
                "unSUDO"
            }
        }
    }
}