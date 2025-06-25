use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct HorizontalCarouselProps {
    pub slots: Vec<Element>,
    pub style: Option<String>
}

#[component]
pub fn HorizontalCarousel(props: HorizontalCarouselProps) -> Element {
    let slots = props.slots.clone();
    let all_slots = [slots.clone()].concat();  // Duplicate the slots for infinity effect
    rsx! {
        div {
            class: "carousel-box",
            style: r#"
                width: 100%;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            div {
                class: "carousel",
                for slot in all_slots.iter() {
                    div {
                        class: "carousel-item",
                        { slot }
                    }
                }
            }
        }
    }
}