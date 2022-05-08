use std::fmt::{Debug, Error, Formatter};

/// Value used by by the parser and evaluator.
#[derive(Clone, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
}

impl Debug for Value {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Value::*;
        match &*self {
            Bool(n) => write!(fmt, "{:?}", n),
            String(n) => write!(fmt, "{:?}", n.clone()),
            Number(n) => write!(fmt, "{:?}", n),
            Array(n) => write!(fmt, "{:?}", n),
        }
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Value::String(string.to_string())
    }
}

impl From<f64> for Value {
    fn from(float: f64) -> Self {
        Value::Number(float)
    }
}

impl From<f32> for Value {
    fn from(float: f32) -> Self {
        Value::Number(float as f64)
    }
}

impl From<i64> for Value {
    fn from(int: i64) -> Self {
        Value::Number(int as f64)
    }
}

impl From<i32> for Value {
    fn from(int: i32) -> Self {
        Value::Number(int as f64)
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Value::Bool(boolean)
    }
}
