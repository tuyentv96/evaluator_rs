#![allow(unused)]
#[macro_use]
extern crate lalrpop_util;

mod ast;
mod evaluator;
mod parser;

pub use ast::{expr::Expr, value::Value};
pub use evaluator::evaluator::{evaluate, EvaluatorError};
pub use parser::parser::{parse_expr_from_json, parse_expr_from_str, ParserError};
