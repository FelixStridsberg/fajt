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
        // Traverse implementation for the enums
        $(
            impl Traverse for $enum {
                fn traverse(&mut self, visitor: &mut dyn Visitor) {
                    let traverse_children = visitor.$enum_enter(self);

                    if traverse_children {
                        match self {
                            $( Self::$variant(v) => v.traverse(visitor), )*

                            #[allow(unreachable_patterns)]
                            _ => {}
                        };
                    }

                    visitor.$enum_exit(self);
                }
            }
        )*

        // Traverse implementation for the structs
        $(
            impl Traverse for $struct {
                fn traverse(&mut self, visitor: &mut dyn Visitor) {
                    let traverse_children = visitor.$struct_enter(self);

                    if traverse_children {
                        $( self.$field.traverse(visitor); )*
                    }

                    visitor.$struct_exit(self)
                }
            }
        )*

        // Visitor trait with all methods defined for any struct or enum
        pub trait Visitor {
            fn enter(&mut self) {}
            fn exit(&mut self) {}

            $(
                fn $enum_enter(&mut self, _node: &mut $enum) -> bool { true }

                fn $enum_exit(&mut self, _node: &mut $enum) { }
            )*

            $(
                fn $struct_enter(&mut self, _node: &mut $struct) -> bool { true }

                fn $struct_exit(&mut self, _node: &mut $struct) { }
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
                fn $enum_enter(&mut self, _node: &mut $enum) -> bool {
                    self.visits.push(stringify!($enum_enter));
                    true
                }

                fn $enum_exit(&mut self, _node: &mut $enum) {
                    self.visits.push(stringify!($enum_exit));
                }
            )*

            $(
                fn $struct_enter(&mut self, _node: &mut $struct) -> bool {
                    self.visits.push(stringify!($struct_enter));
                    true
                }

                fn $struct_exit(&mut self, _node: &mut $struct) {
                    self.visits.push(stringify!($struct_exit));
                }
            )*
        }
    }
}
