use super::*;

#[component]
pub fn SpikedButton() -> Element {
    rsx! {
        svg {
            style: r#"
                width: 200px;
                height: 50px;
            "#,
            polygon {
                points: r#"
                    25   , 25
                    50  , 0 
                    150 , 0 
                    175 , 25 
                    180 , 50 
                    20  , 50
                "#,
                fill: "{color::SILVER}"
            },
            polygon {
                points: r#"
                    0  , 0
                    30 , 0
                    15  , 20
                "#,
                fill: "none",
                stroke: "{color::SILVER}",
                stroke_width: "1"
            }
        }
    }
}