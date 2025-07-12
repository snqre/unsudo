macro_rules! force_override {
    ($edit:ident $self:ident $($key:ident)*) => {
        Self {
            $(
                $key: $edit.$key.or($self.$key),
            )*
        }
    };
}