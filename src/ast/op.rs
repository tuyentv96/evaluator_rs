use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Copy, Debug, Clone, PartialEq, PartialOrd)]
pub enum Op {
    Logical(LogicalOp),
    Equality(EqualityOp),
    Relational(RelationalOp),
    Additive(AdditiveOp),
    Multiplicative(MultiplicativeOp),
}

#[derive(Copy, Debug, Clone, PartialEq, PartialOrd)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Copy, Debug, Clone, PartialEq, PartialOrd)]
pub enum AdditiveOp {
    Add,
    Sub,
}

#[derive(Copy, Debug, Clone, PartialEq, PartialOrd)]
pub enum EqualityOp {
    Eq,
    Neq,
    In,
}

#[derive(Copy, Debug, Clone, PartialEq, PartialOrd)]
pub enum RelationalOp {
    Gt,
    Lt,
    Gte,
    Lte,
}

#[derive(Copy, Debug, Clone, PartialEq, PartialOrd)]
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
            Equality(o) => write!(fmt, "{}", o),
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

impl Display for EqualityOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::EqualityOp::*;
        match *self {
            Eq => write!(fmt, "=="),
            Neq => write!(fmt, "!="),
            In => write!(fmt, "in"),
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
        }
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

        assert_eq!(format!("{}", Op::Equality(EqualityOp::Eq)), "==");
        assert_eq!(format!("{}", Op::Equality(EqualityOp::Neq)), "!=");
        assert_eq!(format!("{}", Op::Equality(EqualityOp::In)), "in");

        assert_eq!(format!("{}", Op::Relational(RelationalOp::Lt)), "<");
        assert_eq!(format!("{}", Op::Relational(RelationalOp::Lte)), "<=");
        assert_eq!(format!("{}", Op::Relational(RelationalOp::Gt)), ">");
        assert_eq!(format!("{}", Op::Relational(RelationalOp::Gte)), ">=");
    }
}
