macro_rules! generate_fold {
    (
        $(
            $ident:ident $( <$param:ident> )? (enter: $enter:ident, exit: $exit:ident) {
                $(
                    $field: ident
                )*
            }
        )*
    ) => {
        $(
            impl Fold for $ident $( <$param> )? {
                fn fold(self, visitor: &mut dyn Visitor) -> Self {
                    let node = visitor.$enter(self);
                    let folded = $ident {
                        $($field: node.$field.fold(visitor),)*
                        ..node
                    };

                    visitor.$exit(folded)
                }
            }
        )*
    };

    (
        $(
            enum $ident:ident (enter: $enter:ident, exit: $exit: ident) {
                $(
                    $field: ident
                )*
            }
        )*
    ) => {
        $(
            impl Fold for $ident {
                fn fold(self, visitor: &mut dyn Visitor) -> Self {
                    let node = visitor.$enter(self);
                    let folded = match node {
                        $( $ident::$field(v) => $ident::$field(v.fold(visitor)), )*

                        #[allow(unreachable_patterns)]
                        _ => node
                    };

                    visitor.$exit(folded)
                }
            }
        )*
    }
}
