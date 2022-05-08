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

/// Convert String to `Value`
///
/// # Examples
///
/// ```
/// use ast::Value;
///
/// let s: String = "hello".to_string();
/// let x: Value = s.into();
/// ```
impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

/// Convert &str to `Value`
///
/// # Examples
///
/// ```
/// use ast::Value;
///
/// let s: &str = "hello";
/// let x: Value = s.into();
/// ```
impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Value::String(string.to_string())
    }
}

/// Convert 64-bit float to `Value`
///
/// # Examples
///
/// ```
/// use ast::Value;
///
/// let f: f64 = 1.2;
/// let x: Value = f.into();
/// ```
impl From<f64> for Value {
    fn from(float: f64) -> Self {
        Value::Number(float)
    }
}

/// Convert 32-bit float to `Value`
///
/// # Examples
///
/// ```
/// use ast::Value;
///
/// let f: f64 = 1.2;
/// let x: Value = f.into();
/// ```
impl From<f32> for Value {
    fn from(float: f32) -> Self {
        Value::Number(float as f64)
    }
}

/// Convert 64-bit integer to `Value`
///
/// # Examples
///
/// ```
/// use ast::Value;
///
/// let i: i64 = 123;
/// let x: Value = i.into();
/// ```
impl From<i64> for Value {
    fn from(int: i64) -> Self {
        Value::Number(int as f64)
    }
}

/// Convert 32-bit integer to `Value`
///
/// # Examples
///
/// ```
/// use ast::Value;
///
/// let i: i32 = 123;
/// let x: Value = i.into();
/// ```
impl From<i32> for Value {
    fn from(int: i32) -> Self {
        Value::Number(int as f64)
    }
}

/// Convert boolean to `Value`
///
/// # Examples
///
/// ```
/// use ast::Value;
///
/// let b: bool = true;
/// let x: Value = b.into();
/// ```
impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Value::Bool(boolean)
    }
}
