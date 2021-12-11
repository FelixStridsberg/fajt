macro_rules! generate_fold {
    (
        $(
            $ident:ident $( <$param:ident> )? $( (visit: $visit:ident) )? {
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

                    #[allow(unused_mut)]
                    let mut visited = self;
                    $( visited = _visitor.$visit(visited); )?

                    $ident {
                        $($field: visited.$field.fold(_visitor),)*
                        ..visited
                    }
                }
            }
        )*
    };

    (
        $(
            enum $ident:ident {
                $(
                    $field: ident
                )*
            }
        )*
    ) => {
        $(
            impl Fold for $ident {
                fn fold(self, visitor: &mut dyn Visitor) -> Self {
                    match self {
                        $( $ident::$field(v) => $ident::$field(v.fold(visitor)), )*

                        #[allow(unreachable_patterns)]
                        _ => self
                    }
                }
            }
        )*
    }
}