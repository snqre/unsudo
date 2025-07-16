#[macro_export(local_inner_macros)]
macro_rules! ops {
    ($($fn_name:ident $rhs:ident $rhs_ty:ty => $out:ty)*) => {
        paste::paste!(
            $(
                macro_rules! $fn_name {
                    () => {
                        #[inline]
                        fn $fn_name(self, rhs: $rhs_ty) -> $out {
                            self.$fn_name(rhs)
                        }
                    }
                }
            )*
        );
    };

    ($($fn_name:ident => $out:ty)*) => {
        paste::paste!(
            $(
                macro_rules! $fn_name {
                    () => {
                        #[inline]
                        fn $fn_name(self) -> $out {
                            self.$fn_name()
                        }
                    }
                }
            )*
        );
    };
}