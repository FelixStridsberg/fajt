extern crate fajt_lexer;

#[macro_use]
pub mod ast;
#[macro_use]
pub mod error;
pub mod parser;
pub use parser::ContextModify;
pub use parser::Parser;
