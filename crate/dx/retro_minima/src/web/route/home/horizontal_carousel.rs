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
        layout::Row {
            style: r#"
                width: 100%;
                min-width: 100%;
                max-width: 100%;
                justify-content: start;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            layout::Row {
                style: r#"
                    width: 100%;
                    min-width: 100%;
                    max-width: 100%;
                    align-items: start;
                    gap: {web::sequence(2)}px;
                "#,
                for slot in slots.iter() {
                    { slot }
                }
            }
        }
    }
}