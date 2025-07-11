#[macro_export(local_inner_macros)]
macro_rules! attrs_props_has {
    ($($field_ident:ident)*) => {
        #[derive(Props)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        #[derive(Default)]
        pub struct AttrsProps {
            $(
                #[props(default=None)] pub $field_ident: MaybeOpcode<'static>,
            )*
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! try_override_attrs_props {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or_else(|| { $self.$key }),
            )*
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! force_override_attrs_props {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or($self.$key),
            )*
        }
    };
}

pub type MaybeOpcode<'a> = Option<&'a str>;