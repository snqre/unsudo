# Extendable Props System for Dioxus
A lightweight Dioxus utility library for extending low-level HTML attributes, and events in higher-level components. Rust lacks a native rest/spread operator (ie. `...props` in JavaScript/React). This crate fills that gap with ergonomic helpers for attribute, and event drilling.
## Get Started
###### Cargo
```toml
dependencies.dioxus.version = ">=0.6.3,<1.0.0"
dependencies.unsudo-dioxus.version = ">=0.1.0,<1.0.0"
```
###### Import
````rust
use ::dioxus::prelude::*;
use ::unsudo_dioxus::ChainEventHandlerExt as _;
use ::unsudo_dioxus::ChainEventHandler as _;
use ::unsudo_dioxus::EditClassExt as _;
use ::unsudo_dioxus::extendable;
````
###### Props
```rust
#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct FooProps {
    pub attrs: Option<extendable::AttrsProps>,
    pub event: Option<extendable::EventProps>,
    pub children: Option<Element>
}
```
###### Component
```rust
#[component]
pub fn Foo(props: FooProps) -> Element {
    rsx! {
        extendable::Node {
            attrs: props.attrs
                // If `props.attrs.id` is `None` `"my-special-id" will be assigned.`
                .edit(extendable::AttrsProps {
                    id: "my-special-id".to_string().into(),
                    ..Default::default()
                })
                // `spell_check` is replaced with `""`.
                .keep(extendable::AttrsProps {
                    spell_check: "".to_string().into(),
                    ..Default::default()
                })
                // Assign styles to the component which can be overriden by the `props.attrs.style`.
                .with_style_before(r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    padding: 8px;
                    font-family: monospace;
                    font-weight: bold;
                    font-size: 5rem;
                    color: #ffffff;
                "#)
                // Assign styles which will override the `props.attrs.style`
                .with_style(r#"
                    flex-direction: row;
                "#),
            event: props.event.on(extendable::EventProps {
                on_click: |_| {
                    
                }.into(),
                ..Default::default()
            }),
            { props.children }
        }
    }
}


#[component]
pub fn Foo(props: FooProps) -> Element {
    rsx!(
        extendable::Node {
            attrs: props.attrs.then_add_style_before(r#"
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                padding: 8px;
                font-family: monospace;
                font-weight: bold;
                font-size: 5rem;
                color: #ffffff;
            "#),
            event: props.event,
            { props.children }
        }
    )
}
```




## API
###### `EventProps`
| Trait | Method Call | Description
|-|-|-|
| `ChainEventHandlerExt` | `Option<EventProps>.on(/.../)` | Allows chaining `.on(/.../)` on `Option<EventProps>`. Initializes with default if `None`.
| `ChainEventHandler` | `EventProps.on(/.../)` | Attaches an event handler. Existing handlers for the same key are preserved.
###### `AttrsProps`



###### Class & Style


| Trait | Call | Description |
|-------|-|-|
| `ChainEventHandlerExt` | `Option<EventProps>.on` | Call `EventProps.on` on an `Option<EventProps>`. It will create a default `EventProps` where each value is `None`, and attach handlers. |
| `ChainEventHandler` | `EventProps.on` | Attach an `EventHandler` to `EventProps`. All previous handlers for that key are preserved. |
| `EditClass` | `AttrsProps.add_class` | Append a css class. All previous class tags for that key are preserved.
| `EditStyle` | `AttrsProps.add_style` | Append a css stylesheet. All previous stylesheets for that key are preserved.
| `EditStyle` | `AttrsProps.add_style_before` | Append a css stylesheet before its own stylesheet. If its own `style` property is `None`, then its stylesheet is empty `""`. All previous stylesheets for that key are preserved.
| `Edit` | `AttrsProps.keep` | Replace the key with a new value |
| `Edit` | `EventProps.keep` | Replace the key with a new value |
| `Edit` | `AttrsProps.edit` | If the source is `None` then add `Some` value, or let the source override it.

## Caveat
###### Styling
When applying CSS styling, avoid mixing built-in style props (like `color`, `display`, etc.) with the general `style` attribute. Doing so can result in unpredictable hierarchy, and styling behaviour.
Built=in style props override the `style` attribute. However, depending on which properties are applied, and where, the final rendered output may differ, potentially causing confusion, or unintended visual results.
Prefer using the `style` attribute for consistency, and ease of maintainance, unless you have a specific reason to use built-in style props.