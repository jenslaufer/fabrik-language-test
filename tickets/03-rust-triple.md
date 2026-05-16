# Add `triple_value` to the Rust stack

## Implementation
1. In `rust/src/lib.rs`, add `pub fn triple_value(n: i64) -> i64`
   returning `n * 3`.
2. In the same file's `tests` module, add `#[test] fn
   triple_value_triples_its_input()` covering 0, 4, and -3.

## Acceptance
- [ ] `triple_value(0) == 0`, `triple_value(4) == 12`, `triple_value(-3) == -9`.
- [ ] `cargo test` reports both `double_value_doubles_its_input` and
      `triple_value_triples_its_input` passing.

Run `cargo test` from `rust/`.
