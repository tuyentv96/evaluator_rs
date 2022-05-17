use serde_json::Value as JsonValue;
use std::fmt::{Display, Error, Formatter};

/// Value used by by the parser and evaluator.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Value::*;
        match &*self {
            Bool(n) => write!(fmt, "{}", n),
            String(n) => write!(fmt, "{}", &n),
            Number(n) => write!(fmt, "{}", n),
            Array(array) => {
                write!(fmt, "[")?;
                let mut once = false;
                for e in array {
                    if once {
                        write!(fmt, ", ")?;
                    } else {
                        once = true;
                    }
                    e.fmt(fmt)?;
                }
                write!(fmt, "]")
            }
        }
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

impl From<&JsonValue> for Value {
    fn from(jv: &JsonValue) -> Self {
        match jv {
            JsonValue::String(v) => Value::String(v.clone()),
            JsonValue::Number(v) => Value::Number(v.clone().as_f64().unwrap()),
            JsonValue::Bool(v) => Value::Bool(*v),
            JsonValue::Array(array) => {
                let mut rs = vec![];
                for v in array {
                    rs.push(Value::from(v))
                }

                Value::Array(rs)
            }
            v => panic!("unsupport type {}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_value() {
        assert_eq!(Value::Number(1.0), Value::from(1_i32));
        assert_eq!(Value::Number(1.0), Value::from(1_i64));
        assert_eq!(Value::Number(1.5), Value::from(1.5_f32));
        assert_eq!(Value::Number(1.5), Value::from(1.5_f64));
        assert_eq!(Value::Bool(true), Value::from(true));

        assert_eq!(
            Value::String("hello world".to_owned()),
            Value::from("hello world"),
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Value::Number(1_f64)), "1");
        assert_eq!(format!("{}", Value::Number(1.1)), "1.1");
        assert_eq!(format!("{}", Value::Bool(true)), "true");
        assert_eq!(
            format!("{}", Value::String("hello world".to_owned())),
            "hello world"
        );
        assert_eq!(
            format!(
                "{}",
                Value::Array(vec!(Value::from(1), Value::from(2), Value::from(3)))
            ),
            "[1, 2, 3]"
        );
    }
}
