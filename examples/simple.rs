extern crate evaluator_rs;

use std::collections::HashMap;

use evaluator_rs::{evaluate, parse_expr, Value};

fn main() {
    let expr = parse_expr("{a} + 2 + 3").unwrap();
    let parameters = HashMap::from([("a", Value::from(1))]);
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(6));

    let expr = parse_expr("{a} >= 1").unwrap();
    let parameters = HashMap::from([("a", Value::from(1))]);
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(true));

    let expr = parse_expr("{a} in [1, 2 , 3]").unwrap();
    let parameters = HashMap::from([("a", Value::from(1))]);
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(true));
}
