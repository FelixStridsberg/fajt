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

        ast_node! {
            $(#[$enum_attr])*
            $pub $enum $name {
                $( $variant($member), )*
            }
        }

        impl $crate::Spanned for $name {
            fn span(&self) -> &$crate::Span {
                match self {
                    $( Self::$variant(v) => $crate::Spanned::span(v), )*
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
    };
}

/// All AST structures that contains a span attribute should be an ast_struct! to get all common
/// implementations.
macro_rules! ast_struct {
    (
        $(#[$meta:meta])*
        pub $( ($visibility:ident) )? $struct_or_enum:ident $name:ident $($rest:tt)*
    ) => {
        ast_node! {
            $(#[$meta])*
            pub $( ($visibility) )? $struct_or_enum $name $($rest)*
        }

        impl $crate::Spanned for $name {
            fn span(&self) -> &$crate::Span {
                &self.span
            }
        }
    }
}

/// Implements common attributes for all different types in the ast tree.
/// This is called implicitly for ast_struct! and ast_mapping!.
///
/// This is mainly to handle traits that must be applied to the whole tree, for example Debug,
/// Display, PartialEq.
macro_rules! ast_node {
    (
        $(#[$meta:meta])*
        pub $( ($visibility:ident) )? $struct_or_enum:ident $name:ident $($rest:tt)*
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialOrd, PartialEq)]
        #[derive(serde::Serialize, serde::Deserialize)]
        pub $( ($visibility) )? $struct_or_enum $name $($rest)*
    }
}
