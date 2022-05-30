use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use thiserror::Error;

use crate::{
    ast::{
        expr::Expr,
        op::{LogicalOp, Op},
    },
    Value,
};

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Invalid expr: {0}")]
    InvalidExpr(String),
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    #[error("Invalid op: {0}")]
    InvalidOp(String),
    #[error("Missing value: {0}")]
    MissingValue(&'static str),
}

lalrpop_mod!(pub grammar, "/parser/grammar.rs");

/// Parse expression from str
///
/// # Examples
///
/// ```
/// use evaluator_rs::*;
/// use std::collections::HashMap;
///
/// let expr = parse_expr_from_str("{a} + 2 + 3").unwrap();
/// let parameters = HashMap::from([("a", Value::from(4))]);
/// let rs = evaluate(&expr, &parameters).unwrap();
/// assert_eq!(rs, Value::from(9));
/// ```
pub fn parse_expr_from_str(expr_str: &str) -> Result<Box<Expr>, ParserError> {
    match grammar::ExprParser::new().parse(expr_str) {
        Ok(v) => Ok(v),
        Err(e) => Err(ParserError::InvalidExpr(e.to_string())),
    }
}

/// Parse expression from json
///
/// # Examples
///
/// ```
/// use evaluator_rs::*;
/// use std::collections::HashMap;
///
/// let json_expr = r#"{
///     "lhs": "{a}",
///     "op": "in",
///     "rhs": [4, 5, 6]
/// }"#;
/// let expr = parse_expr_from_json(json_expr).unwrap();
/// let parameters = HashMap::from([("a", Value::from(4))]);
/// let rs = evaluate(&expr, &parameters).unwrap();
/// assert_eq!(rs, Value::from(true));
/// ```
pub fn parse_expr_from_json(expr_str: &str) -> Result<Box<Expr>, ParserError> {
    match serde_json::from_str::<serde_json::Value>(expr_str) {
        Ok(value) => parse_expr_from_json_value(&value),
        Err(e) => Err(ParserError::InvalidExpr(e.to_string())),
    }
}

