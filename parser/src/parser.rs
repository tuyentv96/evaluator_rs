use thiserror::Error;

use crate::ast;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid expr: {0}")]
    InvalidExpr(String),
}

lalrpop_mod!(pub grammar);

pub fn parse_expr(expr_str: &str) -> Result<Box<ast::Expr>, ParserError> {
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
