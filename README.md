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
    let expr = parse_expr("{a} + 2 + 3").unwrap();
    let parameters = HashMap::from([("a", Value::from(1))]);
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(6));

    let expr = parse_expr("{a} == 1").unwrap();
    let parameters = HashMap::from([("a", Value::from(1))]);
    let rs = evaluate(&expr, &parameters).unwrap();
    assert_eq!(rs, Value::from(true));

    let expr = parse_expr("{a} in [1, 2 , 3]").unwrap();
    let parameters = HashMap::from([("a", Value::from(1))]);
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

| Operator | Description |
|----------|-------------|
| && | And |
| \|\| | Or |
| == | Equal |
| > | Greater than |
| >= | Greater than or equal |
| < | Lower than |
| <= | Lower than or equal |
| + | Sum |
| - | Sub |
| * | Product |
| / | Division |
| in | Array contains |

## Identifier

Identfiers are wrapped by curly brace. When expression is evaluating, parameters must provide this identifier value.

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
