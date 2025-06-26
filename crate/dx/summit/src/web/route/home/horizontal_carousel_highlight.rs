use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct HorizontalCarouselHighlightProps {
    pub heading: Element,
    pub citation: Element,
    pub source: String
}

#[component]
pub fn HorizontalCarouselHighlight(props: HorizontalCarouselHighlightProps) -> Element {
    rsx! {
        layout::Col {
            style: r#"
                width: 100%;
                min-width: 0%;
                max-width: 100%;
                flex: 1;
                gap: {web::sequence(2)}px;
            "#,
            layout::Col {
                style: r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    align-items: start;
                    gap: {web::sequence(1)}px;
                "#,
                layout::Row {
                    style: r#"
                        width: 100%;
                        min-width: 100%;
                        max-width: 100%;
                        justify-content: start;
                        font-family: br cobane;
                        font-size: {web::sequence(2)}px;
                        font-weight: bold;
                        color: {color::SILVER};
                    "#,
                    { props.heading }
                }
                layout::Row {
                    style: r#"
                        width: 100%;
                        min-width: 100%;
                        max-width: 100%;
                        justify-content: start;
                        font-family: br cobane;
                        font-size: {web::sequence(1)}px;
                        font-weight: normal;
                        color: {color::SILVER};
                    "#,
                    { props.citation }
                }
            }
            layout::Row {
                style: r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    justify-content: start;
                "#,
                intf::LinkedButton {
                    style: r#"
                        font-size: {web::sequence(1)}px;
                    "#,
                    url: props.source,
                    leads_to_external_url: true,
                    "More"
                }
            }
        }
    }
}