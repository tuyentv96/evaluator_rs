#![allow(unused)]
#[macro_use]
extern crate lalrpop_util;

mod ast;
mod evaluator;
mod parser;

pub use ast::value::Value;
pub use evaluator::evaluator::{evaluate, EvaluatorError};
pub use parser::parser::{parse_expr, ParserError};
