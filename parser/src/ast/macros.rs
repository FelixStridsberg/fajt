/// The AST contains enums that maps to structs, these must be wrapped in this macro to add common
/// implementation that can be used to any node in the tree.
macro_rules! ast_mapping {
    (
        $(#[$enum_attr:meta])*
        $pub:ident $enum:ident $name:ident {
            $(
                $variant:ident($member:ident),
            )*
        }
    ) => {
        ast_struct! {
            $(#[$enum_attr])*
            $pub $enum $name {
                $( $variant($member), )*
            }
        }

        impl $name {
            pub fn span(&self) -> &Span {
                match self {
                    $(
                        Self::$variant(v) => &v.span,
                    )*
                }
            }
        }

        $( ast_mapping_impl!($name, $variant, $member); )*
    };
}

macro_rules! ast_mapping_impl {
    ($enum_name:ident, $variant:ident, $member:ident) => {
        impl From<$member> for $enum_name {
            fn from(f: $member) -> Self {
                Self::$variant(f)
            }
        }

        impl From<$member> for Box<$enum_name> {
            fn from(f: $member) -> Self {
                Box::new($enum_name::$variant(f))
            }
        }
    };
}

/// Implements common attributes for structures that are part of the AST.
/// This is mainly to handle traits that must be applied to the whole tree, for example Debug,
/// Display, PartialEq.
macro_rules! ast_struct {
    (
        $(#[$meta:meta])*
        pub $( ($visibility:ident) )? $struct_or_enum:ident $name:ident $($rest:tt)*
    ) => {
        $(#[$meta])*
        #[derive(Debug, PartialOrd, PartialEq)]
        #[derive(serde::Serialize, serde::Deserialize)]
        pub $( ($visibility) )? $struct_or_enum $name $($rest)*
    }
}
