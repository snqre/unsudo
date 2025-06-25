use super::*;
use crate::web::component::*;

leak!(
    hero_case
    hero_heading
    social_icon_group
    social_icon
    svg_shape_0
);

#[component]
pub fn Route() -> Element {
    rsx! {
        layout::VerticalPage {
            layout::VerticalPageSection {
                
            }
        }
    }
}