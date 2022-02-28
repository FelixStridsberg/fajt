// TODO document
macro_rules! impl_trait {
    (
        impl trait $trait_name:ident for $type:ty {
            $(
                fn $fn_name:ident ($( $params:tt )*) -> $return:ty {
                    $($body:tt)*
                }
            )+
        }
    ) => {
        pub(crate) trait $trait_name {
            $(
                fn $fn_name($( $params )*) -> $return;
            )+
        }

        impl $trait_name for $type {
            $(
                fn $fn_name($( $params )*) -> $return {
                    $( $body )?
                }
            )+
        }
    }
}
