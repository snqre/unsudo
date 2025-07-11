# Extendable Props System for Dioxus
A lightweight Dioxus utility library for extending low-level HTML attributes, and events in higher-level components. Rust lacks a native rest/spread operator (ie. `...props` in JavaScript/React). This crate fills that gap with ergonomic helpers for attribute, and event drilling.
## Get Started
###### Cargo
```toml
dependencies.dioxus.version = ">=0.6.3,<1.0.0"
dependencies.unsudo-dioxus.version = ">=0.1.0,<1.0.0"
```
###### Component
```rust
use ::dioxus::prelude::*;
use ::unsudo_dioxus::extendable;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct FooProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}

#[component]
pub fn Foo(props: FooProps) -> Element {
    rsx! {
        extendable::Node {
            attrs: props.unwrap_or_default().try_override(extendable::AttrsProps {
                display: "flex".into(),
                flex_direction: "column".into(),
                justify_content: "center".into(),
                align_items: "center".into(),
                padding: "8px".into(),
                font_family: "monospace".into(),
                font_weight: "bold".into(),
                font_size: "5rem".into(),
                color: "#ffffff".into(),
                ..Default::default()
            }),
            event: props.event,
            { props.children }
        }
    }
}
```