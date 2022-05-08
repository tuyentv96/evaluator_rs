use thiserror::Error;

use crate::ast::expr::Expr;

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Invalid expr: {0}")]
    InvalidExpr(String),
}

lalrpop_mod!(pub grammar, "/parser/grammar.rs");

pub fn parse_expr(expr_str: &str) -> Result<Box<Expr>, ParserError> {
    match grammar::ExprParser::new().parse(expr_str) {
        Ok(v) => Ok(v),
        Err(e) => Err(ParserError::InvalidExpr(e.to_string())),
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

    #[test]
    fn test_parse_expr() {
        assert_eq!(
            *parse_expr("1 + 2").unwrap(),
            Expr::Op(
                Box::new(Expr::Value(Value::from(1))),
                Op::Additive(AdditiveOp::Add),
                Box::new(Expr::Value(Value::from(2))),
            )
        );

        assert_eq!(
            parse_expr("a + 2").unwrap_err(),
            ParserError::InvalidExpr("Invalid token at 0".to_string()),
        );
    }

    #[test]
    fn test_parse_parameter_name() {
        assert_eq!(parse_parameter_name("{a}"), "a".to_owned(),);
        assert_eq!(parse_parameter_name("{}"), "".to_owned(),);
    }
}
