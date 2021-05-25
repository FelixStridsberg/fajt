extern crate fajt_lexer;

pub mod ast;
#[macro_use]
pub mod error;
pub mod parser;
pub use parser::Parser;
