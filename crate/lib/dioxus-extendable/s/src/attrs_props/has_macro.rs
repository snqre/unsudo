use super::*;

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