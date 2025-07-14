use super::*;

#[component]
pub fn Home() -> Element {
    rsx!(
        dioxus_layout::Page {
            content: rsx!(
                dioxus_layout::PageItem {
                    "Hello World"
                }
            )
        }
    )
}