extern crate fajt_lexer;
extern crate serde;

#[macro_use]
pub mod error;
pub mod parser;
pub use parser::ContextModify;
pub use parser::Parser;
