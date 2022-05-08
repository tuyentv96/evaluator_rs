extern crate evaluator_rs;

use std::collections::HashMap;

use evaluator_rs as evaluator;

fn main() {
    let expr = evaluator::parse_expr("1 + 2 + 3").unwrap();
    let parameters = HashMap::new();
    let rs = evaluator::evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, ast::Value::Number(6_f64));
}
