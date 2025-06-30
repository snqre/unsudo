use super::*;

pub type MaybeListener<T> = Option<EventHandler<Event<T>>>;