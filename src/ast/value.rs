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
            String(n) => write!(fmt, "{:?}", &n),
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
/// use evaluator_rs::Value;
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
/// use evaluator_rs::Value;
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
/// use evaluator_rs::Value;
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
/// use evaluator_rs::Value;
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
/// use evaluator_rs::Value;
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
/// use evaluator_rs::Value;
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
/// use evaluator_rs::Value;
///
/// let b: bool = true;
/// let x: Value = b.into();
/// ```
impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Value::Bool(boolean)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_value() {
        assert_eq!(Value::Number(1.0), Value::from(1_i32),);
        assert_eq!(Value::Number(1.0), Value::from(1_i64),);
        assert_eq!(Value::Number(1.5), Value::from(1.5_f32),);
        assert_eq!(Value::Number(1.5), Value::from(1.5_f64),);
        assert_eq!(Value::Bool(true), Value::from(true),);
        assert_eq!(
            Value::String("hello world".to_owned()),
            Value::from("hello world"),
        );
        assert_eq!(
            Value::String("hello world".to_owned()),
            Value::from("hello world".to_owned()),
        );
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Value::Number(1.0)), "1.0");
        assert_eq!(format!("{:?}", Value::Bool(true)), "true");
        assert_eq!(
            format!("{:?}", Value::String("hello world".to_owned())),
            "\"hello world\""
        );
        assert_eq!(
            format!(
                "{:?}",
                Value::Array(vec!(Value::from(1), Value::from(2), Value::from(3)))
            ),
            "[1.0, 2.0, 3.0]"
        );
    }
}
