use super::*;
use crate::web::component::*;

// mobile-s
// mobile-m
// mobile-l
//
// tablet
//
// laptop
// laptop-l
// laptop-4k

::modwire::expose!(
    hero_heading
    horizontal_carousel_highlight
    horizontal_carousel
    social_icon_group
    social_icon
    stripe
    svg_shape_0
    svg_shape_1
);

#[component]
pub fn Route() -> Element {
    let device: Signal<_> = dioxus_window::use_device();
    let laptop: bool = device() == dioxus_window::Device::Laptop4K || device() == dioxus_window::Device::LaptopL || device() == dioxus_window::Device::Laptop;

    rsx!(
        if laptop {
            layout::Page {
                style: format!(
                    r#"
                        background: {};
                    "#,
                    color::OBSIDIAN
                ),
                layout::PageItem {
                    top: rsx!(
                        match device() {
                            dioxus_window::Device::Laptop4K
                            | dioxus_window::Device::LaptopL
                            | dioxus_window::Device::Laptop
                            | dioxus_window::Device::Tablet => rsx!(
                                layout::Row {
                                    style: format!(
                                        r#"
                                            width: 100%;
                                            height: 5px;
                                            background: linear-gradient(
                                                to right,
                                                {},
                                                {}
                                            );
                                        "#,
                                        color::SILVER,
                                        color::STEEL
                                    )
                                }
                            ),
                            _ => rsx!()
                        }
                    )
                }
            }
        } else {

        }



        layout::Page {
            attrs: None.with_style(&format!(
                r#"
                    background: {}
                "#,
                color::OBSIDIAN
            )),

            layout::PageItem {
                top: rsx! {
                    Stripe {}
                    navigation::Scaffold {
                        left: rsx! {
                            navigation::Logo {}
                            layout::Row {
                                style: r#"
                                    width: clamp(100%, auto, auto);
                                    gap: {web::sequence(2)}px;
                                "#,
                                intf::Button {                                                 style: r#"
                                font-size: {web::sequence(1)}px;
                            "#,"Get Started" }
                                intf::Button {                                                 style: r#"
                                font-size: {web::sequence(1)}px;
                            "#,"Community" }
                                intf::Button {                                                 style: r#"
                                font-size: {web::sequence(1)}px;
                            "#,"Whitepaper" }
                            }
                        }
                    },
                    layout::Col {
                        style: r#"
                            justify-content: space-between;
                            width: 100%;
                            height: 100%;
                            padding-left: {web::sequence(2)}px;
                            padding-right: {web::sequence(2)}px;
                        "#,
                        component::window::Rect {
                            style: r#"
                                width: 100%;
                                min-width: 0%;
                                max-width: {web::sequence(11)}px;
                                min-height: 0%;
                                max-height: 100%;
                                padding: {web::sequence(1)}px;
                            "#,
                            layout::RowFill {
                                style: r#"
                                    justify-content: space-between;
                                "#,
                                layout::Col {
                                    style: r#"
                                        height: 100%;
                                        min-height: 100%;
                                        max-height: 100%;
                                    "#,
                                    layout::Col {
                                        style: r#"
                                            justify-content: space-between;
                                            height: 100%;
                                            min-height: 100%;
                                            max-height: 100%;
                                            gap: {web::sequence(2)}px;
                                        "#,
                                        layout::Row {
                                            style: r#"
                                                width: 100%;
                                                min-width: 100%;
                                                max-width: 100%;
                                                justify-content: start;
                                            "#,
                                            util::To {
                                                to: "/",
                                                decor::Icon {
                                                    w: format!("{}px", u(1)),
                                                    url: Url::Internal(asset!("asset/icon/social/discord.svg"))
                                                }
                                            }
                                            util::To {
                                                to: "/",
                                                decor::Icon {
                                                    w: format!("{}px", u(1)),
                                                    url: Url::Internal(asset!("asset/icon/social/github.svg"))
                                                }
                                            }
                                    
                                            SocialIconGroup {
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/discord.svg") }
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/github.svg") }
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/medium.svg") }
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/telegram.svg") }
                                                SocialIcon { size: "{web::sequence(1)}px", to: "/", url: asset!("asset/icon/social/x.svg") }
                                            }
                                        }
                                        layout::Col {
                                            style: r#"
                                                justify-content: start;
                                                height: 100%;
                                                min-height: 0%;
                                                max-height: 100%;
                                                flex: 1;
                                                gap: {web::sequence(1)}px;
                                            "#,
                                            layout::Row {
                                                style: format! {
                                                    r#"
                                                        width: 100%;
                                                        min-width: 100%;
                                                        max-width: 100%;
                                                        justify-content: start;
                                                        font-family: br cobane;
                                                        font-size: clamp(3rem, 5vw + 1rem, 12rem);
                                                        font-weight: bold;
                                                        color: {};
                                                    "#,
                                                    color::SILVER
                                                },
                                                "Trustless by Design. Ruthless in Reliability."
                                            }
                                            layout::Row {
                                                style: format! {
                                                    r#"
                                                        width: 100%;
                                                        min-width: 100%;
                                                        max-width: 100%;
                                                        justify-content: start;
                                                        font-family: br cobane;
                                                        font-size: {}px;
                                                        font-weight: normal;
                                                        color: {};
                                                    "#,
                                                    web::sequence(2),
                                                    color::SILVER
                                                },
                                                "The silent layer beneath the loudest DAOs—secure, scalable, sovereign."
                                            }
                                        }
                                        layout::Row {
                                            style: format! {
                                                r#"
                                                    width: 100%;
                                                    min-width: 100%;
                                                    max-width: 100%;
                                                    justify-content: start;
                                                    gap: {}px;
                                                "#,
                                                web::sequence(2)
                                            },
                                            intf::SlidingButton {
                                                style: format! {
                                                    r#"
                                                        font-family: br cobane;
                                                        font-size: {};
                                                        font-weight: normal;
                                                        color: {};
                                                    "#,
                                                    web::sequence(2),
                                                    color::SILVER
                                                },
                                                w: 200.0,
                                                h: 50.0,
                                                children_on_idle: rsx! {
                                                    "Create"
                                                },
                                                children_on_hover: rsx! {
                                                    decor::Icon {
                                                        w: "15px",
                                                        url: asset!("asset/icon/chev_r.svg")
                                                    }
                                                }
                                            }
                                            intf::SlidingButton {
                                                style: format! {
                                                    r#"
                                                        font-family: br cobane;
                                                        font-size: {};
                                                        font-weight: normal;
                                                        color: {};
                                                    "#,
                                                    web::sequence(2),
                                                    color::SILVER
                                                },
                                                w: 200.0,
                                                h: 50.0,
                                                children_on_idle: rsx! {
                                                    "Join"
                                                },
                                                children_on_hover: rsx! {
                                                    decor::Icon {
                                                        w: "15px",
                                                        url: asset!("asset/icon/chev_r.svg")
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                layout::Col {
                                    style: r#"
                                        height: 100%;
                                        min-height: 100%;
                                        max-height: 100%;
                                    "#,
                                    layout::Col {
                                        style: r#"
                                            justify-content: space-between;
                                            height: 100%;
                                            min-height: 100%;
                                            max-height: 100%;
                                        "#,
                                        layout::Row {}
                                        SvgShape0 {
                                            w: format!("{}px", match device() {
                                                ::window::Device::Laptop4K | ::window::Device::LaptopL | ::window::Device::Laptop => web::sequence(7),
                                                ::window::Device::Tablet => web::sequence(7),
                                                _ => web::sequence(0)
                                            }),
                                            h: format!("{}px", match device() {
                                                ::window::Device::Laptop4K | ::window::Device::LaptopL | ::window::Device::Laptop => web::sequence(7),
                                                ::window::Device::Tablet => web::sequence(7),
                                                _ => web::sequence(0)
                                            }),
                                            color_0: color::interpolate((color::OBSIDIAN, color::SILVER), 0.5),
                                            color_1: color::interpolate((color::OBSIDIAN, color::STEEL), 0.5)
                                        }
                                        layout::Row {
                                            style: r#"
                                                width: 100%;
                                                min-width: 100%;
                                                max-width: 100%;
                                                justify-content: end;
                                            "#,
                                            intf::LinkedButton {
                                                style: r#"
                                                    font-size: {web::sequence(1)}px;
                                                "#,
                                                url: "/",
                                                leads_to_external_url: true,
                                                "Learn More"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match device() {
                        ::window::Device::Laptop4K | ::window::Device::LaptopL | ::window::Device::Laptop => rsx! {
                            HorizontalCarousel {
                                style: r#"
                                    width: 100%;
                                    min-width: 100%;
                                    max-width: 100%;
                                    height: 100%;
                                    min-height: 0%;
                                    max-height: 100%;
                                    flex: 1;
                                    padding-left: {web::sequence(2)}px;
                                    padding-right: {web::sequence(2)}px;
                                "#,
                                slots: vec![rsx! {
                                    HorizontalCarouselHighlight {
                                        heading: rsx! { "DAOs are the next great form of organisation." },
                                        citation: rsx! { "'Yury Serdich, entrepreneur, Forbes'" },
                                        source: "https://www.forbes.com/sites/davidprosser/2022/08/16/why-punk-master-believes-daos-are-the-future/"
                                    }
                                }, rsx! {
                                    HorizontalCarouselHighlight {
                                        heading: rsx! { "DAOs are the new limited liability companies…In five years, companies won’t have equity anymore. They’ll have tokens and they’ll be represented as DAOs." },
                                        citation: rsx! { "'Cooper Turley, DAO investor via Cointelegraph'" },
                                        source: "https://cointelegraph.com/magazine/dao-challenge-business-model-become-new-corporate-paradigm/"
                                    }
                                }, rsx! {
                                    HorizontalCarouselHighlight {
                                        heading: rsx! { "DAOs are a way of democratizing the management structure for the businesses, projects and communities that employ it." },
                                        citation: rsx! { "Forbes Finance Council" },
                                        source: "https://www.forbes.com/councils/forbesfinancecouncil/2022/10/14/the-state-of-daos-and-what-that-can-mean-for-web3/"
                                    }
                                }]
                            }
                        },
                        _ => rsx! {}
                    }
                }
            }
            layout::PageItem {
                intf::SlidingButton {
                    style: r#"
                        color: white;
                        font-size: 2em;
                    "#,
                    w: 200.0,
                    h: 50.0,
                    children_on_idle: rsx! { "Hello World" },
                    children_on_hover: rsx! { "Hi" }
                }
            }
        }
    )
}


#[component]
fn Laptop() -> Element {
    rsx!(
        layout::Page {
            style: format!(
                r#"
                    background: {};
                "#,
                color::OBSIDIAN
            ),
            layout::PageItem {
                top: rsx!(
                    layout::Row {
                        style: format!(
                            r#"
                                width: 100%;
                                height: 5px;
                                background: linear-gradient(
                                    to right,
                                    {},
                                    {}
                                );
                            "#,
                            color::SILVER,
                            color::STEEL
                        )
                    }
                )
            }
        }
    )
}