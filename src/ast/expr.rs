use crate::ast::op::Op;
use crate::ast::value::Value;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Identifier(String),
    Value(Value),
    Op(Box<Expr>, Op, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match &*self {
            Identifier(v) => write!(fmt, "{{{}}}", v),
            Value(v) => write!(fmt, "{}", v),
            Op(ref l, op, ref r) => write!(fmt, "({} {} {})", l, op, r),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::op::AdditiveOp;

    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Expr::Identifier("name".to_owned())), "{name}");
        assert_eq!(format!("{}", Expr::Value(Value::from(1.0))), "1");
        assert_eq!(
            format!(
                "{}",
                Expr::Op(
                    Box::new(Expr::Value(Value::from(1.0))),
                    Op::Additive(AdditiveOp::Add),
                    Box::new(Expr::Value(Value::from(1.0)))
                )
            ),
            "(1 + 1)",
        );
    }
}
