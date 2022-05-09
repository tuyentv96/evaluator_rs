use std::collections::HashMap;
use thiserror::Error;

use crate::ast::{
    expr::Expr,
    op::{AdditiveOp, EqualityOp, LogicalOp, MultiplicativeOp, Op, RelationalOp},
    value::Value,
};

#[derive(Error, Debug, PartialEq)]
pub enum EvaluatorError {
    #[error("invalid parameter {0}")]
    InvalidParameter(String),
    #[error("invalid operation {0:?} {1:?} {2:?}")]
    InvalidOperation(Value, Op, Value),
}

pub fn evaluate(expr: &Expr, parameters: &HashMap<String, Value>) -> Result<Value, EvaluatorError> {
    match expr {
        Expr::Identifier(name) => match parameters.get(name) {
            Some(v) => Ok(v.clone()),
            None => Err(EvaluatorError::InvalidParameter(name.to_string())),
        },
        Expr::Op(ref lhs, op, ref rhs) => evaluate_op(lhs, op, rhs, parameters),
        Expr::Value(v) => Ok(v.clone()),
    }
}

fn evaluate_op(
    lhs: &Expr,
    op: &Op,
    rhs: &Expr,
    parameters: &HashMap<String, Value>,
) -> Result<Value, EvaluatorError> {
    let lr = evaluate(lhs, parameters)?;
    let rr = evaluate(rhs, parameters)?;

    match op {
        Op::Logical(o) => evaluate_logical_expr(&lr, o, &rr),
        Op::Equality(o) => evaluate_equality_expr(&lr, o, &rr),
        Op::Relational(o) => evaluate_relational_expr(&lr, o, &rr),
        Op::Additive(o) => evaluate_additive_expr(&lr, o, &rr),
        Op::Multiplicative(o) => evaluate_multiplicative_expr(&lr, o, &rr),
    }
}

fn evaluate_logical_expr(
    lhs: &Value,
    op: &LogicalOp,
    rhs: &Value,
) -> Result<Value, EvaluatorError> {
    match op {
        LogicalOp::Or => match (lhs, rhs) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l || *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Logical(*op),
                rhs.clone(),
            )),
        },
        LogicalOp::And => match (lhs, rhs) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l && *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Logical(*op),
                rhs.clone(),
            )),
        },
    }
}

fn evaluate_equality_expr(
    lhs: &Value,
    op: &EqualityOp,
    rhs: &Value,
) -> Result<Value, EvaluatorError> {
    match op {
        EqualityOp::Eq => match (lhs, rhs) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l == *r)),
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(*l == *r)),
            (Value::String(l), Value::String(r)) => Ok(Value::Bool(*l == *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Equality(*op),
                rhs.clone(),
            )),
        },
        EqualityOp::Neq => match (lhs, rhs) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l != *r)),
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(*l != *r)),
            (Value::String(l), Value::String(r)) => Ok(Value::Bool(*l != *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Equality(*op),
                rhs.clone(),
            )),
        },
        EqualityOp::In => match (lhs, rhs) {
            (Value::Number(_), Value::Array(r)) => Ok(Value::Bool(r.contains(lhs))),
            (Value::String(_), Value::Array(r)) => Ok(Value::Bool(r.contains(lhs))),
            (Value::Bool(_), Value::Array(r)) => Ok(Value::Bool(r.contains(lhs))),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Equality(*op),
                rhs.clone(),
            )),
        },
    }
}

fn evaluate_relational_expr(
    lhs: &Value,
    op: &RelationalOp,
    rhs: &Value,
) -> Result<Value, EvaluatorError> {
    match op {
        RelationalOp::Gt => match (lhs, rhs) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l > *r)),
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(*l > *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Relational(*op),
                rhs.clone(),
            )),
        },
        RelationalOp::Gte => match (lhs, rhs) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l >= *r)),
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(*l >= *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Relational(*op),
                rhs.clone(),
            )),
        },
        RelationalOp::Lt => match (lhs, rhs) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l < *r)),
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(*l < *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Relational(*op),
                rhs.clone(),
            )),
        },
        RelationalOp::Lte => match (lhs, rhs) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l <= *r)),
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(*l <= *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Relational(*op),
                rhs.clone(),
            )),
        },
    }
}

fn evaluate_additive_expr(
    lhs: &Value,
    op: &AdditiveOp,
    rhs: &Value,
) -> Result<Value, EvaluatorError> {
    match op {
        AdditiveOp::Add => match (lhs, rhs) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(*l + *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Additive(*op),
                rhs.clone(),
            )),
        },
        AdditiveOp::Sub => match (lhs, rhs) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(*l - *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Additive(*op),
                rhs.clone(),
            )),
        },
    }
}

