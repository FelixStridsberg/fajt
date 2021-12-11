macro_rules! generate_fold {
    (
        $(
            $ident:ident $( <$param:ident> )? (enter: $enter:ident) {
                $(
                    $field: ident
                )*
            }
        )*
    ) => {
        $(
            impl Fold for $ident $( <$param> )? {
                fn fold(self, _visitor: &mut dyn Visitor) -> Self {
                    println!("Traverse: {}", stringify!($ident));

                    let node = _visitor.$enter(self);

                    $ident {
                        $($field: node.$field.fold(_visitor),)*
                        ..node
                    }
                }
            }
        )*
    };

    (
        $(
            enum $ident:ident (enter: $enter:ident) {
                $(
                    $field: ident
                )*
            }
        )*
    ) => {
        $(
            impl Fold for $ident {
                fn fold(self, visitor: &mut dyn Visitor) -> Self {
                    println!("Traverse: {}", stringify!($ident));
                    let node = visitor.$enter(self);
                    match node {
                        $( $ident::$field(v) => $ident::$field(v.fold(visitor)), )*

                        #[allow(unreachable_patterns)]
                        _ => node
                    }
                }
            }
        )*
    }
}
