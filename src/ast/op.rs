use std::fmt::{Debug, Display, Error, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Copy, Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Op {
    Logical(LogicalOp),
    Relational(RelationalOp),
    Additive(AdditiveOp),
    Multiplicative(MultiplicativeOp),
}

#[derive(Copy, Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Copy, Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum AdditiveOp {
    Add,
    Sub,
}

#[derive(Copy, Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum RelationalOp {
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,
    In,
}

#[derive(Copy, Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum MultiplicativeOp {
    Mul,
    Div,
    Mod,
}

impl Display for Op {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Op::*;
        match *self {
            Logical(o) => write!(fmt, "{}", o),
            Relational(o) => write!(fmt, "{}", o),
            Additive(o) => write!(fmt, "{}", o),
            Multiplicative(o) => write!(fmt, "{}", o),
        }
    }
}

impl Display for LogicalOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::LogicalOp::*;
        match *self {
            And => write!(fmt, "&&"),
            Or => write!(fmt, "||"),
        }
    }
}

impl Display for MultiplicativeOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::MultiplicativeOp::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Mod => write!(fmt, "%"),
        }
    }
}

impl Display for AdditiveOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AdditiveOp::*;
        match *self {
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Display for RelationalOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::RelationalOp::*;
        match *self {
            Gt => write!(fmt, ">"),
            Lt => write!(fmt, "<"),
            Gte => write!(fmt, ">="),
            Lte => write!(fmt, "<="),
            Eq => write!(fmt, "=="),
            Neq => write!(fmt, "!="),
            In => write!(fmt, "in"),
        }
    }
}

impl From<LogicalOp> for Op {
    fn from(op: LogicalOp) -> Self {
        Op::Logical(op)
    }
}

impl From<RelationalOp> for Op {
    fn from(op: RelationalOp) -> Self {
        Op::Relational(op)
    }
}

impl From<AdditiveOp> for Op {
    fn from(op: AdditiveOp) -> Self {
        Op::Additive(op)
    }
}

impl From<MultiplicativeOp> for Op {
    fn from(op: MultiplicativeOp) -> Self {
        Op::Multiplicative(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Op::Logical(LogicalOp::And)), "&&");
        assert_eq!(format!("{}", Op::Logical(LogicalOp::Or)), "||");

        assert_eq!(
            format!("{}", Op::Multiplicative(MultiplicativeOp::Mul)),
            "*"
        );
        assert_eq!(
            format!("{}", Op::Multiplicative(MultiplicativeOp::Div)),
            "/"
        );
        assert_eq!(
            format!("{}", Op::Multiplicative(MultiplicativeOp::Mod)),
            "%"
        );

        assert_eq!(format!("{}", Op::Additive(AdditiveOp::Add)), "+");
        assert_eq!(format!("{}", Op::Additive(AdditiveOp::Sub)), "-");

        assert_eq!(format!("{}", Op::Relational(RelationalOp::Eq)), "==");
        assert_eq!(format!("{}", Op::Relational(RelationalOp::Neq)), "!=");
        assert_eq!(format!("{}", Op::Relational(RelationalOp::In)), "in");

        assert_eq!(format!("{}", Op::Relational(RelationalOp::Lt)), "<");
        assert_eq!(format!("{}", Op::Relational(RelationalOp::Lte)), "<=");
        assert_eq!(format!("{}", Op::Relational(RelationalOp::Gt)), ">");
        assert_eq!(format!("{}", Op::Relational(RelationalOp::Gte)), ">=");
    }

    #[test]
    fn test_convert_op() {
        assert_eq!(Op::Logical(LogicalOp::And), Op::from(LogicalOp::And));
        assert_eq!(Op::Additive(AdditiveOp::Add), Op::from(AdditiveOp::Add));
        assert_eq!(Op::Relational(RelationalOp::Eq), Op::from(RelationalOp::Eq));
        assert_eq!(
            Op::Multiplicative(MultiplicativeOp::Mul),
            Op::from(MultiplicativeOp::Mul)
        );
    }
}
