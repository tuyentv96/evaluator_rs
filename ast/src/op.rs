use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Op {
    Logical(LogicalOp),
    Equality(EqualityOp),
    Relational(RelationalOp),
    Additive(AdditiveOp),
    Multiplicative(MultiplicativeOp),
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum AdditiveOp {
    Add,
    Sub,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum EqualityOp {
    Eq,
    Neq,
    In,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum RelationalOp {
    Gt,
    Lt,
    Gte,
    Lte,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum MultiplicativeOp {
    Mul,
    Div,
    Mod,
}

impl Debug for Op {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Op::*;
        match *self {
            Logical(o) => write!(fmt, "{:?}", o),
            Equality(o) => write!(fmt, "{:?}", o),
            Relational(o) => write!(fmt, "{:?}", o),
            Additive(o) => write!(fmt, "{:?}", o),
            Multiplicative(o) => write!(fmt, "{:?}", o),
        }
    }
}

impl Debug for LogicalOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::LogicalOp::*;
        match *self {
            And => write!(fmt, "&&"),
            Or => write!(fmt, "||"),
        }
    }
}

impl Debug for MultiplicativeOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::MultiplicativeOp::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Mod => write!(fmt, "%"),
        }
    }
}

impl Debug for AdditiveOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AdditiveOp::*;
        match *self {
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Debug for EqualityOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::EqualityOp::*;
        match *self {
            Eq => write!(fmt, "=="),
            Neq => write!(fmt, "!="),
            In => write!(fmt, "in"),
        }
    }
}

impl Debug for RelationalOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::RelationalOp::*;
        match *self {
            Gt => write!(fmt, ">"),
            Lt => write!(fmt, "<"),
            Gte => write!(fmt, ">="),
            Lte => write!(fmt, "<="),
        }
    }
}