fn parse_expr_from_json_value(expr_json: &serde_json::Value) -> Result<Box<Expr>, ParserError> {
    match expr_json {
        JsonValue::Object(v) => {
            let lhs = expr_json
                .get("lhs")
                .ok_or(ParserError::MissingValue("lhs"))?;
            let op = expr_json.get("op").ok_or(ParserError::MissingValue("op"))?;
            let rhs = expr_json
                .get("rhs")
                .ok_or(ParserError::MissingValue("rhs"))?;

            let op = op
                .as_str()
                .ok_or_else(|| ParserError::InvalidOp(op.to_string()))?;

            let op = match grammar::OpParser::new().parse(op) {
                Ok(v) => v,
                Err(e) => return Err(ParserError::InvalidOp(op.to_owned())),
            };

            let lhs = parse_expr_from_json_value(lhs)?;
            let rhs = parse_expr_from_json_value(rhs)?;

            Ok(Box::new(Expr::Op(lhs, op, rhs)))
        }
        JsonValue::String(v) => match grammar::IdentifierParser::new().parse(v) {
            Ok(var) => Ok(Box::new(Expr::Identifier(parse_parameter_name(&var)))),
            Err(_) => Ok(Box::new(Expr::Value(Value::from(v.as_str())))),
        },
        JsonValue::Null => Err(ParserError::InvalidValue("null".to_owned())),
        v => Ok(Box::new(Expr::Value(Value::from(v)))),
    }
}
pub fn parse_parameter_name(value: &str) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::op::*;
    use crate::Expr;
    use crate::Value;

    #[allow(dead_code)]
    struct TestCase<'a> {
        expr: &'a str,
        want: Result<Box<Expr>, ParserError>,
    }

    #[test]
    fn test_json_expr() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expr: r#"
                {
                    "lhs": true,
                    "op": "&&",
                    "rhs": false 
                }"#,
                want: Ok(Box::new(Expr::Op(
                    Box::new(Expr::Value(Value::from(true))),
                    Op::Logical(LogicalOp::And),
                    Box::new(Expr::Value(Value::from(false))),
                ))),
            },
            TestCase {
                expr: r#"
                {
                    "lhs": 1,
                    "op": "+",
                    "rhs": 2 
                }"#,
                want: Ok(Box::new(Expr::Op(
                    Box::new(Expr::Value(Value::from(1))),
                    Op::Additive(AdditiveOp::Add),
                    Box::new(Expr::Value(Value::from(2))),
                ))),
            },
            TestCase {
                expr: r#"
                {
                    "lhs": 3,
                    "op": "==",
                    "rhs": 4 
                }"#,
                want: Ok(Box::new(Expr::Op(
                    Box::new(Expr::Value(Value::from(3))),
                    Op::Relational(RelationalOp::Eq),
                    Box::new(Expr::Value(Value::from(4))),
                ))),
            },
            TestCase {
                expr: r#"
                {
                    "lhs": "hello",
                    "op": "==",
                    "rhs": "hello" 
                }"#,
                want: Ok(Box::new(Expr::Op(
                    Box::new(Expr::Value(Value::from("hello"))),
                    Op::Relational(RelationalOp::Eq),
                    Box::new(Expr::Value(Value::from("hello"))),
                ))),
            },
            TestCase {
                expr: r#"
                {
                    "lhs": 4,
                    "op": "in",
                    "rhs": [4, 5, 6] 
                }"#,
                want: Ok(Box::new(Expr::Op(
                    Box::new(Expr::Value(Value::from(4))),
                    Op::Relational(RelationalOp::In),
                    Box::new(Expr::Value(Value::Array(vec![
                        Value::from(4),
                        Value::from(5),
                        Value::from(6),
                    ]))),
                ))),
            },
            TestCase {
                expr: r#"
                {
                    "lhs": {
                        "lhs": 4,
                        "op": "-",
                        "rhs": {
                            "lhs": 2,
                            "op": "-",
                            "rhs": "{foo}"
                        }
                    },
                    "op": "==",
                    "rhs": {
                        "lhs": 1,
                        "op": "+",
                        "rhs": "{bar}"
                    }
                }"#,
                want: Ok(Box::new(Expr::Op(
                    Box::new(Expr::Op(
                        Box::new(Expr::Value(Value::from(4))),
                        Op::Additive(AdditiveOp::Sub),
                        Box::new(Expr::Op(
                            Box::new(Expr::Value(Value::from(2))),
                            Op::Additive(AdditiveOp::Sub),
                            Box::new(Expr::Identifier("foo".to_owned())),
                        )),
                    )),
                    Op::Relational(RelationalOp::Eq),
                    Box::new(Expr::Op(
                        Box::new(Expr::Value(Value::from(1))),
                        Op::Additive(AdditiveOp::Add),
                        Box::new(Expr::Identifier("bar".to_owned())),
                    )),
                ))),
            },
            TestCase {
                expr: r#"
                {
                    "lhs": 3,
                }"#,
                want: Err(ParserError::InvalidExpr(
                    "trailing comma at line 4 column 17".to_owned(),
                )),
            },
            TestCase {
                expr: r#"
                {
                    "lhs": 3,
                    "op": "add",
                    "rhs": 4 
                }"#,
                want: Err(ParserError::InvalidOp("add".to_owned())),
            },
        ];
        test_cases.iter().for_each(|case| {
            let expr = parse_expr_from_json(case.expr);
            assert_eq!(case.want, expr);
        });
    }

    #[test]
    fn test_parse_expr() {
        assert_eq!(
            *parse_expr_from_str("1 + 2").unwrap(),
            Expr::Op(
                Box::new(Expr::Value(Value::from(1))),
                Op::Additive(AdditiveOp::Add),
                Box::new(Expr::Value(Value::from(2))),
            )
        );

        assert_eq!(
            parse_expr_from_str("a + 2").unwrap_err(),
            ParserError::InvalidExpr("Invalid token at 0".to_string()),
        );
    }

    #[test]
    fn test_parse_parameter_name() {
        assert_eq!(parse_parameter_name("{a}"), "a".to_owned(),);
        assert_eq!(parse_parameter_name("{}"), "".to_owned(),);
    }
}
