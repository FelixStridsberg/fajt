use crate::{Body, FormalParameters, PropertyName, Span};

ast_struct! {
    pub struct MethodDefinition {
        pub span: Span,
        pub name: PropertyName,
        pub kind: MethodKind,
        pub parameters: FormalParameters,
        pub body: Body,
        pub generator: bool,
        pub asynchronous: bool,
    }
}

ast_node! {
    pub enum MethodKind {
        Method,
        Get,
        Set,
    }
}
