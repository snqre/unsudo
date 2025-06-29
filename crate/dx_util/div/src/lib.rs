use ::prelude::*;
use ::dioxus::prelude::*;

type MaybeListener<T> = Option<EventHandler<Event<T>>>;
type MaybeOpcode = Option<String>;

bundle!(
    attrs_props
    event_props
);