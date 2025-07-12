macro_rules! attrs_props_try_override {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or_else(|| { $self.$key }),
            )*
        }
    };
}