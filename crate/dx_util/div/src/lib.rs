use ::prelude::*;
use ::dioxus::prelude::*;

macro_rules! s {
    ($($attr:ident $route:ident)*) => {
        rsx! {
            div {
                $()*
            }
        }
    };
}

pub type MaybeListener<T> = Option<EventHandler<Event<T>>>;
pub type MaybeOpcode = Option<String>;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct DivProps {
    pub attrs: AttrsProps,
    pub event: EventProps,
    pub children: Option<Element>
}

#[component]
pub fn Div(props: DivProps) -> Element {
    s!(

    )
}

#[allow(dead_code)]
fn into_listener<T>(maybe_listener: MaybeListener<T>) -> impl Fn(Event<T>) {
    move |data| {
        if let Some(listener) = maybe_listener {
            listener(data);
        }
    }
}

bundle!(
    attrs_props
    event_props
);