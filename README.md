# evaluator_rs
[![evaluator_rs](https://github.com/tuyentv96/evaluator_rs/actions/workflows/.test.yml/badge.svg)](https://github.com/tuyentv96/evaluator_rs/actions/workflows/.test.yml) [![codecov](https://codecov.io/gh/tuyentv96/evaluator_rs/branch/master/graph/badge.svg?token=VIyh6tcPDv)](https://codecov.io/gh/tuyentv96/evaluator_rs)

A evaluation engine library for Rust.

## Usage

Please see the [Documentation](https://docs.rs/evaluator_rs/) for more details.

Add to your `Cargo.toml`:

```toml
[dependencies]
evaluator_rs = "0.1.0"
```

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

## License
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