fn evaluate_multiplicative_expr(
    lhs: &Value,
    op: &MultiplicativeOp,
    rhs: &Value,
) -> Result<Value, EvaluatorError> {
    match op {
        MultiplicativeOp::Mul => match (lhs, rhs) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(*l * *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Multiplicative(*op),
                rhs.clone(),
            )),
        },
        MultiplicativeOp::Div => match (lhs, rhs) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(*l / *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Multiplicative(*op),
                rhs.clone(),
            )),
        },
        MultiplicativeOp::Mod => match (lhs, rhs) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(*l % *r)),
            _ => Err(EvaluatorError::InvalidOperation(
                lhs.clone(),
                Op::Multiplicative(*op),
                rhs.clone(),
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser;
    use crate::Value;
    use std::collections::HashMap;

    #[allow(dead_code)]
    struct TestCase<'a> {
        expr: &'a str,
        want: Result<Value, EvaluatorError>,
    }

    #[allow(dead_code)]
    struct TestCaseWithParameters<'a> {
        expr: &'a str,
        parameters: HashMap<String, Value>,
        want: Result<Value, EvaluatorError>,
    }

    #[test]
    fn test_logical_expr() {
        let empty_parameters: HashMap<String, Value> = HashMap::new();
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expr: "true && true",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "true && false",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "false && false",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "true || false",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "false || false",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "true || true",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "1 || true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Logical(LogicalOp::Or),
                    Value::from(true),
                )),
            },
            TestCase {
                expr: "1 && true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Logical(LogicalOp::And),
                    Value::from(true),
                )),
            },
        ];
        test_cases.iter().for_each(|case| {
            let expr = parser::parse_expr(case.expr).unwrap();
            let output = super::evaluate(&expr, &empty_parameters);
            assert_eq!(case.want, output, "expr: {}", case.expr);
        });
    }

    #[test]
    fn test_additive_expr() {
        let empty_parameters: HashMap<String, Value> = HashMap::new();

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expr: "10 + 10",
                want: Ok(Value::from(20.0)),
            },
            TestCase {
                expr: "10 - 10",
                want: Ok(Value::from(0.0)),
            },
            TestCase {
                expr: "1.5 + 1.5",
                want: Ok(Value::from(3.0)),
            },
            TestCase {
                expr: "1.5 + 1",
                want: Ok(Value::from(2.5)),
            },
            TestCase {
                expr: "1 + 1.5",
                want: Ok(Value::from(2.5)),
            },
            TestCase {
                expr: "3.5 - 1",
                want: Ok(Value::from(2.5)),
            },
            TestCase {
                expr: "3 - 1.5",
                want: Ok(Value::from(1.5)),
            },
            TestCase {
                expr: "3.5 - 1.5",
                want: Ok(Value::from(2.0)),
            },
            TestCase {
                expr: "1 + false",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Additive(AdditiveOp::Add),
                    Value::from(false),
                )),
            },
            TestCase {
                expr: "1 - false",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Additive(AdditiveOp::Sub),
                    Value::from(false),
                )),
            },
        ];
        test_cases.iter().for_each(|case| {
            let expr = parser::parse_expr(case.expr).unwrap();
            let output = super::evaluate(&expr, &empty_parameters);
            assert_eq!(case.want, output, "expr: {}", case.expr);
        });
    }

    #[test]
    fn test_multiplicative_expr() {
        let empty_parameters: HashMap<String, Value> = HashMap::new();

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expr: "10 * 10",
                want: Ok(Value::from(100.0)),
            },
            TestCase {
                expr: "10 / 10",
                want: Ok(Value::from(1.0)),
            },
            TestCase {
                expr: "1.1 * 2.0",
                want: Ok(Value::from(2.2)),
            },
            TestCase {
                expr: "1.1 * 2",
                want: Ok(Value::from(2.2)),
            },
            TestCase {
                expr: "2 * 1.1",
                want: Ok(Value::from(2.2)),
            },
            TestCase {
                expr: "10 % 3",
                want: Ok(Value::from(1)),
            },
            TestCase {
                expr: "10 % 2.5",
                want: Ok(Value::from(0)),
            },
            TestCase {
                expr: "1 * true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Multiplicative(MultiplicativeOp::Mul),
                    Value::from(true),
                )),
            },
            TestCase {
                expr: "1 / true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Multiplicative(MultiplicativeOp::Div),
                    Value::from(true),
                )),
            },
            TestCase {
                expr: "1 % false",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Multiplicative(MultiplicativeOp::Mod),
                    Value::from(false),
                )),
            },
        ];
        test_cases.iter().for_each(|case| {
            let expr = parser::parse_expr(case.expr).unwrap();
            let output = super::evaluate(&expr, &empty_parameters);
            assert_eq!(case.want, output, "expr: {}", case.expr);
        });
    }

    #[test]
    fn test_equality_expr() {
        let empty_parameters: HashMap<String, Value> = HashMap::new();

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expr: "10 == 10",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "10 == 1",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "10 != 10",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "1 != 10",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "10.2 != 12.2",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "10.1 == 10.1",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "10 != 12.2",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "10 == 10.0",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "12.2 != 10",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "10.0 == 10",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "true == true",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "true != true",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "true == false",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "'hello' == 'hello'",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "'hello' == 'world'",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "'hello' != 'world'",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "1 == true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Equality(EqualityOp::Eq),
                    Value::from(true),
                )),
            },
            TestCase {
                expr: "1 != true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Equality(EqualityOp::Neq),
                    Value::from(true),
                )),
            },
            TestCase {
                expr: "1 in true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Equality(EqualityOp::In),
                    Value::from(true),
                )),
            },
        ];
        test_cases.iter().for_each(|case| {
            let expr = parser::parse_expr(case.expr).unwrap();
            let output = super::evaluate(&expr, &empty_parameters);
            assert_eq!(case.want, output, "expr: {}", case.expr);
        });
    }

    #[test]
    fn test_relational_expr() {
        let empty_parameters: HashMap<String, Value> = HashMap::new();

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expr: "2 > 1",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "2 >= 1",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "2 >= 2",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "1 < 2",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "1 <= 1",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "1 <= 2",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "2.0 > 1",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "2.2 >= 1",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "2.0 >= 2",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "1.5 < 2",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "2.2 <= 3",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "2.0 <= 2",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "true >= true",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "true >= false",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "true > true",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "true > false",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "false < false",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "false <= false",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "false < true",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "false <= true",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "1 > true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Relational(RelationalOp::Gt),
                    Value::from(true),
                )),
            },
            TestCase {
                expr: "1 >= true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Relational(RelationalOp::Gte),
                    Value::from(true),
                )),
            },
            TestCase {
                expr: "1 < true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Relational(RelationalOp::Lt),
                    Value::from(true),
                )),
            },
            TestCase {
                expr: "1 <= true",
                want: Err(EvaluatorError::InvalidOperation(
                    Value::from(1),
                    Op::Relational(RelationalOp::Lte),
                    Value::from(true),
                )),
            },
        ];
        test_cases.iter().for_each(|case| {
            let expr = parser::parse_expr(case.expr).unwrap();
            let output = super::evaluate(&expr, &empty_parameters);
            assert_eq!(case.want, output, "expr: {}", case.expr);
        });
    }

    #[test]
    fn test_in_array_expr() {
        let empty_parameters: HashMap<String, Value> = HashMap::new();

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expr: "1 in [1, 2]",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "3 in [1, 2]",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "'one' in ['one', 'two']",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "'three' in ['one', 'two']",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "true in [true, false]",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "true in [false]",
                want: Ok(Value::from(false)),
            },
            TestCase {
                expr: "1 in []",
                want: Ok(Value::from(false)),
            },
        ];
        test_cases.iter().for_each(|case| {
            let expr = parser::parse_expr(case.expr).unwrap();
            let output = super::evaluate(&expr, &empty_parameters);
            assert_eq!(case.want, output, "expr: {}", case.expr);
        });
    }

    #[test]
    fn test_parameter_expr() {
        let test_cases: Vec<TestCaseWithParameters> = vec![
            TestCaseWithParameters {
                expr: "{name} > 1",
                parameters: HashMap::from([("name".to_string(), Value::from(2))]),
                want: Ok(Value::from(true)),
            },
            TestCaseWithParameters {
                expr: "{name} == 1",
                parameters: HashMap::from([("name".to_string(), Value::from(1))]),
                want: Ok(Value::from(true)),
            },
            TestCaseWithParameters {
                expr: "{name} < 1",
                parameters: HashMap::from([("name".to_string(), Value::from(0))]),
                want: Ok(Value::from(true)),
            },
            TestCaseWithParameters {
                expr: "{name} > 1",
                parameters: HashMap::from([("name".to_string(), Value::from(0))]),
                want: Ok(Value::from(false)),
            },
            TestCaseWithParameters {
                expr: "{name} == 1",
                parameters: HashMap::from([("name".to_string(), Value::from(0))]),
                want: Ok(Value::from(false)),
            },
            TestCaseWithParameters {
                expr: "{name} < 1",
                parameters: HashMap::from([("name".to_string(), Value::from(2))]),
                want: Ok(Value::from(false)),
            },
        ];

        test_cases.iter().for_each(|case| {
            let expr = parser::parse_expr(case.expr).unwrap();
            let output = super::evaluate(&expr, &case.parameters);
            assert_eq!(case.want, output, "expr: {}", case.expr);
        });
    }

    #[test]
    fn test_precedence_expr() {
        let empty_parameters: HashMap<String, Value> = HashMap::new();

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                expr: "(2 > 1) && (4 > 2)",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "2 > 1 && 4 > 2",
                want: Ok(Value::from(true)),
            },
            TestCase {
                expr: "1 + 2 * 3",
                want: Ok(Value::from(7)),
            },
            TestCase {
                expr: "(1 + 2) * 3",
                want: Ok(Value::from(9)),
            },
        ];
        test_cases.iter().for_each(|case| {
            let expr = parser::parse_expr(case.expr).unwrap();
            let output = super::evaluate(&expr, &empty_parameters);
            assert_eq!(case.want, output, "expr: {}", case.expr);
        });
    }
}
