use super::*;

#[component]
pub fn Home() -> Element {
    rsx!(
        dioxus_layout::Page {
            scroll_snap: dioxus_layout::PageScrollSnap::Proximity,
            style: r#"
                background: grey;
                color: red;
            "#,
            dioxus_layout::PageItem {
                top: rsx!(
                    
                ),

            }
            dioxus_layout::PageItem {
                div {
                    style: r#"
                        width: 200px;
                        aspect-ratio: 1/1;
                        background: red;
                    "#
                }
            }
        }
    )
}