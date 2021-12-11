macro_rules! generate_visit_fold {
    (
        $(
            $ident:ty:  (enter: $enter:ident, exit: $exit:ident) {
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
                    let folded = Self {
                        $($field: node.$field.fold(visitor),)*
                        ..node
                    };

                    visitor.$exit(folded)
                }
            }
        )*

        pub trait StructVisitor {
            $(
                fn $enter(&mut self, node: $ident)-> $ident {
                    node
                }

                fn $exit(&mut self, node: $ident)-> $ident {
                    node
                }
            )*
        }

        impl StructVisitor for TraceVisitor {
            $(
                fn $enter(&mut self, node: $ident) -> $ident {
                    self.visits.push(stringify!($enter));
                    node
                }

                fn $exit(&mut self, node: $ident) -> $ident {
                    self.visits.push(stringify!($exit));
                    node
                }
            )*
        }
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

        impl EnumVisitor for TraceVisitor {
            $(
                fn $enter(&mut self, node: $ident) -> $ident {
                    self.visits.push(stringify!($enter));
                    node
                }

                fn $exit(&mut self, node: $ident) -> $ident {
                    self.visits.push(stringify!($exit));
                    node
                }
            )*
        }

        pub trait EnumVisitor {
            $(
                fn $enter(&mut self, node: $ident)-> $ident {
                    node
                }

                fn $exit(&mut self, node: $ident)-> $ident {
                    node
                }
            )*
        }
    };
}