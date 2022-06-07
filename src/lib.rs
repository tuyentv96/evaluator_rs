//! # evaluator_rs
//!
//! A evaluation engine library for Rust.
//!
//! ## Usage
//!
//! Please see the [Documentation](https://docs.rs/evaluator_rs/) for more details.
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! evaluator_rs = "0.1"
//! ```
//!
//! Examples:
//!
//! ```rust
//! extern crate evaluator_rs;
//!
//! use std::collections::HashMap;
//!
//! use evaluator_rs::{evaluate, parse_expr_from_str, parse_expr_from_json, Value};
//!
//! fn main() {
//!     // from str expression
//!     let expr = parse_expr_from_str("{a} + 2 + 3").unwrap();
//!     let parameters = HashMap::from([("a", Value::from(1))]);
//!     let rs = evaluate(&expr, &parameters).unwrap();
//!     assert_eq!(rs, Value::from(6));
//!
//!     let expr = parse_expr_from_str("{a} in [1, 2 , 3]").unwrap();
//!     let parameters = HashMap::from([("a", Value::from(1))]);
//!     let rs = evaluate(&expr, &parameters).unwrap();
//!     assert_eq!(rs, Value::from(true));
//!
//!     // from json expression
//!     let json_expr = r#"{
//!         "lhs": "{a}",
//!         "op": "in",
//!         "rhs": [4, 5, 6]
//!     }"#;
//!     let expr = parse_expr_from_json(json_expr).unwrap();
//!     let parameters = HashMap::from([("a", Value::from(4))]);
//!     let rs = evaluate(&expr, &parameters).unwrap();
//!     assert_eq!(rs, Value::from(true));
//! }
//! ```

#![allow(unused)]
#[macro_use]
extern crate lalrpop_util;

mod ast;
mod evaluator;
mod parser;

pub use ast::{expr::Expr, value::Value};
pub use evaluator::evaluator::{evaluate, EvaluatorError};
pub use parser::parser::{parse_expr_from_json, parse_expr_from_str, ParserError};
