macro_rules! generate_fold_and_visit {
    (
        enums: {
            $(
                $enum:ident: (enter: $enum_enter:ident, exit: $enum_exit: ident) {
                    $(
                        $variant:ident
                    )*
                }
            )*
        }

        structs: {
            $(
                $struct:ty:  (enter: $struct_enter:ident, exit: $struct_exit:ident) {
                    $(
                        $field: ident
                    )*
                }
            )*
        }
    ) => {
        // Fold implementation for the enums
        $(
            impl Fold for $enum {
                fn fold(self, visitor: &mut dyn Visitor) -> Self {
                    let node = visitor.$enum_enter(self);
                    let folded = match node {
                        $( Self::$variant(v) => Self::$variant(v.fold(visitor)), )*

                        #[allow(unreachable_patterns)]
                        _ => node
                    };

                    visitor.$enum_exit(folded)
                }
            }
        )*

        // Fold implementation for the structs
        $(
            impl Fold for $struct {
                fn fold(self, visitor: &mut dyn Visitor) -> Self {
                    let node = visitor.$struct_enter(self);
                    let folded = Self {
                        $($field: node.$field.fold(visitor),)*
                        ..node
                    };

                    visitor.$struct_exit(folded)
                }
            }
        )*

        // Visitor trait with all methods defined for any struct or enum
        pub trait Visitor {
            fn enter(&mut self) {}
            fn exit(&mut self) {}

            $(
                fn $enum_enter(&mut self, node: $enum)-> $enum {
                    node
                }

                fn $enum_exit(&mut self, node: $enum)-> $enum {
                    node
                }
            )*

            $(
                fn $struct_enter(&mut self, node: $struct)-> $struct {
                    node
                }

                fn $struct_exit(&mut self, node: $struct)-> $struct {
                    node
                }
            )*
        }

        // Trace visitor for easy debugging
        pub struct TraceVisitor {
            pub visits: Vec<&'static str>,
        }

        impl TraceVisitor {
            pub fn new() -> Self {
                Self {
                    visits: vec![],
                }
            }
        }

        impl Visitor for TraceVisitor {
            $(
                fn $enum_enter(&mut self, node: $enum)-> $enum {
                    self.visits.push(stringify!($enum_enter));
                    node
                }

                fn $enum_exit(&mut self, node: $enum)-> $enum {
                    self.visits.push(stringify!($enum_exit));
                    node
                }
            )*

            $(
                fn $struct_enter(&mut self, node: $struct)-> $struct {
                    self.visits.push(stringify!($struct_enter));
                    node
                }

                fn $struct_exit(&mut self, node: $struct)-> $struct {
                    self.visits.push(stringify!($struct_exit));
                    node
                }
            )*
        }
    }
}
