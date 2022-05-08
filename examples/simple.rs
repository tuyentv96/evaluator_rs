extern crate evaluator_rs;

use std::collections::HashMap;

use evaluator_rs::{evaluate, parse_expr, Value};

fn main() {
    let expr = parse_expr("1 + 2 + 3").unwrap();
    let parameters = HashMap::new();
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(6));
}
