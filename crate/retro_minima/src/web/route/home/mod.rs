use super::*;
use crate::web::component::*;

leak!(
    hero_case
    hero_heading
    social_icon_group
    social_icon
    stripe
    svg_shape_0
    svg_shape_1
);

#[component]
pub fn Route() -> Element {
    rsx! {
        layout::VerticalPage {
            style: r#"
                background: {color::OBSIDIAN};
            "#,
            layout::VerticalPageSection {
                top: rsx! {
                    Stripe {}
                    navbar::Root {
                        left: rsx! {
                            
                        },
                        layout::Row {
                            style: r#"
                                min-width: 100%;
                                gap: 20px;
                            "#,
                            interface::SimpleButton { "Get Started" }
                            interface::SimpleButton { "Community" }
                            interface::SimpleButton { "Whitepaper" }
                        }
                    },
                    window::Rect {
                        layout::Row {
                            layout::Col {

                            }
                            layout::Col {
                                
                            }
                        }
                    }
                }
            }
        }
    }
}