#[macro_export(local_inner_macros)]
macro_rules! take_or_keep {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or($self.$key),
            )*
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! take_or_else {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or_else(|| $self.$key),
            )*
        }
    };
}

pub trait Overridable {

    /// # Example
    /// ```rs
    /// use ::dioxus::prelude::*;
    /// use ::unsudo_dioxus::extendable;
    /// 
    /// #[derive(Props)]
    /// #[derive(Clone)]
    /// #[derive(PartialEq)]
    /// pub struct FooProps {
    ///     pub attrs: Option<extendable::AttrsProps>,
    ///     pub event: Option<extendable::EventProps>,
    ///     pub children: Option<Element>
    /// }
    /// 
    /// #[component]
    /// pub fn Foo(props: FooProps) -> Element {
    ///     rsx! {
    ///         extendable::Node {
    ///             attrs: props.attrs.unwrap_or_default().take_or_keep(extendable::AttrsProps {
    ///                 // Will replace `display`.
    ///                 display: "flex".into(),
    ///                 ..Default::default()
    ///             }),
    ///             event: props.event,
    ///             { props.children }
    ///         }
    ///     }
    /// }
    /// ```
    fn take_or_keep(self, edit: Self) -> Self;
    
    /// # Example
    /// ```rs
    /// use ::dioxus::prelude::*;
    /// use ::unsudo_dioxus::extendable;
    /// 
    /// #[derive(Props)]
    /// #[derive(Clone)]
    /// #[derive(PartialEq)]
    /// pub struct FooProps {
    ///     pub attrs: Option<extendable::AttrsProps>,
    ///     pub event: Option<extendable::EventProps>,
    ///     pub children: Option<Element>
    /// }
    /// 
    /// #[component]
    /// pub fn Foo(props: FooProps) -> Element {
    ///     rsx! {
    ///         extendable::Node {
    ///             attrs: props.attrs,
    ///             event: props.event.unwrap_or_default().take_or_else(extendable::EventProps {
    ///                 on_click: |_| {
    ///                     // If `props.event.on_click` is `None` then this listener is passed down.
    ///                     // If `props.event.on_click` is `Some` then `props.event.on_click` listener is passed down.
    ///                 }.into(),
    ///                 ..Default::default()
    ///             }),
    ///             { props.children }
    ///         }
    ///     }
    /// }
    /// ```
    fn take_or_else(self, edit: Self) -> Self;
}