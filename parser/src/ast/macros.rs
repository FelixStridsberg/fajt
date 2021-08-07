/// Adds common implementations for enum that builds up the ast.
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

        $( ast_enum_struct_impl!($name, $variant, $member); )*
    };
}

macro_rules! ast_enum_struct_impl {
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
