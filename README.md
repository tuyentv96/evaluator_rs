# evaluator_rs
[![Build](https://github.com/tuyentv96/evaluator_rs/actions/workflows/.test.yml/badge.svg)](https://github.com/tuyentv96/evaluator_rs/actions/workflows/.test.yml) [![codecov](https://codecov.io/gh/tuyentv96/evaluator_rs/branch/master/graph/badge.svg?token=VIyh6tcPDv)](https://codecov.io/gh/tuyentv96/evaluator_rs) [![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A evaluation engine library for Rust.

## Usage

Please see the [Documentation](https://docs.rs/evaluator_rs/) for more details.

Add to your `Cargo.toml`:

```toml
[dependencies]
evaluator_rs = "0.1.0"
```

Examples:

```rust
extern crate evaluator_rs;

use std::collections::HashMap;

use evaluator_rs::{evaluate, parse_expr, Value};

fn main() {
    // from str expression
    let expr = parse_expr("{a} + 2 + 3").unwrap();
    let parameters = HashMap::from([("a", Value::from(1))]);
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(6));

    let expr = parse_expr_from_str("{a} in [1, 2 , 3]").unwrap();
    let parameters = HashMap::from([("a", Value::from(1))]);
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(true));

    // from json expression
    let json_expr = r#"{
        "lhs": "{a}",
        "op": "in",
        "rhs": [4, 5, 6] 
    }"#
    let expr = parse_expr_from_json(json_expr).unwrap();
    let parameters = HashMap::from([("a", Value::from(4))]);
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(true));
}
```

## Data types

| Type | Examples |
|----------|-------------|
| Number | 1 |
| String | 'hello world' |
| Bool | true |
| Array | [1, 2, 3] |

## Supported operators

| Operator | Precedence | Description |
|----------|-------------|-------------|
| && | 1 | And |
| \|\| | 1 | Or |
| == | 2 | Equal |
| > | 2 | Greater than |
| >= | 2 | Greater than or equal |
| < | 2 | Lower than |
| <= | 2 | Lower than or equal |
| in | 2 | Array contains |
| * | 3 | Product |
| / | 3 | Division |
| + | 4 | Sum |
| - | 4 | Sub |

## Identifier

Identifiers are wrapped by curly brace. When expression is evaluated, parameters must be provided identifier value.

Examples:

```rust
    let expr = parse_expr("{a} in [1, 2 , 3]").unwrap();
    let parameters = HashMap::from([("a", Value::from(1))]);
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(true));
```


## License
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
