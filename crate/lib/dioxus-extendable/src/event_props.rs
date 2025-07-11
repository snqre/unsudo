use super::*;

#[macro_export(local_inner_macros)]
macro_rules! event_props_has {
    ($($field:ident $payload:ty)*) => {
        #[derive(Props)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        #[derive(Default)]
        pub struct EventProps {
            $(
                #[props(default=None)] pub $field: MaybeListener<$payload>,
            )*
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! try_override_event_props {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or_else(|| { $self.$key.to_owned() }),
            )*
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! force_override_event_props {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or($self.$key),
            )*
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! append_event_props {
    ($edit:ident $self:ident $($key:ident $payload_ty:ty)*) => {
        Self {
            $(
                $key: match ($edit.$key, $self.$key) {
                    (Some(a), Some(b)) => {
                        Some(Callback::new(move |data: dioxus::prelude::Event<$payload_ty>| {
                            let a: _ = a.to_owned();
                            let b: _ = b.to_owned();
                            a(data.to_owned());
                            b(data);
                        }))
                    },
                    (Some(a), None) => Some(a.to_owned()),
                    (None, Some(b)) => Some(b.to_owned()),
                    (None, None) => None
                },
            )*
        }
    };
}

pub type MaybeListener<T> = Option<EventHandler<dioxus::prelude::Event<T>>>;

#[allow(dead_code)]
pub(crate) fn into_listener<T>(maybe_listener: MaybeListener<T>) -> impl Fn(dioxus::prelude::Event<T>) {
    move |data| {
        if let Some(listener) = maybe_listener {
            listener(data);
        }
    }
}