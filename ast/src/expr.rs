use crate::op::Op;
use crate::value::Value;
use std::fmt::{Debug, Error, Formatter};

#[derive(PartialEq, PartialOrd)]
pub enum Expr {
    Identifier(String),
    Value(Value),
    Op(Box<Expr>, Op, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match &*self {
            Identifier(v) => write!(fmt, "{:?}", v),
            Value(v) => write!(fmt, "{:?}", v),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
        }
    }
}
